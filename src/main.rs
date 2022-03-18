use std::ffi::CString;
use gl_model::{GlShader, GlProgram};

const VERT_SRC : &str = "
#version 330 core

layout (location = 0) in vec3 Position;
in vec3 Normal;
uniform mat4 uModelMatrix;
out vec3 Normal_frag;
void main()
{
    gl_Position = uModelMatrix * vec4(Position, 1.0);
    Normal_frag = Normal;
}
";

const FRAG_SRC : &str = "
#version 330 core

in vec3 Normal_frag;
out vec4 Color;

void main()
{
    Color = vec4(abs(Normal_frag.x), abs(Normal_frag.y), abs(Normal_frag.z), 1.0);
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
    shader_program.add_attr_name("Normal", model3d::VertexAttr::Normal).unwrap();
    shader_program.add_uniform_name("uModelMatrix", gl_model::UniformId::ModelMatrix).unwrap();

    // Using the set of indices/vertex data defined create primitives (a triangle)
    let mut obj: model3d::Object<gl_model::Renderable> = model3d::Object::new();
    let material = model3d::BaseMaterial::rgba((1., 0., 0., 1.));
    let m_id = obj.add_material(&material);

   // Create a triangle object with an empty skeleton
    let mut triangle = model3d::ExampleVertices::new();
    model3d::example_objects::triangle::new::<gl_model::Renderable>(&mut triangle, 0.5);
    let v_id = obj.add_vertices(triangle.borrow_vertices(0));
    let mesh = model3d::example_objects::triangle::mesh(v_id, m_id);
    obj.add_component(None, None, mesh);

    gl_model::check_errors().unwrap();

   // Create a tetrahedron object with an empty skeleton
    let mut tetrahedron = model3d::ExampleVertices::new();
    model3d::example_objects::tetrahedron::new::<gl_model::Renderable>(&mut tetrahedron, 0.5);
    let v_id = obj.add_vertices(tetrahedron.borrow_vertices(0));
    let mesh = model3d::example_objects::tetrahedron::mesh(v_id, m_id);
    let transformation = model3d::Transformation::new()
        .set_translation([0.5,0.,0.])
        ;
    obj.add_component(None, Some(transformation), mesh);

    gl_model::check_errors().unwrap();

    obj.analyze();
    let mut render_context = gl_model::RenderContext{};
    obj.create_client(&mut render_context);

    gl_model::check_errors().unwrap();
   
    let instantiable = obj.into_instantiable();
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
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        gl_model::check_errors().unwrap();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();
        shader_instantiable.gl_draw(&instance);
        let v = [1., 1., 0.];
        instance.transformation.translate(&v, 0.01*t.sin());
        t += 0.1;

        gl_model::check_errors().unwrap();

        window.gl_swap_window();
    }
}
