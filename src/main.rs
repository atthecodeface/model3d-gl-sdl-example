use std::ffi::CString;
use gl_model::{GlShader, GlProgram};

const VERT_SRC : &str = "
#version 330 core

layout (location = 0) in vec3 Position;

void main()
{
    gl_Position = vec4(Position, 1.0);
}
";

const FRAG_SRC : &str = "
#version 330 core

out vec4 Color;

void main()
{
    Color = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
";

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

    let mut shader_program = GlProgram::compile_program(&[
        (gl::VERTEX_SHADER, VERT_SRC),
        (gl::FRAGMENT_SHADER, FRAG_SRC),
    ]).unwrap();
    shader_program.add_attr_name("Position", model3d::VertexAttr::Position).unwrap();

   // Create a triangle object with an empty skeleton
    let mut triangle = model3d::ExampleVertices::new();
    model3d::example_objects::triangle::new::<gl_model::Renderable>(&mut triangle, 0.5);

    // Using the set of indices/vertex data defined create primitives (a triangle)
    let material = model3d::BaseMaterial::rgba((1., 0., 0., 1.));
    let mut obj: model3d::Object<gl_model::Renderable> = model3d::Object::new();
    let v_id = obj.add_vertices(triangle.borrow_vertices(0));
    let m_id = obj.add_material(&material);
    let mut mesh = model3d::Mesh::new();
    mesh.add_primitive(model3d::Primitive::new(
        model3d::PrimitiveType::Triangles,
        v_id,
        0,
        3,
        m_id,
    ));
    obj.add_component(None, None, mesh);
    obj.analyze();
    let mut render_context = gl_model::RenderContext{};
    obj.create_client(&mut render_context);
   
    let instantiable = obj.into_instantiable();
    // let shader_instantiable = obj.bind_shader(&instantiable, &shader_program);
    let instance = instantiable.instantiate();
    let shader_instantiable = gl_model::ShaderInstantiable::new(&shader_program, &instantiable);
    // set up shared state for window

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // main loop
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();
        shader_instantiable.gl_draw(&instance);
        window.gl_swap_window();
    }
}
