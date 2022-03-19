
pub fn new(render_context:&mut gl_model::RenderContext) -> model3d::Instantiable<gl_model::Renderable> {
    let mut vertices = model3d::ExampleVertices::new();
    let material = model3d::BaseMaterial::rgba((1., 0., 0., 1.));
    
    let mut obj: model3d::Object<gl_model::Renderable> = model3d::Object::new();
    
    // Using the set of indices/vertex data defined create primitives (a triangle)
    let m_id = obj.add_material(&material);

    // Create a triangle object with an empty skeleton
    model3d::example_objects::triangle::new::<gl_model::Renderable>(&mut vertices, 0.5);
    model3d::example_objects::tetrahedron::new::<gl_model::Renderable>(&mut vertices, 0.5);
    let v_id = obj.add_vertices(vertices.borrow_vertices(0));
    let mesh = model3d::example_objects::triangle::mesh(v_id, m_id);
    obj.add_component(None, None, mesh);

    gl_model::check_errors().unwrap();

    // Create a tetrahedron object with an empty skeleton
    let v_id = obj.add_vertices(vertices.borrow_vertices(1));
    let mesh = model3d::example_objects::tetrahedron::mesh(v_id, m_id);
    let transformation = model3d::Transformation::new()
        .set_translation([0.5,0.,0.])
        ;
    obj.add_component(None, Some(transformation), mesh);

    gl_model::check_errors().unwrap();

    obj.analyze();
    obj.create_client(render_context);
    obj.into_instantiable()
}
