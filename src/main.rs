use gl_model::{ShaderClass};

mod base_shader;
mod objects;

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

    let shader_program = base_shader::compile();

    gl_model::check_errors().expect("Compiled program");
    
    let mut render_context = gl_model::RenderContext{};
    let instantiable = objects::new(&mut render_context);

    gl_model::check_errors().expect("Created instantiable");
   
    let shader_instantiable = gl_model::ShaderInstantiable::new(&shader_program, &instantiable);

    let mut instance = instantiable.instantiate();

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    gl_model::check_errors().unwrap();
    
    // main loop
    let mut event_pump = sdl.event_pump().unwrap();
    let mut t : f32 = 0.0;
    let mut view_transformation = model3d::Transformation::new();
    let spin = geo_nd::quat::rotate_x(&geo_nd::quat::identity(),0.01);


    let mut material_gl = gl_model::GlBuffer::default();
    material_gl.uniform_buffer(&[0.0_f32;8]);
    if let Some(u) = shader_program.uniform(gl_model::UniformId::Buffer(1)) {
        unsafe {
            println!("Bind to {}",u);
            gl::BindBufferRange( gl::UNIFORM_BUFFER,
                                 u as u32,
                                 material_gl.gl_buffer(),
                                 0 /* offset */,
                                 32 /* size */); // sizeof basedata
            gl::UniformBlockBinding( shader_program.id(), u as u32, material_gl.gl_buffer());
        }
    }
    gl_model::check_errors().expect("Bound uniform for material");

    // These are not flags
    unsafe {gl::Enable(gl::CULL_FACE)};
    unsafe {gl::Enable(gl::DEPTH_TEST)};
    gl_model::check_errors().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        gl_model::check_errors().unwrap();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        shader_program.set_used();
        if let Some(u) = shader_program.uniform(gl_model::UniformId::ViewMatrix) {
            unsafe {gl::UniformMatrix4fv(u, 1, gl::FALSE, view_transformation.mat4().as_ptr());}
        }
        shader_instantiable.gl_draw(&instance);
        let v = [1., 1., 0.];
        instance.transformation.translate(&v, 0.01*t.sin());
        t += 0.1;
        view_transformation.rotate_by(&spin);

        gl_model::check_errors().unwrap();

        window.gl_swap_window();
    }
}
