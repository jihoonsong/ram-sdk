// Copyright (C) 2024 Jihoon Song

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

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
