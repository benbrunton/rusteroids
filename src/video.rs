use sdl2;

pub fn main() {
    sdl2::init(sdl2::InitVideo);

    let title = "rusteroids";
    let window_x_pos = sdl2::video::PosCentered;
    let window_y_pos = sdl2::video::PosCentered;
    let window_width = 800;
    let window_height = 600;
    /*  Flags
        Fullscreen              |   SDL_WINDOW_FULLSCREEN           |   fullscreen window
        OpenGL                  |   SDL_WINDOW_OPENGL               |   window usable with OpenGL context
        Shown                   |   SDL_WINDOW_SHOWN                |   window is visible
        Hidden                  |   SDL_WINDOW_HIDDEN               |   window is not visible
        Borderless              |   SDL_WINDOW_BORDERLESS           |   no window decoration
        Resizable               |   SDL_WINDOW_RESIZABLE            |   window can be resized
        Minimized               |   SDL_WINDOW_MINIMIZED            |   window is minimized
        Maximized               |   SDL_WINDOW_MAXIMIZED            |   window is maximized
        InputGrabbed            |   SDL_WINDOW_INPUT_GRABBED        |   window has grabbed input focus
        InputFocus              |   SDL_WINDOW_INPUT_FOCUS          |   window has input focus
        MouseFocus              |   SDL_WINDOW_MOUSE_FOCUS          |   window has mouse focus
        FullscreenDesktop       |   SDL_WINDOW_FULLSCREEN_DESKTOP   |   fullscreen window
        Foreign                 |   SDL_WINDOW_FOREIGN              |   window not created by SDL
        SDL_WINDOW_OPENGL
        
        OR'd together
        e.g:

        sdl2::video::OpenGL | sdl2::video::Borderless
     */
    let window_flags = /*sdl2::video::Fullscreen | */ sdl2::video::OpenGL | sdl2::video::Borderless;

    let window = match sdl2::video::Window::new(title, window_x_pos, window_y_pos, window_width, window_height, window_flags) {
        Ok(window) => window,
        Err(err) => fail!(format!("failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::Accelerated) {
        Ok(renderer) => renderer,
        Err(err) => fail!(format!("failed to create renderer: {}", err))
    };

    let _ = renderer.set_draw_color(sdl2::pixels::RGB(255, 0, 0));
    let _ = renderer.clear();
    renderer.present();

    'main : loop {
        'event : loop {
            match sdl2::event::poll_event() {
                sdl2::event::QuitEvent(_) => break 'main,
                sdl2::event::KeyDownEvent(_, _, key, _, _) => {
                    if key == sdl2::keycode::EscapeKey {
                        break 'main
                    }
                },
                sdl2::event::NoEvent => break 'event,
                // MouseButtonDownEvent(event.timestamp as uint, window,
                                     // event.which as uint,
                                     // mouse::wrap_mouse(event.button),
                                     // event.x as int, event.y as int)
                sdl2::event::MouseButtonDownEvent(_, _, _, _, x, y) => println!("{}, {}", x, y),
                _ => {},
            }

        }
    }

    sdl2::quit();
}
