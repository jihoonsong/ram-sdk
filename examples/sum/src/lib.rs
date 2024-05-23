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

// NOTE: all you need to do is attaching #[live_object] to the function you want to export.
// TODO: attach #[live_object] macro here.
pub fn sum(x: i32, y: i32) -> i32 {
    x + y
}

// NOTE: below will be generated by #[live_object] macro.
// TODO: implement #[live_object] macro and remove below code.
// Place export functions here to prevent function name collisions.
#[cfg(target_arch = "wasm32")]
mod __export_functions {
    use live_object_sdk::*;

    #[repr(C)]
    #[derive(serde::Deserialize)]
    pub struct SumInput {
        x: i32,
        y: i32,
    }

    #[repr(C)]
    pub struct SumOutput {
        output: i32,
    }

    impl SumInput {
        pub fn from_memory_slice_ptr(memory_slice_ptr: u32) -> Self {
            let memory_slice = MemorySlice::from_raw_ptr(memory_slice_ptr as *mut MemorySlice);
            let input_bytes = memory_slice.read();
            serde_json::from_slice(&input_bytes).unwrap()
        }
    }

    impl SumOutput {
        pub fn into_memory_slice_ptr(output: i32) -> u32 {
            let output_bytes = serde_json::to_vec(&output).unwrap();
            let memory_slice = MemorySlice::new(output_bytes.len() as u32);
            memory_slice.write(&output_bytes);
            memory_slice.into_raw_ptr() as u32
        }
    }

    #[no_mangle]
    pub fn sum(memory_slice_ptr: u32) -> u32 {
        let SumInput { x, y } = SumInput::from_memory_slice_ptr(memory_slice_ptr);
        let output = super::sum(x, y);
        SumOutput::into_memory_slice_ptr(output)
    }
}
