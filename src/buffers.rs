use gl::{self, types::GLenum};

/// Enum that specifies the type of buffer. Heavily used by BufferManager.
pub enum BufferType {
   VertexBuffer,
   IndexBuffer,
}

/// Keeps track of how many (types of) buffers are currently around.
pub struct BufferManager {
    /// Count of currently bound vertex buffers.
    vertex_buffer: u32,
    /// Count of currently bound index buffers.
    index_buffer: u32,
}

impl BufferManager {
    /// Creates a new BufferManager.
    pub fn new() -> Self {
        Self {
            vertex_buffer: 0,
            index_buffer: 0,
        }
    }

    /// This function should be called whenever a buffer is bound. As a parameter, it takes in
    /// the type of buffer in form of a BufferType. It returns the new size of currently bound
    /// buffers.
    pub fn increase(&mut self, btype: BufferType) -> u32 {
        match btype {
            // you have to specify the path of the variant or Rust will punish you with lots of
            // warnings...
            crate::buffers::BufferType::VertexBuffer => { 
                self.vertex_buffer += 1;
                return self.vertex_buffer;
            },
            crate::buffers::BufferType::IndexBuffer => {
                self.vertex_buffer += 1;
                return self.index_buffer;
            }
        };
    }

    /// This function should be called whenever a buffer is unbound. It decreases the number of
    /// currently bound buffers and returns it.
    pub fn decrease(&mut self, btype: BufferType) -> u32 {
        match btype {
            crate::buffers::BufferType::VertexBuffer => { 
                self.vertex_buffer -= 1;
                return self.vertex_buffer;
            },
            crate::buffers::BufferType::IndexBuffer => {
                self.vertex_buffer -= 1;
                return self.index_buffer;
            }
        };
    }
}

/// A representation of a vertex buffer.
pub struct VertexBuffer {
    /// just the 'name' of the buffer, it's not a pointer! 
    pub handle: u32,        
    /// points to the handle (=> 'name')
    pub ptr: *mut u32,      
    /// indicates whether the buffer is bound or not
    pub is_bound: bool,     
}

impl VertexBuffer {
    /// Create vertex buffer, binding it in the process.
    pub fn new(positions: &mut [f32], floats_per_vertex: usize, bm: &mut BufferManager) -> VertexBuffer {
        // increase the count of currently bound vertex buffers and use the returned number as
        // id or name for the buffer
        let mut handle: u32 = bm.increase(BufferType::VertexBuffer); 
        let ptr: *mut u32 = &mut handle;
        unsafe {
            gl::GenBuffers(1, ptr);
            gl::BindBuffer(gl::ARRAY_BUFFER, handle);
            // provide information about the data stored in the buffer.
            gl::BufferData(gl::ARRAY_BUFFER, (positions.len() * std::mem::size_of::<f32>()) as isize,
                           positions.as_mut_ptr() as *const core::ffi::c_void, gl::STATIC_DRAW);
            // tell opengl how your data is layed out in memory.
            // std::mem::size_of::<f32> * 2 => 2 floats per vertex
            gl::VertexAttribPointer(0, floats_per_vertex as i32, gl::FLOAT, gl::FALSE, (std::mem::size_of::<f32>() * floats_per_vertex) as i32,
                                    0 as *const std::ffi::c_void);
            // 'bind' it on position 0
            gl::EnableVertexAttribArray(0);
        }

        // increase the count of currently bound buffers
        bm.increase(BufferType::VertexBuffer);

        VertexBuffer {
            handle,
            ptr,
            is_bound: true,
        }
    }
    
    /// Bind vertex buffer. If it's already bound, do nothing
    pub fn bind(&self, kind: GLenum, bm: &mut BufferManager) {
        // can't bind buffer that is already bound => do nothing
        if self.is_bound {
            return;
        }

        // increase the count of currently bound buffers...
        bm.increase(BufferType::VertexBuffer);

        unsafe { gl::BindBuffer(kind, self.handle); }
    }
}

/// Represents an index buffer.
pub struct IndexBuffer {
    /// The 'id' or 'name' of the buffer, it's not a pointer but acts as one!
    pub name: u32,
    /// Pointer to the name. An awful lot of OpenGL functions require it...
    pub ptr: *mut u32,
    /// Indicator whether the index buffer is bound or not.
    pub is_bound: bool,
}

impl IndexBuffer {
    /// Create a new index buffer. indices.len() is a perfectly fine value for size.
    pub fn new(size: usize, indices: &mut Vec<u32>, bm: &mut BufferManager) -> Self {
        // increase the count of currently bound (will be bound in a few lines) index buffers +
        // return that number -- it will act as the id or name of the buffer.
        let mut name = bm.increase(BufferType::IndexBuffer);
        let ptr: *mut u32 = &mut name;
        unsafe {
            gl::GenBuffers(1, ptr);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, name);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (size * std::mem::size_of::<u32>()) as isize,
                indices.as_mut_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
        }
        
        IndexBuffer {
            name,
            ptr,
            is_bound: true,
        }
    }

    pub fn bind(&self, bm: &mut BufferManager) {
        if self.is_bound {
            return;
        }

        bm.increase(BufferType::IndexBuffer);
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.name); }
    }
}
