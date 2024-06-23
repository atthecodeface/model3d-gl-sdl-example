use model3d_gl::Gl;

// Working:
//
// "Avocado.glb", ["0"]
// "BoomBox.glb", ["0"]   (scale 10)
// "Duck.glb", ["0"]
// "DamagedHelmet.glb", ["0"]
// "WaterBottle.glb", ["0"]
// "BarramundiFish.glb", ["0"]
// "AntiqueCamera.glb", ["0", "1"] (scale 0.1)
// "Lantern.glb", ["3"] (scale 0.05)
// ReciprocatingSaw.glb 0,365 scale 0.003
// "2CylinderEngine.glb", ["0", "81"] scale 0.001
// "ToyCar.glb", ["0"] scale 0.001
//
// Not working
//
// Fox is skinned
// "Fox.glb", ["0", "1"]
// GearboxAssy.glb, 0, 117
pub fn new<G: Gl>(render_context: &mut G) -> Result<model3d_base::Instantiable<G>, String> {
    // let filename = "DamagedHelmet.glb";
    let filename = "ToyCar.glb";
    let node_names = ["0"];
    fn buf_reader(
        file: &mut std::fs::File,
        byte_length: usize,
    ) -> Result<Option<Vec<u8>>, std::io::Error> {
        use std::io::Read;
        let mut buffer = vec![0; byte_length];
        file.read_exact(&mut buffer)?;
        Ok(Some(buffer))
    }
    let mut file = std::fs::File::open(filename).map_err(|e| format!("{e:?}"))?;
    let (mut gltf, opt_buffer_0) = model3d_gltf::glb_load(&mut file, &buf_reader, 16 * 1000 * 1000)
        .map_err(|e| format!("{e:?}"))?;

    let mut od = model3d_gltf::ObjectData::new(&gltf);
    for n in node_names {
        od.add_object(&gltf, gltf.get_node(n).unwrap());
    }
    od.derive_uses(&gltf);
    let buffers = od
        .gen_byte_buffers(&mut gltf, &model3d_gltf::buf_parse_fail, opt_buffer_0)
        .map_err(|e| format!("{e:?}"))?;
    let buffer_data = od.gen_buffer_data::<_, _, G>(&|x| &buffers[x]);
    let buffer_accessors = od.gen_accessors(&gltf, &|x| &buffer_data[x]);
    let vertices = od.gen_vertices(&gltf, &|x| &buffer_accessors[x]);
    let mut obj = od.gen_object(&gltf, &vertices);

    let material = model3d_base::BaseMaterial::rgba((1., 0., 0., 1.));
    let _ = obj.add_material(&material);

    obj.analyze();
    obj.into_instantiable(render_context).map_err(|(_, e)| e)
}
