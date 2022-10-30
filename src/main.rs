mod client;
mod types;
mod _io_uring;
mod net;

use io_uring::IoUring;
use std::os::unix::io::AsRawFd;
use std::{fs, io};
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::Write;
use syscalls::{Sysno, syscall};
use nix::sys::socket::AddressFamily::Inet;
use nix::sys::socket::SockAddr;
use nix::sys::socket::InetAddr;
use nix::sys::socket::SockaddrLike;
use nix::sys::socket::sockaddr;
use std::mem;
use nix::libc::socklen_t;
use libc;
use std::ptr;
use libc::user;
use std::rc::Rc;
use crate::client::{Client, RcUnsafeClient};
use crate::_io_uring::{CompletionQueueMessage, client_accept, client_read, client_send, completion_queue};
use crate::net::{setup_connection, create_sock_addr};
use crate::types::{Readable, Writeable};

const HTTP_MESSAGE: &str =
"
HTTP/1.1 200 OK
Date: Mon, 23 May 2005 22:38:34 GMT
Content-Type: text/html; charset=UTF-8
Content-Length: 155
Last-Modified: Wed, 08 Jan 2003 23:11:55 GMT
Server: Apache/1.3.3.7 (Unix) (Red-Hat/Linux)
ETag: \"3f80f-1b6-3e1cb03b\"
Accept-Ranges: bytes
Connection: close

<html>
  <head>
    <title>Ewan Website</title>
  </head>
  <body>
    <p>Hello. Welcome to my website. Built with io_uring.</p>
  </body>
</html>
";

fn main() -> io::Result<()> {
    env_logger::init();
    log::info!("starting up");

    let mut ring = IoUring::new(8)?;

    let mut clients: HashMap<i32, RcUnsafeClient> = HashMap::new();

    let socket_fd = setup_connection();
    log::debug!("socket_fd: {}", socket_fd);

    let socket = create_sock_addr();
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
                let client = clients.get(&client_fd).unwrap().clone();
                // let read_buffer = Readable::get_mut_ptr(&mut *client.get());
                let message = std::str::from_utf8((&*client.get()).get_read_buffer()).unwrap();
                log::info!("client sent: {}", message);
                let write_buf = (*client.get()).get_write_buffer();
                let http_message_bytes = HTTP_MESSAGE.as_bytes();
                for i in 0..HTTP_MESSAGE.len() {
                    write_buf[i] = http_message_bytes[i];
                }

                client_send(&mut ring, client_fd, client.clone(), HTTP_MESSAGE.len() as u32);

                // Add events to io_uring
                client_read(&mut ring, client_fd, client.clone());
            }
            CompletionQueueMessage::MessageSent(_) => {}
        }
    }
}