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

@file    bone.rs
@brief   Bone and bone hierarchy
 */

//a Imports
use geo_nd::matrix;
use indent_display::{IndentedDisplay, IndentedOptions, Indenter};

use crate::hierarchy;
use crate::{Mat4};
use crate::{BoneSet, BonePose};


//a BonePoseSet
//tp BonePoseSet
/// A pose structure for a complete [BoneSet]
///
/// This includes a set of [Mat4] matrix transformations for
/// mesh-space to animated-model-space
pub struct BonePoseSet<'a> {
    /// The BoneSet the pose corresponds to
    bones        : &'a BoneSet,
    /// A pose for every [Bone] in the [BoneSet]
    poses        : Vec<BonePose<'a>>,
    /// A mesh-to-animated-model-space matrix transformation for each
    /// bone
    data         : Vec<Mat4>,
    /// A monotonic counter to allow updating of the matrices once per
    /// animation tick
    last_updated : usize,
}

//ip BonePoseSet
impl <'a> BonePoseSet<'a> {
    //fp new
    /// Create a new [BonePoseSet] for a [BoneSet]
    ///
    /// The [BoneSet] must have been resolved
    pub fn new(bones:&'a BoneSet) -> Self {
        let mut poses = Vec::new();
        for b in bones.bones.borrow_elements().iter() {
            poses.push( BonePose::new(&b.data) );
        }
        let mut data = Vec::new();
        for _ in 0..bones.max_index {
            data.push([0.;16]);
        }
        let last_updated = 0;
        Self { bones, poses, data, last_updated }
    }

    //fp derive_animation
    /// Derive the animation for the current poses of the [BonePoseSet]
    ///
    /// This traverses the hierarchy as required
    pub fn derive_animation(&mut self) {
        let mut mat_depth = 0;
        for (_, recipe) in &self.bones.roots {
            for op in recipe.borrow_ops() {
                match op {
                    hierarchy::NodeEnumOp::Push(n,_) => {
                        if mat_depth == 0 {
                            self.data[mat_depth]   = *self.poses[*n].derive_animation(true, &self.data[mat_depth]);
                        } else {
                            self.data[mat_depth+1] = *self.poses[*n].derive_animation(false, &self.data[mat_depth]);
                        }
                        mat_depth += 1;
                    },
                    _ => {
                        mat_depth -= 1;
                    }
                }
            }
        }
    }

    //fp update
    /// Update the animation matrices if required - depending on the
    /// last updated tick
    pub fn update(&mut self, tick:usize) {
        if tick != self.last_updated {
            self.last_updated = tick;
            self.derive_animation();
            let bones = self.bones.bones.borrow_elements();
            for i in 0..self.poses.len() {
                let matrix_index = bones[i].data.matrix_index;
                self.data[matrix_index] = *self.poses[i].borrow_animated_mtm();
            }
        }
    }
}

//ip IndentedDisplay for BonePoseSet
impl<'a, 'b, Opt: IndentedOptions<'a>> IndentedDisplay<'a, Opt>
    for BonePoseSet<'b>
{
    //mp fmt
    /// Display for humans with indent
    fn indent(&self, f: &mut Indenter<'a, Opt>) -> std::fmt::Result {
        use std::fmt::Write;
        for (_, recipe) in &self.bones.roots {
            let mut sub = f.sub();
            for op in recipe.borrow_ops() {
                match op {
                    hierarchy::NodeEnumOp::Push(n,_) => {
                        sub = sub.sub();
                    },
                    _ => {
                        sub = sub.pop();
                    }
                }
            }
        }
        Ok(())
    }
}

/*
        pass
    #f hier_debug
    def hier_debug(self, hier:Hierarchy) -> Hierarchy:
        hier.add(f"BonePoseSet {self.bones.roots} {self.max_index} {self.last_updated} {self.data}")
        hier.push()
        self.bones.hier_debug(hier)
        for pose in self.poses:
            pose.hier_debug(hier)
            pass
        hier.pop()
        return hier
    #f All done
    pass
 */

        /*
#c AnimatedBonePose
class AnimatedBonePose:
    def __init__(self, poses:List[BonePose]) -> None:
        self.poses = poses
        self.animatable = Bezier2(Transformation())
        self.animatable.set_target( t1=1.,
                                    c0=Transformation( quaternion=Glm.quat.setAxisAngle(Glm.quat.create(), Glm.vec3.fromValues(1.,0.,0.), 0.3)),
                                    c1=Transformation( quaternion=Glm.quat.setAxisAngle(Glm.quat.create(), Glm.vec3.fromValues(1.,0.,0.), 0.3)),
                                    tgt=Transformation(quaternion=Glm.quat.setAxisAngle(Glm.quat.create(), Glm.vec3.fromValues(1.,0.,0.), 0.3)),
                                    callback=self.animation_callback )
        pass
    def interpolate_to_time(self, t:float) -> None:
        z = self.animatable.interpolate_to_time(t)
        # print(t, z)
        self.poses[1].transformation_reset()
        self.poses[1].transform(z)
        pass
    def animation_callback(self, t:float) -> None:
        t_sec = math.floor(t)
        t_int = int(t_sec)
        tgt = 1.0
        if (t_int&1): tgt=-1.
        self.animatable.set_target( t1=t_sec+1.,
                                    c0=Transformation(quaternion=Glm.quat.setAxisAngle(Glm.quat.create(), Glm.vec3.fromValues(1.,0.,0.), 0.3)),
                                    c1=Transformation(quaternion=Glm.quat.setAxisAngle(Glm.quat.create(), Glm.vec3.fromValues(0.,1.,0.), 0.5)),
                                    tgt=Transformation(quaternion=Glm.quat.setAxisAngle(Glm.quat.create(), Glm.vec3.fromValues(1.,0.,0.), tgt*0.3)),
                                    callback=self.animation_callback )
        pass
    pass

*/
