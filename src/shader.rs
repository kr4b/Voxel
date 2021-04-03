use std::path::Path;

use gl::types::{GLuint, GLint};

#[derive(Debug)]
pub enum ShaderError {
    Io(std::io::Error),
    Shader(String),
    Program(String),
}

pub struct Shader {
    vertex_shader: GLuint,
    fragment_shader: GLuint,
    program: GLuint,
}

impl Shader {
    pub fn new(vertex_bytes: &[u8], fragment_bytes: &[u8]) -> Result<Shader, ShaderError> {
        let program = unsafe { gl::CreateProgram() };
        let vertex_shader = Self::load_shader(vertex_bytes, gl::VERTEX_SHADER)
            .map_err(|x| ShaderError::Shader(x))?;
        let fragment_shader = Self::load_shader(fragment_bytes, gl::FRAGMENT_SHADER)
            .map_err(|x| ShaderError::Shader(x))?;


        unsafe {
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
        }

        let mut success = 0;
        unsafe {
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        }

        if success == gl::TRUE as i32 {
            Ok(Shader {
                vertex_shader,
                fragment_shader,
                program,
            })
        } else {
            let mut len = 0;
            unsafe {
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = String::with_capacity(len as usize);
            unsafe {
                gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), error.as_ptr() as *mut _);
            }

            Err(ShaderError::Program(error))
        }
    }

    pub fn from_files<T>(vertex_path: T, fragment_path: T) -> Result<Shader, ShaderError>
    where
        T: AsRef<Path>,
    {
        let vertex = std::fs::read(vertex_path).map_err(|x| ShaderError::Io(x))?;
        let fragment = std::fs::read(fragment_path).map_err(|x| ShaderError::Io(x))?;

        Self::new(&vertex, &fragment)
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn uniform_location(&self, name: &str) -> GLint {
        let cstr = std::ffi::CString::new(name).unwrap();
        unsafe {
            gl::GetUniformLocation(
                self.program,
                cstr.as_ptr() as *mut _,
            )
        }
    }

    fn load_shader(bytes: &[u8], shader_type: GLuint) -> Result<GLuint, String> {
        let id = unsafe { gl::CreateShader(shader_type) };
        let mut success = 0;
        let cstr = std::ffi::CString::new(bytes).unwrap();

        unsafe {
            gl::ShaderSource(id, 1, &cstr.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == gl::TRUE as i32 {
            Ok(id)
        } else {
            let mut len = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let mut error = Vec::with_capacity(len as usize);
            unsafe {
                error.set_len(len as usize - 1);
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_mut_ptr() as *mut _);
            }

            Err(String::from_utf8(error).unwrap())
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DetachShader(self.program, self.vertex_shader);
            gl::DetachShader(self.program, self.fragment_shader);
            gl::DeleteShader(self.vertex_shader);
            gl::DeleteShader(self.fragment_shader);
            gl::DeleteProgram(self.program);
        }
    }
}
