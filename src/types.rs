/// 3-dimensional vector
pub type Vec3 = [f32; 3];

/// 3-dimensional vector with extra coord (1 for position, 0 for direction)
pub type Vec4 = [f32; 4];

/// 3-by-3 matrix for transformation of Vec3
pub type Mat3 = [f32; 9];

/// 4-by-4 matrix for transformation of Vec4
pub type Mat4 = [f32; 16];

/// Quaternion
pub type Quat = [f32; 4];

/// Vertex-attribute object
pub type Vao = gl::types::GLuint;
