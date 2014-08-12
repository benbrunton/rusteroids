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
use std::rand::Rng;

mod actor;
mod actor_manager;
mod spaceship;
mod bullet;
mod asteroid;
mod kamikaze;
mod explosion;

// Shader sources
// vertex shader
static VS_SRC: &'static str =
   "#version 150\n\
    in vec4 shape;\n\
    uniform vec3 position;\n\
    uniform vec2 camera;\n\
    void main() {\n\
        float x = shape[0];\n\
        float y = shape[1];\n\
        float x_pos = position[0];\n\
        float y_pos = position[1];\n\
        float angle = position[2];\n\
        float c_x   = camera[0];\n\
        float c_y   = camera[1];\n\
        float xx = (x * cos(angle) + y * sin(angle)) + x_pos - c_x;\n\
        float yy = (-x * sin(angle) + y * cos(angle)) + y_pos - c_y;\n\
        gl_Position = vec4(xx, yy, 0.0, 1.0);\n\
    }";


// fragment shader
static FS_SRC: &'static str =
   "#version 150\n\
    out vec4 out_color;\n\
    uniform vec3 color;\n\
    void main() {\n\
       out_color = vec4(color, 0.8);\n\
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
    let mut loc;
    let mut cam;
    let mut color;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);

        // Generate a buffer for the indices
 // GLuint elementbuffer;
 // glGenBuffers(1, &elementbuffer);
 // glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, elementbuffer);


        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Use shader program
        gl::UseProgram(program);
        "out_color".with_c_str(|ptr| gl::BindFragDataLocation(program, 0, ptr));

        // Specify the layout of the vertex data
        let pos_attr = "shape".with_c_str(|ptr| gl::GetAttribLocation(program, ptr));
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint,     // must match the layout in the shader.
                                2,                      // size
                                gl::FLOAT,              // type
                                gl::FALSE as GLboolean, // normalized?
                                0,                      // stride
                                ptr::null());           // array buffer offset

        loc = "position".with_c_str(|ptr| gl::GetUniformLocation(program, ptr));
        cam = "camera".with_c_str(|ptr| gl::GetUniformLocation(program, ptr));
        color = "color".with_c_str(|ptr| gl::GetUniformLocation(program, ptr));
    }

    let global_time = time::get_time();
    let mut t = time::get_time();
    let mut inner_t = time::get_time();
    let fr:i32 = 100000000 / 60;


    let mut actors = actor_manager::ActorManager::new();
    actors.new_player();
    
    while !window.should_close() {

        // Poll events
        glfw.poll_events();

        let t2 = time::get_time();


        //////////////////////////////////////
        //
        //     INNER LOOP
        //     
        //////////////////////////////////////


        // switch to 1 frame a second
        //let fr = 1000000000;


        if t2.nsec - fr > t.nsec || t2.sec > t.sec {


            t = t2;

            let mut messages = vec!();

            for event in glfw::flush_messages(&events) {
                handle_window_event(&window, event, &mut messages);
            }


            calculate_collisions(&actors, &mut messages);


            let mut output_messages = vec!();
            actors.update(messages, &mut output_messages);

            draw_scene(&actors, loc, cam, color, &window);

            actors.process_messages(&mut output_messages);

            generate_actors(&mut actors);



            // every second
            let t3 = time::get_time();
            if t3.sec > inner_t.sec {
                inner_t = t3;
                println!("::  {}s  ::::::::::::::::::::::::::::::", t3.sec - global_time.sec);
                println!("# of actors : {}", actors.get().len());
                for &actor in actors.get().iter(){
                    if actor.id == 1 {

                        println!("Player::");
                        println!(":: x  :: {}", actor.x);
                        println!(":: y  :: {}", actor.y);
                        break;
                    }
                }

                println!("::  {}s  ::::::::::::::::::::::::::::::\n", t3.sec - global_time.sec);

                restart(&mut actors);
                
            }

        }



        //////////////////////////////////////
        //
        // END OF INNER LOOP
        // 
        ///////////////////////////////////////
        

        //window.set_should_close(true);

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

fn generate_actors(actors: &mut actor_manager::ActorManager){
    let max_actors = 30;
    let mut player_pos:actor::ActorView = actor::ActorView{id:0, x:0.0, y:0.0, width:0, height:0, rotation:0.0, parent:0, shape:vec!(), color:vec!()};

    for &mut actor in actors.get().iter(){
        if actor.id == 1 {
            player_pos = actor;
            break;
        }
    }

    while actors.get().len() < max_actors {
        let x = std::rand::task_rng().gen_range(player_pos.x as i32 - 4000, player_pos.x as i32 + 4000);
        let y = std::rand::task_rng().gen_range(player_pos.y as i32 - 4000, player_pos.y as i32 + 4000);
        
        let x_dis = x - player_pos.x as i32;
        let y_dis = y - player_pos.y as i32;
        let distance = ((x_dis * x_dis + y_dis * y_dis) as f32).sqrt();

        if distance > 2600.0{
            let rand = std::rand::task_rng().gen_range(0u32, 100);
            match rand {
                0..50  => actors.new_asteroid(x, y),
                51..79 => actors.new_spaceship(x, y, 0.0),
                80..89 => actors.new_kamikaze(x, y, (player_pos.x, player_pos.y)),
                _      => ()
            }
        }
    }
}

fn restart(actors: &mut actor_manager::ActorManager){
    let mut player_exists = false;
    for &mut actor in actors.get().iter(){
        if actor.id == 1 {
            player_exists = true;
            break;
        }
    }

    if !player_exists{
        actors.restart();
    }
}


fn calculate_collisions(actor_manager: &actor_manager::ActorManager, messages: &mut Vec<(i32, &str)>){

    let actors = actor_manager.get();

    for &a1 in actors.iter(){
        let actors2 = actor_manager.get();

        for &a2 in actors2.iter(){

            if     a1.id    == 0 
                || a2.id    == 0
                || a1.id    == a2.id
                || a1.id    == a2.parent
                || a2.id    == a1.parent {
                continue;
            }

            let d = 100.0;
            
            if a1.x + d > a2.x && a1.x - d < a2.x && a1.y + d > a2.y && a1.y - d < a2.y {
                messages.push((a1.id, "die"));
            }
        }
    }

}

fn handle_window_event(window: &glfw::Window, (time, event): (f64, glfw::WindowEvent), messages : &mut Vec<(i32, &str)>) {
    match event {
        // glfw::PosEvent(x, y)                => window.set_title(format!("Time: {}, Window pos: ({}, {})", time, x, y).as_slice()),
        // glfw::SizeEvent(w, h)               => window.set_title(format!("Time: {}, Window size: ({}, {})", time, w, h).as_slice()),
        glfw::CloseEvent                    => println!("Time: {}, Window close requested.", time),
        glfw::RefreshEvent                  => println!("Time: {}, Window refresh callback triggered.", time),
        glfw::FocusEvent(true)              => println!("Time: {}, Window focus gained.", time),
        glfw::FocusEvent(false)             => println!("Time: {}, Window focus lost.", time),
        glfw::IconifyEvent(true)            => println!("Time: {}, Window was minimised", time),
        glfw::IconifyEvent(false)           => println!("Time: {}, Window was maximised.", time),
        glfw::FramebufferSizeEvent(w, h)    => println!("Time: {}, Framebuffer size: ({}, {})", time, w, h),
        // glfw::CharEvent(character)          => println!("Time: {}, Character: {}", time, character),
        glfw::MouseButtonEvent(btn, action, mods) => println!("Time: {}, Button: {}, Action: {}, Modifiers: [{}]", time, glfw::ShowAliases(btn), action, mods),
        // glfw::CursorPosEvent(xpos, ypos)    => window.set_title(format!("Time: {}, Cursor position: ({}, {})", time, xpos, ypos).as_slice()),
        glfw::CursorEnterEvent(true)        => println!("Time: {}, Cursor entered window.", time),
        glfw::CursorEnterEvent(false)       => println!("Time: {}, Cursor left window.", time),
        // glfw::ScrollEvent(x, y)             => window.set_title(format!("Time: {}, Scroll offset: ({}, {})", time, x, y).as_slice()),
        glfw::KeyEvent(key, /* scancode */ _, action, /* mods */ _ ) => {
            // println!("Time: {}, Key: {}, ScanCode: {}, Action: {}, Modifiers: [{}]", time, key, scancode, action, mods);
            match (key, action) {
                (glfw::KeyEscape, glfw::Press) => window.set_should_close(true),
                (glfw::KeyUp, glfw::Press) => messages.push((1, "begin_increase_throttle")),
                (glfw::KeyDown, glfw::Press) => messages.push((1, "begin_decrease_throttle")),
                (glfw::KeyUp, glfw::Release) => messages.push((1, "stop_increase_throttle")),
                (glfw::KeyDown, glfw::Release) => messages.push((1, "stop_decrease_throttle")),
                (glfw::KeyRight, glfw::Press) => messages.push((1, "begin_rotate_right")),
                (glfw::KeyLeft, glfw::Press) => messages.push((1, "begin_rotate_left")),
                (glfw::KeyRight, glfw::Release) => messages.push((1, "stop_rotate_right")),
                (glfw::KeyLeft, glfw::Release) => messages.push((1, "stop_rotate_left")),
                (glfw::KeySpace, glfw::Release) => messages.push((1, "fire")),
                // (glfw::KeyR, glfw::Press) => {
                //     // Resize should cause the window to "refresh"
                //     let (window_width, window_height) = window.get_size();
                //     window.set_size(window_width + 1, window_height);
                //     window.set_size(window_width, window_height);
                // }
                _ => {}
            }
        },

        _ => ()
    }
}

fn draw_scene(actor_manager:&actor_manager::ActorManager, loc:i32, cam:i32, color:i32, window: &glfw::Window){

    let actors = actor_manager.get();

    let mut cx:f32 = 0.0;
    let mut cy:f32 = 0.0;
    
    gl::ClearColor(0.2, 0.2, 0.4, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);

    for &v in actors.iter() {
        if v.id == 1 {
            cx = v.x;
            cy = v.y;
        }

        draw_actor(&v, loc, cam, color, cx, cy);
    }

    window.swap_buffers();
}

fn draw_actor(p: &actor::ActorView, loc:i32, cam:i32, color:i32, cx: f32, cy: f32){
    
    unsafe{

        gl::BufferData(gl::ARRAY_BUFFER,
               (p.shape.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
               mem::transmute(&p.shape[0]),
               gl::DYNAMIC_DRAW); // STATIC | DYNAMIC | STREAM

        
        gl::Uniform3f(loc, p.x / 2000.0, p.y / 2000.0, p.rotation);
        gl::Uniform2f(cam, cx / 2000.0, cy / 2000.0);
        gl::Uniform3f(color, p.color[0], p.color[1], p.color[2]);
    }

    
    // Draw a triangle from the 3 vertices
    // LINE_LOOP
    gl::DrawArrays(gl::TRIANGLES, 0, p.shape.len() as i32 / 2);
}
