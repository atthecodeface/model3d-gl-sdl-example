mod base_shader;
mod model;
mod objects;

use model3d_gl::Gl;
use model3d_gl::Model3DOpenGL;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut model3d = Model3DOpenGL::new();

    let base = model::Base::new(&mut model3d).unwrap();
    let instantiables = base.make_instantiable(&mut model3d).unwrap();
    let mut game_state = model::GameState::new();
    let mut instances = base.make_instances();

    unsafe {
        let (w, h) = window.drawable_size();
        let w = w as i32;
        let h = h as i32;
        gl::Viewport(0, 0, w, h);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    model3d_gl::opengl_utils::check_errors().unwrap();

    // main loop
    let mut event_pump = sdl.event_pump().unwrap();

    // These are not flags
    unsafe { gl::Enable(gl::CULL_FACE) };
    unsafe { gl::Enable(gl::DEPTH_TEST) };
    model3d_gl::opengl_utils::check_errors().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    // Don't need to do this - it is automatic
                    //
                    // But the drawable is NOT the window size it is the window size
                    // modified by Retinaness
                    let (w, h) = window.drawable_size();
                    let w = w as i32;
                    let h = h as i32;
                    unsafe { gl::Viewport(0, 0, w, h) };
                }
                _ => {}
            }
        }

        model3d_gl::opengl_utils::check_errors().unwrap();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        base.update(
            &mut model3d,
            &mut game_state,
            &instantiables,
            &mut instances,
        );
        model3d_gl::opengl_utils::check_errors().unwrap();

        window.gl_swap_window();
    }
}
