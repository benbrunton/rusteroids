#![feature(globs)]

extern crate gl;
extern crate glfw;
extern crate native;
extern crate time;

use gl::types::*;
use glfw::Context;
use std::mem;
use std::ptr;
use std::str;

mod actor;

// Vertex data
static VERTEX_DATA: [GLfloat, ..6] = [
     0.0,  0.05,
     0.025, -0.05,
    -0.025, -0.05
];

// Shader sources
// vertex shader
static VS_SRC: &'static str =
   "#version 150\n\
    in vec2 position;\n\
    uniform float y_pos;\n\
    uniform float x_pos;\n\
    uniform float angle;\n\
    void main() {\n\
        float x = position[0];\n\
        float y = position[1];\n\
        float xx = (x * cos(angle) + y * sin(angle)) + x_pos;\n\
        float yy = (-x * sin(angle) + y * cos(angle)) + y_pos;\n\
       gl_Position = vec4(xx, yy, 0.0, 1.0);\n\
    }";


// fragment shader
static FS_SRC: &'static str =
   "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(1.0, 0.3, 0.3, 0.8);\n\
    }";


fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader = gl::CreateShader(ty);
    unsafe {
        // Attempt to compile the shader
        src.with_c_str(|ptr| gl::ShaderSource(shader, 1, &ptr, ptr::null()));
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar);
            fail!("{}", str::from_utf8(buf.as_slice()).expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);
    unsafe {
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar);
            fail!("{}", str::from_utf8(buf.as_slice()).expect("ProgramInfoLog not valid utf8"));
        }
    }
    program
}

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Choose a GL profile that is compatible with OS X 10.7+
    glfw.window_hint(glfw::ContextVersion(3, 2));
    glfw.window_hint(glfw::OpenglForwardCompat(true));
    glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));

    let (window, events) = glfw.create_window(800, 600, "rusteroids", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_all_polling(true);

    // It is essential to make the context current before calling `gl::load_with`.
    window.make_current();

    // Load the OpenGL function pointers
    gl::load_with(|s| glfw.get_proc_address(s));

    // Create GLSL shaders
    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&VERTEX_DATA[0]),
                       gl::DYNAMIC_DRAW); // STATIC | DYNAMIC | STREAM

        // Use shader program
        gl::UseProgram(program);
        "out_color".with_c_str(|ptr| gl::BindFragDataLocation(program, 0, ptr));

        // Specify the layout of the vertex data
        let pos_attr = "position".with_c_str(|ptr| gl::GetAttribLocation(program, ptr));
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint,     // must match the layout in the shader.
                                2,                      // size
                                gl::FLOAT,              // type
                                gl::FALSE as GLboolean, // normalized?
                                0,                      // stride
                                ptr::null());           // array buffer offset

    }

    let mut t = time::get_time();
    let mut player = actor::Actor::new(0, 0, 10, 10);
    let fr:i32 = 100000000 / 60;  

    while !window.should_close() {
        // Poll events
        glfw.poll_events();

        for event in glfw::flush_messages(&events) {
            handle_window_event(&window, event, &mut player);
        }

        let t2 = time::get_time();
        if t2.nsec - fr > t.nsec || t2.sec > t.sec {

            t = t2;

            player.update();

            let p = player.get_view();

            //println!("r: {}\nx: {}\ny: {}", p.rotation, p.x, p.y);

            unsafe{
                let loc = "y_pos".with_c_str(|ptr| gl::GetUniformLocation(program, ptr));
                gl::Uniform1f(loc, p.y / 2000.0);

                let loc = "x_pos".with_c_str(|ptr| gl::GetUniformLocation(program, ptr));
                gl::Uniform1f(loc, p.x / 2000.0);

                let loc = "angle".with_c_str(|ptr| gl::GetUniformLocation(program, ptr));
                gl::Uniform1f(loc, p.rotation);
            }

            // Clear the screen to black
            gl::ClearColor(0.2, 0.2, 0.4, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Draw a triangle from the 3 vertices
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            window.swap_buffers();
        }

    }

    // Cleanup
    gl::DeleteProgram(program);
    gl::DeleteShader(fs);
    gl::DeleteShader(vs);
    unsafe {
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}

fn handle_window_event(window: &glfw::Window, (time, event): (f64, glfw::WindowEvent), player : &mut actor::Actor) {
    match event {
        glfw::PosEvent(x, y)                => window.set_title(format!("Time: {}, Window pos: ({}, {})", time, x, y).as_slice()),
        glfw::SizeEvent(w, h)               => window.set_title(format!("Time: {}, Window size: ({}, {})", time, w, h).as_slice()),
        glfw::CloseEvent                    => println!("Time: {}, Window close requested.", time),
        glfw::RefreshEvent                  => println!("Time: {}, Window refresh callback triggered.", time),
        glfw::FocusEvent(true)              => println!("Time: {}, Window focus gained.", time),
        glfw::FocusEvent(false)             => println!("Time: {}, Window focus lost.", time),
        glfw::IconifyEvent(true)            => println!("Time: {}, Window was minimised", time),
        glfw::IconifyEvent(false)           => println!("Time: {}, Window was maximised.", time),
        glfw::FramebufferSizeEvent(w, h)    => println!("Time: {}, Framebuffer size: ({}, {})", time, w, h),
        glfw::CharEvent(character)          => println!("Time: {}, Character: {}", time, character),
        glfw::MouseButtonEvent(btn, action, mods) => println!("Time: {}, Button: {}, Action: {}, Modifiers: [{}]", time, glfw::ShowAliases(btn), action, mods),
        glfw::CursorPosEvent(xpos, ypos)    => window.set_title(format!("Time: {}, Cursor position: ({}, {})", time, xpos, ypos).as_slice()),
        glfw::CursorEnterEvent(true)        => println!("Time: {}, Cursor entered window.", time),
        glfw::CursorEnterEvent(false)       => println!("Time: {}, Cursor left window.", time),
        glfw::ScrollEvent(x, y)             => window.set_title(format!("Time: {}, Scroll offset: ({}, {})", time, x, y).as_slice()),
        glfw::KeyEvent(key, scancode, action, mods) => {
            println!("Time: {}, Key: {}, ScanCode: {}, Action: {}, Modifiers: [{}]", time, key, scancode, action, mods);
            match (key, action) {
                (glfw::KeyEscape, glfw::Press) => window.set_should_close(true),
                (glfw::KeyUp, glfw::Press) => player.begin_increase_throttle(),
                (glfw::KeyDown, glfw::Press) => player.begin_decrease_throttle(),
                (glfw::KeyUp, glfw::Release) => player.stop_increase_throttle(),
                (glfw::KeyDown, glfw::Release) => player.stop_decrease_throttle(),
                (glfw::KeyRight, glfw::Press) => player.begin_rotate_right(),
                (glfw::KeyLeft, glfw::Press) => player.begin_rotate_left(),
                (glfw::KeyRight, glfw::Release) => player.stop_rotate_right(),
                (glfw::KeyLeft, glfw::Release) => player.stop_rotate_left(),
                // (glfw::KeyR, glfw::Press) => {
                //     // Resize should cause the window to "refresh"
                //     let (window_width, window_height) = window.get_size();
                //     window.set_size(window_width + 1, window_height);
                //     window.set_size(window_width, window_height);
                // }
                _ => {}
            }
        }
    }
}
