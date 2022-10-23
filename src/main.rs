use io_uring::{opcode, types, IoUring, Submitter, SubmissionQueue, CompletionQueue};
use std::os::unix::io::AsRawFd;
use std::{fs, io};
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::hash::Hash;
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

const BUFFER_SIZE: usize = 1024;

enum CompletionQueueMessage {
    ClientConnected(i32),
    MessageReceived(i32),
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
   let accept = opcode::Accept::new(types::Fd(socket_fd as i32),
                                    &mut sock_addr as *mut sockaddr,
                                    sock_len as _)
       .build()
       .user_data(0x42);
   log::debug!("sending accept: {:?}", accept);

   unsafe {
       sqe
           .submission()
           .push(&accept)
           .expect("submission queue is full");
   }
}

unsafe fn client_read(sqe: &mut IoUring, socket_fd: i32, buffer: *mut Vec<u8>) {
    let read = opcode::Recv::new(types::Fd(socket_fd), (*buffer).as_mut_ptr(), BUFFER_SIZE as u32)
        .build()
        .user_data(0x43);

    unsafe {
        sqe
            .submission()
            .push(&read)
            .expect("submission queue is full");
    }
}

fn completion_queue(cqe: &mut IoUring) -> CompletionQueueMessage {
    debug!("reading completion queue");
    let msg = cqe.completion().next().expect("completion queue is empty");

    let syscall_ret_value = msg.result();
    match msg.user_data() {
        0x42 => {
            CompletionQueueMessage::ClientConnected(syscall_ret_value)
        }
        0x43 => {
            CompletionQueueMessage::MessageReceived(syscall_ret_value)
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

    let mut client_buffers: HashMap<i32, UnsafeCell<Vec<u8>>> = HashMap::new();

    let socket_fd = setup_connection();
    log::debug!("socket_fd: {}", socket_fd);

    let socket = create_sock_addr();
    let (sock_addr, mut sock_len) = socket.as_ffi_pair();
    let mut sock_addr = sock_addr.clone();

    let accept = opcode::Accept::new(types::Fd(socket_fd as i32),
                                     &mut sock_addr as *mut sockaddr,
                                     sock_len as *mut socklen_t)
        .build()
        .user_data(0x42);
    log::debug!("sending accept: {:?}", accept);

    unsafe {
        ring
            .submission()
            .push(&accept)
            .expect("submission queue is full");
    }

    loop {
        ring.submit_and_wait(1).expect("TODO: panic message");
        debug!("waiting for event");
        match completion_queue(&mut ring) {
            CompletionQueueMessage::ClientConnected(client_fd) => unsafe {
                client_buffers.insert(client_fd, UnsafeCell::new(vec![0; BUFFER_SIZE]));
                client_read(&mut ring, client_fd, client_buffers[&client_fd].get());
                // allow for more clients to be connected
                client_accept(&mut ring, socket_fd, sock_addr, sock_len)
            }
            CompletionQueueMessage::MessageReceived(client_fd) => unsafe {
                println!("message receieved");
            }
        }
    }
}