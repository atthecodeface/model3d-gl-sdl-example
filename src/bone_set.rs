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

@file    bone_set.rs
@brief   Bone hierarchy
 */

//a Imports
use indent_display::{IndentedDisplay, IndentedOptions, NullOptions, Indenter};

use crate::hierarchy;
use crate::Bone;
use crate::Mat4;
use crate::Transformation;

//a BoneSet
//tp BoneSet
/// A set of related bones, with one or more roots
///
/// This corresponds to a skeleton (or a number thereof), with each
/// bone appearing once in each skeleton. The bones form a hierarchy.
pub struct BoneSet {
    /// The bones that make up the set, with the hierarchical relationships
    pub bones: hierarchy::Hierarchy<Bone>,
    /// The roots of the bones and hierarchical recipes for traversal
    pub roots: Vec<(usize, hierarchy::Recipe)>,
    /// An array of matrices long enough for the one per level of traversal
    pub temp_mat4s: Vec<Mat4>,
    /// Max bone index
    pub max_index: usize,
}

//ip BoneSet
impl BoneSet {
    //fp new
    /// Create a new set of bones
    pub fn new() -> Self {
        let bones = hierarchy::Hierarchy::new();
        let roots = Vec::new();
        let temp_mat4s = Vec::new();
        Self {
            bones,
            roots,
            temp_mat4s,
            max_index: 0,
        }
    }

    //mp add_bone
    /// Add a bone with a given base [Transformation] relative to its
    /// parent (if it has one), and an index to a Vec of Mat4 that the
    /// bone pose will utilize
    ///
    /// It returns the bone reference index
    pub fn add_bone(&mut self, transformation: Transformation, matrix_index: usize) -> usize {
        self.roots.clear();
        let bone = Bone::new(transformation, matrix_index);
        self.bones.add_node(bone)
    }

    //mp relate
    /// Relate a parent bone to a child bone (by bone reference indices)
    pub fn relate(&mut self, parent: usize, child: usize) {
        self.bones.relate(parent, child);
    }

    //mi find_max_matrix_index
    /// Find the maximum matrix index of all the bones (plus 1)
    fn find_max_matrix_index(&mut self) {
        let mut max_index = 0;
        for b in self.bones.borrow_elements() {
            if b.data.matrix_index >= max_index {
                max_index = b.data.matrix_index + 1
            }
        }
        self.max_index = max_index;
    }

    //mp resolve
    /// Resolve the [BoneSet] by finding the roots, generating
    /// traversal [hierarchy::Recipe]s for each root, allocating the
    /// required number of temporary [Mat4]s for the deepest of all
    /// the recipes, and finding the number of bone matrices required
    /// to be exported
    pub fn resolve(&mut self) {
        if self.roots.len() == 0 {
            self.bones.find_roots();
            for r in self.bones.borrow_roots() {
                self.roots
                    .push((*r, hierarchy::Recipe::of_ops(self.bones.enum_from(*r))));
            }
            let mut max_depth = 0;
            for (_, recipe) in &self.roots {
                max_depth = if recipe.depth() > max_depth {
                    recipe.depth()
                } else {
                    max_depth
                };
            }
            self.temp_mat4s = Vec::new();
            for _ in 0..max_depth {
                self.temp_mat4s.push([0.; 16]);
            }
            self.find_max_matrix_index();
        }
    }

    //mp rewrite_indices
    /// Rewrite the bone matrix indices from 0 if required
    ///
    /// Each bone in the [BoneSet] is allocated the matrix index as it
    /// is reached through traversal from the roots of the [BoneSet].
    pub fn rewrite_indices(&mut self) {
        self.resolve();
        if self.max_index < self.bones.len() {
            let mut bone_count = 0;
            let (_, bones) = self.bones.borrow_mut();
            for (_, recipe) in &self.roots {
                for op in recipe.borrow_ops() {
                    match op {
                        hierarchy::NodeEnumOp::Push(n, _) => {
                            bones[*n].data.matrix_index = bone_count;
                            bone_count += 1;
                        }
                        _ => {}
                    }
                }
            }
            self.max_index = bone_count;
        }
    }

    //mp derive_matrices
    /// Derive the matrices (as specified by [Bone]) for every bone in
    /// the [BoneSet] after the bones have been resolved.
    ///
    ///
    pub fn derive_matrices(&mut self) {
        assert!(
            self.roots.len() != 0,
            "Resolve MUST have been invoked prior to derive_matrices"
        );
        let (_, bones) = self.bones.borrow_mut();
        let mut mat_depth = 0;
        for (_, recipe) in &self.roots {
            for op in recipe.borrow_ops() {
                match op {
                    hierarchy::NodeEnumOp::Push(n, _) => {
                        if mat_depth == 0 {
                            self.temp_mat4s[mat_depth] = *bones[*n]
                                .data
                                .derive_matrices(true, &self.temp_mat4s[mat_depth]);
                        } else {
                            self.temp_mat4s[mat_depth] = *bones[*n]
                                .data
                                .derive_matrices(false, &self.temp_mat4s[mat_depth-1]);
                        }
                        mat_depth += 1;
                    }
                    _ => {
                        mat_depth -= 1;
                    }
                }
            }
        }
    }

    //fp iter_roots
    /// Iterate through the root bone indices in the [BoneSet]
    pub fn iter_roots<'z>(&'z self) -> impl Iterator<Item = usize> + '_ {
        self.roots.iter().map(|(n, _)| *n)
    }

    //zz All done
}

//ip IndentedDisplay for BoneSet
impl<'a, Opt: IndentedOptions<'a>> IndentedDisplay<'a, Opt>
    for BoneSet
{
    //mp fmt
    /// Display for humans with indent
    fn indent(&self, f: &mut Indenter<'a, Opt>) -> std::fmt::Result {
        self.bones.indent(f)
    }
}

//ip Display for BoneSet
impl std::fmt::Display for BoneSet
{
    //mp fmt
    /// Display for humans with indent
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut v = Vec::<u8>::new();
        let mut ind = Indenter::new(&mut v, " ", &NullOptions {});
        self.indent(&mut ind)?;
        drop(ind);
        write!(f, "{}", &String::from_utf8(v).unwrap())
    }
}

