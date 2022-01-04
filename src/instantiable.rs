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

@file    drawable.rs
@brief   Part of OpenGL library
 */

//a Notes
//
//

//a Imports
use geo_nd::{matrix};

use crate::{Mat4, Transformation, BoneSet, Instance};

//a Instantiable
//ti BoneSetAndOffset
struct BoneSetAndOffset {
    set:BoneSet,
    bone_matrix_index:usize
}

//ti MeshIndexData
pub struct MeshIndexData {
    /// Index to the mesh matrices of the first mesh for the BoneSet
    pub mesh_matrix_index: usize,
    /// Pair of usize
    pub bone_matrices    : (usize, usize)
}

//tp Instantiable
/// An Instantiable is a type that is related to a set of Mesh data, which can be instanced for different drawable::Instance's
///
/// It requires a related set of Mesh data that it does not refer to:
/// in object construction this Mesh data is likely to be the
/// structures containing vertex information and so on resident on a
/// CPU; in rendering the Mesh data is likely to be graphical objects
/// (such as OpenGL VAOs) that may reside (at least in part) on the
/// GPU.
///
/// The Instantiable data must be kept available to its related Instance.
///
/// The content of the Instantiable includes an array of BoneSets and
/// mesh transformation matrices, with appropriate index values. These
/// index values are into the related set of Mesh data.
pub struct Instantiable {
    /// The sets of bones, each of which will have a pose, and a corresponding first bone matrix
    bones   : Vec<BoneSetAndOffset>,
    /// Transformation matrices for the meshes
    pub mesh_matrices   : Vec<Mat4>,
    /// An array indexed by the associated mesh data index value, and for each such index the content
    /// is an index in to this structure's mesh_matrices, and the range of bone matrices required by that associated mesh data.
    /// If the associated mesh data requires no bones then the tuple will be (0,0)
    mesh_data : Vec<MeshIndexData>,
    /// Number of bone matrices required for all the bone sets in this structure
    pub num_bone_matrices : usize,
}

//ip Instantiable
impl Instantiable {
    //fp new
    /// Create a new instantiable drawable - something to which meshes
    /// and bone sets will be added, and for which a set of mesh
    /// matrices and rest bone positions will be derived.
    ///
    /// Such a type can that be 'instance'd with a specific
    /// transformation and bone poses, and such instances can then be
    /// drawn using shaders.
    pub fn new() -> Self {
        let bones = Vec::new();
        let mesh_matrices = Vec::new();
        let mesh_data = Vec::new();
        let num_bone_matrices = 0;
        Self { bones, mesh_matrices, mesh_data, num_bone_matrices }
    }

    //mp add_mesh
    /// Add a mesh with an optional parent mesh_data index (and hence parent transformation) and bone_matrices
    pub fn add_mesh(&mut self, parent:&Option<usize>, transformation:&Option<Mat4>, bone_matrices:&(usize,usize)) -> usize {
        let mesh_matrix_index = {
            if let Some(parent) = parent {
                let parent = self.mesh_data[*parent].mesh_matrix_index;
                if let Some(transformation) = transformation {
                    let n = self.mesh_matrices.len();
                    // let t = transformation.mat4();
                    let m = matrix::multiply4(&self.mesh_matrices[parent], transformation);
                    self.mesh_matrices.push(m);
                    n
                } else {
                    parent
                }
            } else if let Some(transformation) = transformation { // parent is none
                let n = self.mesh_matrices.len();
                // let t = transformation.mat4();
                let t = transformation.clone();
                self.mesh_matrices.push(t);
                n
            } else { // both are none - requires an identity matrix
                let n = self.mesh_matrices.len();
                self.mesh_matrices.push(matrix::identity4());
                n
            }
        };
        let n = self.mesh_data.len();
        self.mesh_data.push( MeshIndexData {mesh_matrix_index, bone_matrices:*bone_matrices} );
        n
    }

    //mp add_bone_set
    /// Add a bone set; clones it, and generates a number of bone matrices and updates appropriately, returning the range of bone matrices that the set corresponds to
    pub fn add_bone_set(&mut self, _bone_set:&BoneSet) -> (usize, usize) {
        (0,0)
    }

    //mp borrow_mesh_data
    /// Borrow the mesh data
    pub fn borrow_mesh_data (&self, index:usize) -> &MeshIndexData {
        &self.mesh_data[index]
    }

    //mp instantiate
    /// Create an `Instance` from this instantiable - must be used with accompanying mesh data in the appropriate form for the client
    /// Must still add bone_poses one per bone set
    pub fn instantiate<'a>(&'a self) -> Instance<'a> {
        Instance::new(self, self.num_bone_matrices)
    }

    //zz All done
}

