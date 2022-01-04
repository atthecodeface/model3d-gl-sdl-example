use gl_model::{Transformation, Bone, BoneSet};

fn build_bone_set() -> BoneSet {
    let mut bones = BoneSet::new();
    let b0 = bones.add_bone( Transformation::new(), 0 );
    let b1 = bones.add_bone( Transformation::new().set_translation([1.,0.,0.]), 0 );
    let b2 = bones.add_bone( Transformation::new().set_translation([0.,1.,0.]), 0 );
    let b3 = bones.add_bone( Transformation::new().set_translation([0.,0.,1.]), 0 );
    let b21 = bones.add_bone( Transformation::new().set_translation([0.5,0.,0.]), 0 );
    let b22 = bones.add_bone( Transformation::new().set_translation([0.0,0.,0.5]), 0 );
    bones.relate(b0, b1);
    bones.relate(b0, b2);
    bones.relate(b0, b3);
    bones.relate(b2, b21);
    bones.relate(b2, b22);
    bones.resolve();
    bones.rewrite_indices();
    bones
}

#[test]
fn test_0() {
    let bones = build_bone_set();
    println!("{}",bones);
    assert_eq!(1, bones.iter_roots().count());
    // assert!(false);
}

#[test]
fn test_1() {
    let mut bones = build_bone_set();
    bones.derive_matrices();
    println!("{}",bones);
    assert_eq!(bones.bones.borrow_node(4).borrow_mtb(),
               &[1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., -0.5,-1.,0.,1.]);
    assert_eq!(bones.bones.borrow_node(5).borrow_mtb(),
               &[1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,-1.,-0.5,1.]);
}

