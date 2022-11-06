

use std::cell::UnsafeCell;












use std::ptr;

use io_uring::{opcode, types, IoUring};
use std::rc::Rc;
use crate::types::{Readable, Writeable};


type BytesRead = i32;

pub enum CompletionQueueMessage {
    ClientConnected(i32),
    MessageReceived(i32, BytesRead),
    MessageSent(i32),
    ClientClosed(i32),
}

pub fn client_accept(sqe: &mut IoUring, socket_fd: usize) {
    let user_data: u64 = (0_u64 << 32) | 0_u64;
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

pub unsafe fn client_read<T>
(sqe: &mut IoUring, socket_fd: i32, buffer: Rc<UnsafeCell<T>>)
    where
        T: Readable,
{
    let buffer_ptr = Readable::get_mut_ptr(&mut *buffer.get());
    let user_data: u64 = ((socket_fd as u64) << 32) | 1_u64;
    let read = opcode::Recv::new(types::Fd(socket_fd), buffer_ptr, crate::client::BUFFER_SIZE as u32)
        .build()
        .user_data(user_data);

    let raw_ptr = Rc::into_raw(buffer);
    Rc::increment_strong_count(raw_ptr);
    sqe
        .submission()
        .push(&read)
        .expect("submission queue is full");
}

pub unsafe fn client_send<T>(sqe: &mut IoUring, socket_fd: i32, buffer: Rc<UnsafeCell<T>>, len: u32)
where
    T: Writeable
{
    let buffer_ptr = Writeable::get_mut_ptr(&mut *buffer.get());
    let user_data: u64 = ((socket_fd as u64) << 32) | 2_u64;
    let send = opcode::Send::new(types::Fd(socket_fd), buffer_ptr, len)
        .build()
        .user_data(user_data);

        let raw_ptr = Rc::into_raw(buffer);
        Rc::increment_strong_count(raw_ptr);
        sqe
            .submission()
            .push(&send)
            .expect("submission queue is full");
}

pub unsafe fn client_close<T>
(sqe: &mut IoUring, socket_fd: i32, buffer: Rc<UnsafeCell<T>>)
    where
        T: Readable,
{
    let user_data: u64 = ((socket_fd as u64) << 32) | 3_u64;
    let read = opcode::Close::new(types::Fd(socket_fd))
        .build()
        .user_data(user_data);

        let raw_ptr = Rc::into_raw(buffer);
        Rc::increment_strong_count(raw_ptr);
        sqe
            .submission()
            .push(&read)
            .expect("submission queue is full");
}


pub fn completion_queue(cqe: &mut IoUring) -> CompletionQueueMessage {
    let msg = cqe.completion().next().expect("completion queue is empty");

    let syscall_ret_value = msg.result();
    if syscall_ret_value < 0 {
        // This is an error
        let _error = nix::errno::Errno::from_i32(-syscall_ret_value);
        // panic!("syscall error: {:?}", error);
    }
    let msg_userdata = msg.user_data();
    let client_fd = (msg_userdata >> 32) as u32;
    let msg_type = msg_userdata as u32;
    match msg_type {
        0 => {
            CompletionQueueMessage::ClientConnected(syscall_ret_value as i32)
        }
        1 => {
            CompletionQueueMessage::MessageReceived(client_fd as i32, syscall_ret_value)
        }
        2 => {
            CompletionQueueMessage::MessageSent(client_fd as i32)
        }
        3 => {
            CompletionQueueMessage::ClientClosed(client_fd as i32)
        }
        _ => panic!("help me")
    }
}

