/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    material.rs
@brief   Part of 3D geometric model library
 */

//a Documentation

/*!

This provides for abstract Materials which can be used by any 3D model

!*/

//a Imports
use crate::{Texture};

//a Material
#[derive(Debug)]
pub enum MaterialAspect {
    Color,
    Normal,
    MetallicRoughness,
    Occlusion,
    Emission,
}

//tp BaseData
/// The basic data for a material; the most simple material is
/// actually just RGB, but to keep the system simple the [BaseData]
/// includes an alpha, metallicness and roughness.
///
/// For a simple material the alpha should be 1.0, and the metallic 0,
/// and roughness 1
///
/// The simplest of shaders will use just the RGB values
#[derive(Debug)]
pub struct BaseData {
    /// Color of the material
    rgba : (f32, f32, f32, f32),
    /// Metallic nature of the material: 0 is fully dielectric, 1.0 is fully metallic
    metallic : f32,
    /// Roughness of the material:  0.5 is specular, no specular down to 0 full reflection, up to 1 fully matt
    roughness: f32,
}

//ip Default for BaseData
impl Default for BaseData {
    fn default() -> Self {
        Self {
            rgba : (1., 1., 1., 1.),
            metallic: 0.,
            roughness : 1.,
        }
    }
}

//ip BaseData
impl BaseData {
    pub fn rgba(rgba:(f32, f32, f32, f32)) -> Self {
        Self { rgba, metallic:0., roughness:1. }
    }
    pub fn set_metallic(mut self, metallic:f32) -> Self {
        self.metallic = metallic;
        self
    }
    pub fn set_roughness(mut self, roughness:f32) -> Self {
        self.roughness = roughness;
        self
    }
    pub fn set_mr(mut self, metallic:f32, roughness:f32) -> Self {
        self.metallic = metallic;
        self.roughness = roughness;
        self
    }
}

//tt Material
/// A [Material] provides means to access the data for a material, be
/// it simple of full PBR. A fragment shader may require some aspects
/// of a material to be provided to it for rendering, and this API
/// allows that information to be gathered from any kind of material
pub trait Material<T:Texture> {
    /// Borrow the basic data of a material - color and base
    /// metallic/roughness, for example
    fn borrow_base_data(&self) -> &BaseData;
    ///
    fn borrow_texture(&self, _aspect:MaterialAspect) -> Option<&T> {
        None
    }
}

//a BaseMaterial
//tp BaseMaterial
/// Base material that provides simply color and constant metallicness/roughness
#[derive(Debug)]
pub struct BaseMaterial {
    /// Base material data
    base_data : BaseData,
}

//ip BaseMaterial
impl BaseMaterial {
    //fp rgba
    /// Create a new [BaseMaterial] of an RGB color and alpha
    pub fn rgba(rgba:(f32, f32, f32, f32)) -> Self {
        let base_data = BaseData::rgba(rgba);
        Self { base_data }
    }
    //cp set_metallic
    /// Set the metallicness value for the [BaseMaterial]
    pub fn set_metallic(mut self, metallic:f32) -> Self {
        self.base_data = self.base_data.set_metallic(metallic);
        self
    }
    //cp set_roughness
    /// Set the roughness value for the [BaseMaterial]
    pub fn set_roughness(mut self, roughness:f32) -> Self {
        self.base_data = self.base_data.set_roughness(roughness);
        self
    }
    //cp set_mr
    /// Set the metallicness and roughness value for the [BaseMaterial]
    pub fn set_mr(mut self, metallic:f32, roughness:f32) -> Self {
        self.base_data = self.base_data.set_mr(metallic, roughness);
        self
    }
}

//ip Material for BaseMaterial
impl <T:Texture> Material<T> for BaseMaterial {
    fn borrow_base_data(&self) -> &BaseData {
        &self.base_data
    }
}

//a TexturedMaterial
//tp TexturedMaterial
/// A simple textured material with a color and optional normal map
#[derive(Debug)]
pub struct TexturedMaterial<T:Texture> {
    base_data : BaseData,
    base_texture     : Option<T>,
    normal_texture   : Option<T>,
}

//ip Material for TexturedMaterial
impl <T:Texture> Material<T> for TexturedMaterial<T> {
    fn borrow_base_data(&self) -> &BaseData {
        &self.base_data
    }
}

//a PbrMaterial
//tp PbrMaterial
/// A physically-based rendered material with full set of textures
#[derive(Debug)]
pub struct PbrMaterial<T:Texture> {
    base_data : BaseData,
    base_texture     : Option<T>,
    normal_texture   : Option<T>,
    mr_texture       : Option<T>,
    occlusion_texture: Option<T>,
    emission_texture : Option<T>,
}

impl <T:Texture> PbrMaterial<T> {
    pub fn new() -> Self {
        Self {
            base_data : BaseData::default(),
            base_texture: None,
            normal_texture: None,
            mr_texture: None,
            occlusion_texture: None,
            emission_texture: None,
        }
    }
}

/*
    #f gl_create
    def gl_create(self) -> None:
        if self.base_texture is not None:      self.base_texture.gl_create()
        if self.mr_texture is not None:        self.mr_texture.gl_create()
        if self.normal_texture is not None:    self.normal_texture.gl_create()
        if self.occlusion_texture is not None: self.occlusion_texture.gl_create()
        if self.emission_texture is not None:  self.emission_texture.gl_create()
        pass
    #f gl_program_configure
    def gl_program_configure(self, program:ShaderProgram) -> None:
        if self.base_texture is not None:
            assert self.base_texture.texture is not None
            GL.glActiveTexture(GL.GL_TEXTURE0)
            GL.glBindTexture(GL.GL_TEXTURE_2D, self.base_texture.texture)
            program.set_uniform_if("uMaterial.base_texture",
                                   lambda u:GL.glUniform1i(u, 0) )
            pass
        program.set_uniform_if("uMaterial.base_color",
                               lambda u: GL.glUniform4f(u, self.color[0], self.color[1], self.color[2], self.color[3], ) )
        pass
    #f __str__
    def __str__(self) -> str:
        r = str(self.__dict__)
        return r
    #f All done
    pass

 */
