pub trait Readable {
    fn get_mut_ptr(&mut self) -> *mut u8;
}
