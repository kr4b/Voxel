use gl::types::GLuint;

pub struct VertexArray {
    id: GLuint,
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        VertexArray {
            id,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}