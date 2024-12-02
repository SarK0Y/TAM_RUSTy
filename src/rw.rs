use std::mem;
use std::io::Read;
use std::fs;
use std::error::Error;
pub fn read_file_to_vec < T > (path: String) -> Result <Vec < T >, Box <dyn Error> > {
    let t_size = mem::size_of::<T>();
    let u8_size = mem::size_of:: <u8>();
    let ratio = t_size / u8_size;
    unsafe {
        let mut read_file = fs::read (&path)?;
        let capacity = read_file.capacity() / ratio;
        let len = read_file.len() / ratio;
        let mut ptr: *mut T = read_file.as_mut_ptr() as *mut T;
        let mut ret_vec = Vec::<T>::from_raw_parts (ptr, capacity, len,); 
        mem::forget (read_file);
        Ok(ret_vec)
    }
}