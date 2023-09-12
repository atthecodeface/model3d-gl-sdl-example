mod base_shader;
mod objects;

#[derive(Debug, Default)]
struct Light {
    position: model3d_gl::Vec4,
    color: model3d_gl::Vec4,
}

#[derive(Debug, Default)]
#[repr(C)]
struct WorldData {
    view_matrix: model3d_gl::Mat4,
    lights: [Light; 4],
}

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
    let shader_program = base_shader::compile_shader_program(&model3d).unwrap();
    // model3d.use_program(Some(&program));

    model3d_gl::opengl_utils::check_errors().expect("Compiled program");

    let instantiable = objects::new(&mut model3d);

    model3d_gl::opengl_utils::check_errors().expect("Created instantiable");

    let shader_instantiable =
        model3d_gl::ShaderInstantiable::new(&mut model3d, &shader_program, &instantiable).unwrap();

    let mut instance = instantiable.instantiate();

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
    let mut t: f32 = 0.0;
    let mut view_transformation = model3d_base::Transformation::new();
    let spin = geo_nd::quat::rotate_x(&geo_nd::quat::identity(), 0.01);

    // let mut material_gl = model3d_gl::GlBuffer::default();
    // material_gl.uniform_buffer(&[0.0_f32; 8]);
    // if let Some(u) = shader_program.uniform(model3d_gl::UniformId::Buffer(1)) {
    // unsafe {
    // println!("Bind to {}", u);
    // gl::BindBufferRange(
    // gl::UNIFORM_BUFFER,
    // u as u32,
    // material_gl.gl_buffer(),
    // 0,  /* offset */
    // 32, /* size */
    // ); // sizeof basedata
    // gl::UniformBlockBinding(shader_program.id(), u as u32, material_gl.gl_buffer());
    // }
    // }
    // model3d_gl::opengl_utils::check_errors().expect("Bound uniform for material");

    // let mut world_gl = model3d_gl::GlBuffer::default();
    // let mut world_data = [WorldData::default(); 1];
    // world_data[0].view_matrix[0] = 1.;
    // world_data[0].view_matrix[5] = 1.;
    // world_data[0].view_matrix[10] = 1.;
    // world_data[0].view_matrix[15] = 1.;
    // world_data[0].lights[0].position = [2., 0., 0., 0.1];
    // world_data[0].lights[0].color = [1., 0., 0., 0.];
    // world_data[0].lights[1].position = [-1., 0., 0., 0.1];
    // world_data[0].lights[1].color = [0., 1., 0., 0.];
    // world_data[0].lights[2].position = [-1., 0., 0., -1.];
    // world_data[0].lights[2].color = [0., 0., 1., 0.];
    // world_gl.uniform_buffer(&world_data);
    // if let Some(u) = shader_program.uniform(model3d_gl::UniformId::Buffer(2)) {
    // // 2 is in base_shader the world ub
    // unsafe {
    // // Bind a range of gl_buffer to binding point '3'
    // println!("Bind to {} {:?}", u, world_data.as_ptr());
    // println!("{:?} {}", world_data, std::mem::size_of::<WorldData>());
    // gl::UniformBlockBinding(
    // shader_program.id(),
    // u as u32,
    // 3, /* binding point made up by us */
    // ); //world_gl.gl_buffer());
    // // Pick the whole of world_data in world_gl buffer as the data for the uniform
    // gl::BindBufferRange(
    // gl::UNIFORM_BUFFER,
    // 3, // binding point made up by us as u32,
    // world_gl.gl_buffer(),
    // 0, /* offset */
    // std::mem::size_of::<WorldData>() as isize,
    // );
    // }
    // } else {
    // panic!("Could not set world data");
    // }
    // model3d_gl::opengl_utils::check_errors().expect("Bound uniform for world");

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

        // Update world_gl.gl_buffer world_data[0] (there is only one)
        // view_transformation.rotate_by(&spin);
        // world_data[0].view_matrix = view_transformation.mat4();

        // unsafe {
        // gl::BindBuffer(gl::UNIFORM_BUFFER, world_gl.gl_buffer());
        // gl::BufferSubData(
        // gl::UNIFORM_BUFFER,
        // 0, /* offset in buffer */
        // std::mem::size_of::<WorldData>() as isize,
        // world_data.as_ptr() as *const std::os::raw::c_void,
        // );
        // }

        shader_program.set_used();
        shader_instantiable.gl_draw(&mut model3d, &instance);
        let v = [1., 1., 0.];
        instance.transformation.translate(&v, 0.01 * t.sin());
        instance.transformation.rotate_by(&spin);
        t += 0.05;

        model3d_gl::opengl_utils::check_errors().unwrap();

        window.gl_swap_window();
    }
}
