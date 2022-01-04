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
use crate::{BufferData, BufferClientID};

//a BufferView
/// A subset of a `BufferData`, used for vertex attributes;
/// hence for use in a vertex attribute pointer.
///
/// A `BufferView` is used for a single attribute of a set of data, such as
/// Position or Normal.
pub struct BufferView<'a, T:BufferClientID> {
    /// The `BufferData` that contains the actual vertex attribute data
    data: &'a BufferData<'a, T>,
    // Number of elements per vertex - 1 to 4
    count: u32,
    /// The type of each element, e.g. GL_FLOAT
    gl_type : gl::types::GLenum,
    /// Offset from start of buffer to first byte of data
    offset : u32,
    /// Stride of data in the buffer - 0 for count*sizeof(gl_type)
    stride : u32,
}

//ip BufferView
impl<'a, T:BufferClientID> BufferView<'a, T> {
    //fp new
    /// Create a new view of a `BufferData`
    pub fn new(data:&'a BufferData<'a, T>, count:u32, gl_type:gl::types::GLenum, offset:u32, stride:u32) -> Self {
        Self { data, count, gl_type, offset, stride }
    }

    //mp create_client
    /// Create the OpenGL buffer required by the BufferView
    pub fn create_client(&self) {
        self.data.create_client(0)
    }

    /*
    //mp gl_bind_attribute
    /// Bind this view to a particular attribute
    pub fn gl_bind_attribute(&self, attr:gl::types::GLuint) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.data.gl_buffer());
            gl::EnableVertexAttribArray(attr);
            gl::VertexAttribPointer(attr, // index
                                    self.count, // size
                                    self.gl_type, // types
                                    gl::FALSE, // normalized
                                    self.stride, // stride
                                    std::mem::transmute::<usize, *const std::os::raw::c_void>(self.offset) // ptr
            );
        }
    }
     */

    //zz All done
}

//ip Display for BufferView
impl <'a, T:BufferClientID> std::fmt::Display for BufferView<'a, T> {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f,"BufferView[{}#{}]\n  {}+{}+n*{}\n", self.gl_type, self.count, self.data, self.offset,self.stride)
    }
}

//ip DefaultIndentedDisplay for BufferView
impl <'a, T:BufferClientID> indent_display::DefaultIndentedDisplay for BufferView<'a, T> {}
