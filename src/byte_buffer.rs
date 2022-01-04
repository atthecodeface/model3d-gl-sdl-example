/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    bezier.rs
@brief   Part of geometry library
 */

//a Notes
//
//

//a Imports

//a ByteBuffer
//tp ByteBuffer
/// A trait for all types that are to be used as sources of data for
/// buffers of, e.g. vertex data, indices, etc
///
/// The data is viewed by OpenGL as a pointer and byte length; these
/// methods provide access to the data in that way.
///
/// These methods are all safe - any use of the information they
/// provide may be unsafe.
pub trait ByteBuffer {
    /// Get the length of the data buffer in bytes
    fn byte_length(&self) -> usize;
    /// Borrow the data as an array of bytes
    fn borrow_bytes (&self) -> &[u8];
    /// Return a pointer to the first byte of the data contents
    fn as_ptr(&self) -> *const u8 { self.borrow_bytes().as_ptr() }
}

//ti ByteBuffer for [T; N]
/// Implement ByteBuffer for [T]
impl <T, const N:usize> ByteBuffer for [T; N] {
    //fp byte_length
    fn byte_length(&self) -> usize {
        std::mem::size_of::<T>() * N
    }

    //fp borrow_bytes
    fn borrow_bytes<'a>(&'a self) -> &'a [u8] {
	unsafe { std::mem::transmute::<&[T], &[u8]>(self) }
    }

    //zz All done
}

//ti ByteBuffer for Vec
/// Implement ByteBuffer for Vec
impl <T> ByteBuffer for Vec<T> {
    //fp byte_length
    fn byte_length(&self) -> usize {
        std::mem::size_of::<T>() * self.len()
    }

    //fp borrow_bytes
    fn borrow_bytes<'a>(&'a self) -> &'a [u8] {
	    unsafe { std::mem::transmute::<&[T], &[u8]>(&self) }
    }

    //zz All done
}
