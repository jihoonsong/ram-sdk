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

extern "C" {
    pub fn storage_has(key_ptr: u32) -> u32;
    pub fn storage_read(key_ptr: u32) -> u32;
    pub fn storage_write(key_ptr: u32, value_ptr: u32);
    pub fn storage_delete(key_ptr: u32);
}
