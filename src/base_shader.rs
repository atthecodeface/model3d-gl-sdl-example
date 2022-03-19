const VERT_SRC : &str = "
#version 330 core

layout (location = 0) in vec3 Position;
in vec3 Normal;
uniform Material {
    vec4 base_color;
};
uniform mat4 uModelMatrix;
uniform mat4 uViewMatrix;
uniform mat4 uMeshMatrix;
out vec3 Normal_frag;
void main()
{
    gl_Position = uViewMatrix * uModelMatrix * uMeshMatrix * vec4(Position, 1.0);
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

pub fn compile() -> gl_model::GlProgram {
    let mut shader_program = gl_model::GlProgram::compile_program(&[
        (gl::VERTEX_SHADER, VERT_SRC),
        (gl::FRAGMENT_SHADER, FRAG_SRC),
    ]).unwrap();
    shader_program.add_attr_name("Position", model3d::VertexAttr::Position).unwrap();
    shader_program.add_attr_name("Normal", model3d::VertexAttr::Normal).unwrap();
    shader_program.add_uniform_name("uModelMatrix", gl_model::UniformId::ModelMatrix).unwrap();
    shader_program.add_uniform_name("uMeshMatrix", gl_model::UniformId::MeshMatrix).unwrap();
    shader_program.add_uniform_name("uViewMatrix", gl_model::UniformId::ViewMatrix).unwrap();
    shader_program
}
