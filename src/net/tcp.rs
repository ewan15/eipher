use syscalls::{Sysno, syscall};
use nix::sys::socket::AddressFamily::Inet;

const PORT: u16 = 8080;

pub fn setup_connection() -> usize {
    let socket_fd = match unsafe { syscall!(Sysno::socket, Inet, 1, 0) } {
        Ok(fd) => fd,
        Err(err) => panic!("unable to get socket"),
    };

    let socket_addr = create_sock_addr();

    match unsafe { syscall!(Sysno::bind, socket_fd, (&socket_addr as *const _), 16) } {
        Ok(0) => 0,
        _ => panic!("unable to bind to socket"),
    };

    match unsafe { syscall!(Sysno::listen, socket_fd, 10) } {
        Ok(0) => (),
        _ => panic!("unable to listen")
    }

    socket_fd
}

pub fn create_sock_addr() -> std::net::SocketAddrV4 {
    std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), PORT)
}
