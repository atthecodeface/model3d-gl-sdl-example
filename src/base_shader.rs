//ci VERT_SRC
const VERT_SRC: &str = "
#version 330 core

struct MaterialBaseData {
    vec4 base_color;
    vec4 mrxx;
};

struct Light { // 32 bytes
    vec4 position;
    vec4 color;
};

struct WorldData {
    mat4 view_matrix; // 64 bytes
    Light lights[4];  // 128 bytes
};


layout (location = 0) in vec3 Position;
in vec3 Normal;
out vec3 Normal_frag;
out vec4 World_position;

layout(std140) uniform Material {
    MaterialBaseData material;
};

layout(std140) uniform World {
    WorldData world;
};
uniform mat4 uModelMatrix;
uniform mat4 uMeshMatrix;

void main()
{
    World_position = uModelMatrix * uMeshMatrix * vec4(Position, 1.);
    gl_Position = world.view_matrix * World_position;
    Normal_frag = Normal;
}
";

//ci FRAG_SRC
const FRAG_SRC: &str = "
#version 330 core

struct MaterialBaseData {
    vec4 base_color;
    vec4 mrxx;
};

struct Light { // 32 bytes
    vec4 position;
    vec4 color;
};

struct WorldData {
    mat4 view_matrix; // 64 bytes
    Light lights[4];  // 128 bytes
};

in vec4 World_position;
in vec3 Normal_frag;
out vec4 Color;
layout(std140) uniform Material {
    MaterialBaseData material;
};

layout(std140) uniform World {
    WorldData world;
};

void main()
{
    Color = vec4(0.);
    for(int i=0; i<4; ++i) {
        vec3 light_direction;
        light_direction = world.lights[i].position.xyz - World_position.xyz;
        float distance2;
        distance2 = dot(light_direction, light_direction);
        distance2 = clamp( distance2, world.lights[i].position.w, 1000.);
        if (world.lights[i].position.w <= 0) {
            distance2 = 1.0;
            light_direction = world.lights[i].position.xyz;
        }
        float dot_product = dot( normalize(light_direction), normalize(Normal_frag) );
        dot_product = clamp( dot_product, 0., 1.);
        Color += world.lights[i].color * dot_product / distance2;
    }
}
";

//fp compile
pub fn compile() -> gl_model::GlProgram {
    let mut shader_program = gl_model::GlProgram::compile_program(&[
        (gl::VERTEX_SHADER, VERT_SRC),
        (gl::FRAGMENT_SHADER, FRAG_SRC),
    ])
    .unwrap();
    shader_program
        .add_attr_name("Position", model3d_rs::VertexAttr::Position)
        .unwrap();
    shader_program
        .add_attr_name("Normal", model3d_rs::VertexAttr::Normal)
        .unwrap();
    shader_program
        .add_uniform_name("uModelMatrix", gl_model::UniformId::ModelMatrix)
        .unwrap();
    shader_program
        .add_uniform_name("uMeshMatrix", gl_model::UniformId::MeshMatrix)
        .unwrap();
    shader_program.add_uniform_buffer_name("World", 2).unwrap();
    shader_program
        .add_uniform_buffer_name("Material", 1)
        .unwrap();
    shader_program
}
