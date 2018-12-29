pub fn get_err() -> Option<&'static str> {
    let err = unsafe { gl::GetError() };

    match err {
        gl::NO_ERROR => None,
        gl::INVALID_ENUM =>      Some("GL_INVALID_ENUM"),
        gl::INVALID_VALUE =>     Some("GL_INVALID_VALUE"),
        gl::INVALID_OPERATION => Some("GL_INVALID_OPERATION"),
        gl::STACK_OVERFLOW =>    Some("GL_STACK_OVERFLOW"),
        gl::STACK_UNDERFLOW =>   Some("GL_STACK_UNDERFLOW"),
        gl::OUT_OF_MEMORY =>     Some("GL_OUT_OF_MEMORY"),
        _ => None
    }
}

macro_rules! check_gl {
    () => {
        match crate::gl_error::get_err() {
            Some(str) => println!("{}:{} - {}", file!(), line!(), str),
            None => {}
        }
    }
}
