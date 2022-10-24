use io_uring::{opcode, types, IoUring, Submitter, SubmissionQueue, CompletionQueue};
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
use nix::sys::socket::IpAddr;
use nix::sys::socket::SockaddrLike;
use nix::sys::socket::sockaddr;
use std::mem;
use log::debug;
use nix::libc::socklen_t;
use libc;
use std::ptr;
use libc::user;

const BUFFER_SIZE: usize = 1024;
type ClientFd = u32;
type BytesRead = u32;

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

enum CompletionQueueMessage {
    ClientConnected(i32),
    MessageReceived(i32, BytesRead),
    MessageSent(i32),
}

fn create_sock_addr() -> SockAddr {
    let localhost = IpAddr::new_v4(0,0,0,0);
    let addr = InetAddr::new(localhost, 5123);
    SockAddr::new_inet(addr)
}

/// Create a tcp listener
fn setup_connection() -> usize {
    let socket_fd = match unsafe { syscall!(Sysno::socket, Inet, 1, 0) } {
        Ok(fd) => fd,
        Err(err) => panic!("unable to get socket"),
    };

    let socket_addr = create_sock_addr();


    match unsafe { syscall!(Sysno::bind, socket_fd, (socket_addr.as_ptr() as usize), mem::size_of::<SockAddr>()) } {
        Ok(0) => 0,
        _ => panic!("unable to bind to socket"),
    };

    match unsafe { syscall!(Sysno::listen, socket_fd, 10) } {
        Ok(0) => (),
        _ => panic!("unable to listen")
    }

    socket_fd
}

fn client_accept(sqe: &mut IoUring, socket_fd: usize, mut sock_addr: sockaddr, sock_len: socklen_t) {
   //let accept = opcode::Accept::new(types::Fd(socket_fd as i32),
   //                                 &mut sock_addr as *mut sockaddr,
   //                                 sock_len as _)
   //    .build()
   //    .user_data(0x42);
   let user_data: u64 = ((0 as u64) << 32) | (0 as u64);
   let accept = opcode::Accept::new(types::Fd(socket_fd as i32),
    ptr::null_mut(), ptr::null_mut())
       .build()
       .user_data(user_data);
   log::debug!("sending accept: {:?}", accept);

   unsafe {
       sqe
           .submission()
           .push(&accept)
           .expect("submission queue is full");
   }
}

unsafe fn client_read(sqe: &mut IoUring, socket_fd: i32, buffer: *mut Vec<u8>) {
    let buffer_ptr = (*buffer).as_mut_ptr();
    let user_data: u64 = ((socket_fd as u64) << 32) | (1 as u64);
    let read = opcode::Recv::new(types::Fd(socket_fd), buffer_ptr, BUFFER_SIZE as u32)
        .build()
        .user_data(user_data);

    unsafe {
        sqe
            .submission()
            .push(&read)
            .expect("submission queue is full");
    }
}

unsafe fn client_send(sqe: &mut IoUring, socket_fd: i32, buffer: *mut Vec<u8>, len: u32) {
    let buffer_ptr = (*buffer).as_mut_ptr();
    let user_data: u64 = ((socket_fd as u64) << 32) | (2 as u64);
    let send = opcode::Send::new(types::Fd(socket_fd), buffer_ptr, len)
        .build()
        .user_data(user_data);

    unsafe {
        sqe
            .submission()
            .push(&send)
            .expect("submission queue is full");
    }
}

fn completion_queue(cqe: &mut IoUring) -> CompletionQueueMessage {
    let msg = cqe.completion().next().expect("completion queue is empty");

    let syscall_ret_value = msg.result();
    if syscall_ret_value < 0 {
        // This is an error
        let error = nix::errno::Errno::from_i32(-syscall_ret_value);
        panic!("syscall error: {:?}", error);
    }
    let msg_userdata = msg.user_data();
    let client_fd = (msg_userdata >> 32) as u32;
    let msg_type = msg_userdata as u32;
    match msg_type {
        0 => {
            CompletionQueueMessage::ClientConnected(syscall_ret_value as i32)
        }
        1 => {
            CompletionQueueMessage::MessageReceived(client_fd as i32, syscall_ret_value as u32)
        }
        2 => {
            CompletionQueueMessage::MessageSent(client_fd as i32)
        }
        _ => panic!("help me")
    }
}

fn main() -> io::Result<()> {
    env_logger::init();
    log::info!("starting up");

    let mut ring = IoUring::new(8)?;

    // sqe = submitted queue
    // cqe = completion queue
    // let (mut submitter, mut sqe, mut cqe) = ring.split();

    let mut client_read_buffers: HashMap<i32, UnsafeCell<Vec<u8>>> = HashMap::new();
    let mut client_write_buffers: HashMap<i32, UnsafeCell<Vec<u8>>> = HashMap::new();

    let socket_fd = setup_connection();
    log::debug!("socket_fd: {}", socket_fd);

    let socket = create_sock_addr();
    let (sock_addr, mut sock_len) = socket.as_ffi_pair();
    let mut sock_addr = sock_addr.clone();
    client_accept(&mut ring, socket_fd, sock_addr, sock_len);

    loop {
        ring.submit_and_wait(1).expect("TODO: panic message");
        debug!("waiting for event");
        match completion_queue(&mut ring) {
            CompletionQueueMessage::ClientConnected(client_fd) => unsafe {
                log::info!("client connected! with fd: {}", client_fd);
                client_read_buffers.insert(client_fd, UnsafeCell::new(vec![0; BUFFER_SIZE]));
                client_write_buffers.insert(client_fd, UnsafeCell::new(vec![0; BUFFER_SIZE]));

                client_read(&mut ring, client_fd, client_read_buffers[&client_fd].get());

                // allow for more clients to be connected
                client_accept(&mut ring, socket_fd, sock_addr, sock_len)
            }
            CompletionQueueMessage::MessageReceived(client_fd, bytes_read) => unsafe {
                let message = std::str::from_utf8(&(&*client_read_buffers[&client_fd].get())[0..(bytes_read as usize)]).unwrap();
                log::info!("client sent: {}", message);
                client_write_buffers[&client_fd].get()
                    .write(HTTP_MESSAGE.as_bytes().to_vec());

                client_read(&mut ring, client_fd, client_read_buffers[&client_fd].get());
                client_send(&mut ring, client_fd, client_write_buffers[&client_fd].get(), HTTP_MESSAGE.len() as u32);
            }
            CompletionQueueMessage::MessageSent(_) => {}
        }
    }
}