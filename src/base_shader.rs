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
    float scale = 0.001;
    World_position = uModelMatrix * uMeshMatrix * (vec4(scale,scale,scale,1.0)*vec4(Position, 1.));
    //     gl_Position = world.view_matrix * World_position;
    //     gl_Position = World_position * vec4(0.1,0.1,0.1,1.0);
    //         gl_Position = World_position * vec4(10,10,10,1.0);
    gl_Position = World_position;
    Normal_frag = (uModelMatrix * uMeshMatrix * vec4(Normal, 0.)).xyz;
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
use model3d_gl::{Gl, GlShaderType};

pub fn compile_shader_program<G: Gl>(model3d: &G) -> Result<<G as Gl>::Program, String> {
    let frag_shader = model3d.compile_shader(GlShaderType::Fragment, FRAG_SRC)?;
    let vert_shader = model3d.compile_shader(GlShaderType::Vertex, VERT_SRC)?;

    model3d.link_program(
        &[&vert_shader, &frag_shader],
        &[
            ("Position", model3d_base::VertexAttr::Position),
            ("Normal", model3d_base::VertexAttr::Normal),
        ],
        &[
            ("uModelMatrix", model3d_gl::UniformId::ModelMatrix),
            ("uMeshMatrix", model3d_gl::UniformId::MeshMatrix),
        ],
        &[("World", 2), ("Material", 1)],
    )
}
