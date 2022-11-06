use std::str::FromStr;
use syscalls::{Sysno, syscall};
use nix::sys::socket::AddressFamily::Inet;
use crate::config;
use config::Host;

pub fn setup_connection(host: &Host) -> usize {
    let socket_fd = match unsafe { syscall!(Sysno::socket, Inet, 1, 0) } {
        Ok(fd) => fd,
        Err(err) => panic!("unable to get socket"),
    };

    let socket_addr = create_sock_addr(&host);

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

pub fn create_sock_addr(host: &Host) -> std::net::SocketAddrV4 {
    let ip = std::net::Ipv4Addr::from_str(&*host.hostname).expect("unable to parse hostname");
    std::net::SocketAddrV4::new(ip, host.port)
}
