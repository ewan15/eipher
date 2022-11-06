mod client;
mod types;
mod _io_uring;
mod net;
mod http_server;
mod config;
mod cli;

use io_uring::IoUring;


use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::error::Error;












use std::rc::Rc;

use crate::client::{Client, RcUnsafeClient};
use crate::_io_uring::{CompletionQueueMessage, client_accept, client_read, client_send, completion_queue, client_close};
use crate::net::{setup_connection, create_sock_addr};

use crate::http_server::HttpServer;
use crate::cli::Cli;
use clap::Parser;
use config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    log::info!("starting up");

    let args = Cli::parse();
    log::info!("reading config from: {}", args.config_file);
    let config = Config::new_from_ini_file(args.config_file)?;

    let http_server = HttpServer::new();

    let mut ring = IoUring::new(8)?;

    let mut clients: HashMap<i32, RcUnsafeClient> = HashMap::new();

    let socket_fd = setup_connection(&config.host);
    log::debug!("socket_fd: {}", socket_fd);

    let _socket = create_sock_addr(&config.host);
    client_accept(&mut ring, socket_fd);

    loop {
        ring.submit_and_wait(1).expect("TODO: panic message");
        log::debug!("waiting for event");
        match completion_queue(&mut ring) {
            CompletionQueueMessage::ClientConnected(client_fd) => unsafe {
                log::info!("client connected! with fd: {}", client_fd);
                let client = Rc::new(UnsafeCell::new(Client::new()));
                clients.insert(client_fd, client.clone());

                client_read(&mut ring, client_fd, client);

                // allow for more clients to be connected
                client_accept(&mut ring, socket_fd)
            }
            CompletionQueueMessage::MessageReceived(client_fd, bytes_read) => unsafe {
                if bytes_read <= 0 {
                    log::info!("client disconnected");
                    continue;
                }
                let client = clients.get(&client_fd).unwrap().clone();
                let message = std::str::from_utf8((&*client.get()).get_read_buffer());
                if message.is_err() {
                    continue;
                }
                let message = message.unwrap();
                log::info!("client sent: {}", message);

                let write_buf = (*client.get()).get_write_buffer();

                if let Ok(http_message) = http_server.process_message(message) {
                    let http_message_bytes = http_message.as_bytes();
                    for i in 0..http_message.len() {
                        write_buf[i] = http_message_bytes[i];
                    }

                    client_send(&mut ring, client_fd, client.clone(), http_message.len() as u32);
                }

                // Add events to io_uring
                client_close(&mut ring, client_fd, client.clone());
            }
            CompletionQueueMessage::MessageSent(_) => {}
            CompletionQueueMessage::ClientClosed(_) => {}
        }
    }
}