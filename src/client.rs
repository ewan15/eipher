use std::cell::UnsafeCell;
use std::rc::Rc;

use crate::types::{Readable, Writeable};

pub const BUFFER_SIZE: usize = 10240;

pub struct Client {
    read_buffer: Vec<u8>,
    write_buffer: Vec<u8>,
}

pub type RcUnsafeClient = Rc<UnsafeCell<Client>>;

impl Client {
    pub fn new() -> Self {
        Self{
            read_buffer: vec![0; BUFFER_SIZE],
            write_buffer: vec![0; BUFFER_SIZE],
        }
    }

    pub fn handle_message() {

    }

    fn decode_message() {

    }

    pub(crate) fn get_read_buffer(&self) -> &[u8] {
        self.read_buffer.as_slice()
    }

    pub(crate) fn get_write_buffer(&mut self) -> &mut Vec<u8> {
        &mut self.write_buffer
    }
}

impl Readable for Client {
    fn get_mut_ptr(&mut self) -> *mut u8 {
        self.read_buffer.as_mut_ptr()
    }
}

impl Writeable for Client {
    fn get_mut_ptr(&mut self) -> *mut u8 {
        self.write_buffer.as_mut_ptr()
    }
}
