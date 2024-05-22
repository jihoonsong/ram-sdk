use crate::MemorySlice;

extern "C" {
    fn storage_has(key_ptr: u32) -> u32;
    fn storage_read(key_ptr: u32) -> u32;
    fn storage_write(key_ptr: u32, value_ptr: u32);
    fn storage_delete(key_ptr: u32);
}

pub struct Storage {}

impl Storage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn has(&self, key: &[u8]) -> bool {
        let key_memory_slice = MemorySlice::new(key.len() as u32);

        key_memory_slice.write(key);

        let value = unsafe { storage_has(key_memory_slice.into_raw_ptr() as u32) };

        value != 0
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let key_memory_slice = MemorySlice::new(key.len() as u32);

        key_memory_slice.write(key);

        let value_ptr = unsafe { storage_read(key_memory_slice.into_raw_ptr() as u32) };

        if value_ptr == 0 {
            return None;
        }

        let value_memory_slice = MemorySlice::from_raw_ptr(value_ptr as *mut MemorySlice);

        let value = value_memory_slice.read();

        Some(value)
    }

    pub fn set(&self, key: &[u8], value: &[u8]) {
        let key_memory_slice = MemorySlice::new(key.len() as u32);
        let value_memory_slice = MemorySlice::new(value.len() as u32);

        key_memory_slice.write(key);
        value_memory_slice.write(value);

        unsafe {
            storage_write(
                key_memory_slice.into_raw_ptr() as u32,
                value_memory_slice.into_raw_ptr() as u32,
            );
        };
    }

    pub fn delete(&self, key: &[u8]) {
        let key_memory_slice = MemorySlice::new(key.len() as u32);

        key_memory_slice.write(key);

        unsafe { storage_delete(key_memory_slice.into_raw_ptr() as u32) };
    }
}
