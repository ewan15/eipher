use io_uring::{opcode, types, IoUring};
use std::os::unix::io::AsRawFd;
use std::{fs, io};
use syscalls::{Sysno, syscall};
use nix::sys::socket::AddressFamily::Inet;
use nix::sys::socket::SockAddr;
use nix::sys::socket::InetAddr;
use nix::sys::socket::IpAddr;
use nix::sys::socket::SockaddrLike;
use nix::sys::socket::sockaddr;
use std::mem;

fn create_sock_addr() -> SockAddr {
    let localhost = IpAddr::new_v4(0,0,0,0);
    let addr = InetAddr::new(localhost, 5123);
    SockAddr::new_inet(addr)
}

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
        Ok(_) => (),
        Err(_) => panic!("unable to listen")
    }

    socket_fd
}

fn main() -> io::Result<()> {
    let mut ring = IoUring::new(8)?;

    let socket_fd = setup_connection();
    let socket = create_sock_addr();
    let (sock_addr, mut sock_len) = socket.as_ffi_pair();
    let mut sock_addr = sock_addr.clone();

    let accept = opcode::Accept::new(types::Fd(socket_fd as i32), &mut sock_addr as *mut sockaddr, sock_len as *mut u32)
        .build()
        .user_data(0x42);

    unsafe {
        ring.submission()
            .push(&accept)
            .expect("submission queue is full");
    }

    ring.submit_and_wait(1)?;

    let cqe = ring.completion().next().expect("completion queue is empty");


    Ok(())
}
