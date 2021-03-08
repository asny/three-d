
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let window = Window::new("Wireframe", Some((1280, 720))).unwrap();
    let gl = window.gl();

    // Renderer
    let scene_center = vec3(0.0, 2.0, 0.0);
    let scene_radius = 6.0;
    let mut pipeline = PhongDeferredPipeline::new(&gl).unwrap();
    let mut camera = CameraControl::new(Camera::new_perspective(&gl, scene_center + scene_radius * vec3(0.6, 0.3, 1.0).normalize(), scene_center, vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), window.viewport().aspect(), 0.1, 1000.0).unwrap());

    Loader::load(&["./examples/assets/suzanne.obj", "./examples/assets/suzanne.mtl"], move |loaded|
    {
        let (mut meshes, mut materials)  = Obj::parse(loaded, "./examples/assets/suzanne.obj").unwrap();
        let cpu_mesh = meshes.remove(0);
        let mut cpu_material = materials.remove(0);
        cpu_material.diffuse_intensity = Some(0.2);
        cpu_material.specular_intensity = Some(0.4);
        cpu_material.specular_power = Some(20.0);
        let model = PhongDeferredMesh::new(&gl, &cpu_mesh, &PhongMaterial::new(&gl, &cpu_material).unwrap()).unwrap();

        let wireframe_material = PhongMaterial {
            name: "wireframe".to_string(),
            diffuse_intensity: 0.8,
            specular_intensity: 0.2,
            specular_power: 5.0,
            color_source: ColorSource::Color(vec4(0.9, 0.2, 0.2, 1.0))
        };
        let edges = PhongDeferredInstancedMesh::new(&gl, &edge_transformations(&cpu_mesh), &CPUMesh::cylinder(0.007, 1.0, 10), &wireframe_material).unwrap();
        let vertices = PhongDeferredInstancedMesh::new(&gl, &vertex_transformations(&cpu_mesh), &CPUMesh::sphere(0.015), &wireframe_material).unwrap();

        let plane = PhongDeferredMesh::new(&gl,
            &CPUMesh {
                positions: vec!(-10000.0, -1.0, 10000.0, 10000.0, -1.0, 10000.0, 0.0, -1.0, -10000.0),
                normals: Some(vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0]),
                ..Default::default()},
            &PhongMaterial {color_source: ColorSource::Color(vec4(1.0, 1.0, 1.0, 1.0)),
                diffuse_intensity: 0.2,
                specular_intensity: 0.4,
                specular_power: 20.0, ..Default::default()}
        ).unwrap();

        let mut spot_light0 = SpotLight::new(&gl, 0.6, &vec3(1.0, 1.0, 1.0), &vec3(5.0, 7.0, 5.0),
                                             &vec3(-1.0, -1.0, -1.0), 25.0, 0.1, 0.001, 0.0001).unwrap();
        let mut spot_light1 = SpotLight::new(&gl, 0.6, &vec3(1.0, 1.0, 1.0), &vec3(-5.0, 7.0, 5.0),
                                             &vec3(1.0, -1.0, -1.0), 25.0, 0.1, 0.001, 0.0001).unwrap();
        let mut spot_light2 = SpotLight::new(&gl, 0.6, &vec3(1.0, 1.0, 1.0), &vec3(-5.0, 7.0, -5.0),
                                             &vec3(1.0, -1.0, 1.0), 25.0, 0.1, 0.001, 0.0001).unwrap();
        let mut spot_light3 = SpotLight::new(&gl, 0.6, &vec3(1.0, 1.0, 1.0), &vec3(5.0, 7.0, -5.0),
                                             &vec3(-1.0, -1.0, 1.0), 25.0, 0.1, 0.001, 0.0001).unwrap();

        let render_scene = |viewport: Viewport, camera: &Camera| {
            let transformation = Mat4::from_translation(vec3(0.0, 2.0, 0.0));
            let render_states = RenderStates {depth_test: DepthTestType::LessOrEqual, cull: CullType::Back, ..Default::default()};
            model.render_depth(render_states, viewport, &transformation, camera)?;
            edges.render_depth(render_states, viewport, &transformation, camera)?;
            vertices.render_depth(render_states, viewport, &transformation, camera)?;
            Ok(())
        };
        spot_light0.generate_shadow_map(50.0, 512, &render_scene).unwrap();
        spot_light1.generate_shadow_map(50.0, 512, &render_scene).unwrap();
        spot_light2.generate_shadow_map(50.0, 512, &render_scene).unwrap();
        spot_light3.generate_shadow_map(50.0, 512, &render_scene).unwrap();

        // main loop
        let mut rotating = false;
        window.render_loop(move |frame_input|
        {
            let mut redraw = frame_input.first_frame;
            redraw |= camera.set_aspect(frame_input.viewport.aspect()).unwrap();

            for event in frame_input.events.iter() {
                match event {
                    Event::MouseClick { state, button, .. } => {
                        rotating = *button == MouseButton::Left && *state == State::Pressed;
                    },
                    Event::MouseMotion { delta, .. } => {
                        if rotating {
                            camera.rotate_around_up(delta.0 as f32, delta.1 as f32).unwrap();
                            redraw = true;
                        }
                    },
                    Event::MouseWheel { delta, .. } => {
                        camera.zoom(delta.1 as f32).unwrap();
                        redraw = true;
                    },
                    _ => {}
                }
            }

            if redraw {
                // Geometry pass
                pipeline.geometry_pass(frame_input.viewport.width, frame_input.viewport.height, || {
                    let transformation = Mat4::from_translation(vec3(0.0, 2.0, 0.0));
                    let render_states = RenderStates {depth_test: DepthTestType::LessOrEqual, cull: CullType::Back, ..Default::default()};
                    model.render_geometry(render_states, frame_input.viewport, &transformation, &camera)?;
                    edges.render_geometry(render_states, frame_input.viewport, &transformation, &camera)?;
                    vertices.render_geometry(render_states, frame_input.viewport, &transformation, &camera)?;
                    plane.render_geometry(render_states, frame_input.viewport, &Mat4::identity(), &camera)?;
                    Ok(())
                }).unwrap();

                // Light pass
                Screen::write(&gl, &ClearState::default(), ||
                {
                    pipeline.light_pass(frame_input.viewport, &camera, None, &[], &[&spot_light0, &spot_light1, &spot_light2, &spot_light3], &[])?;
                    Ok(())
                }).unwrap();
            }

            if args.len() > 1 {
                // To automatically generate screenshots of the examples, can safely be ignored.
                FrameOutput {screenshot: Some(args[1].clone()), exit: true, ..Default::default()}
            } else {
                FrameOutput {swap_buffers: redraw, ..Default::default()}
            }
        }).unwrap();
    });
}

fn vertex_transformations(cpu_mesh: &CPUMesh) -> Vec<Mat4>
{
    let mut iter = cpu_mesh.positions.iter();
    let mut vertex_transformations = Vec::new();
    while let Some(v) = iter.next() {
        vertex_transformations.push(Mat4::from_translation(vec3(*v, *iter.next().unwrap(), *iter.next().unwrap())));
    }
    vertex_transformations
}

fn edge_transformations(cpu_mesh: &CPUMesh) -> Vec<Mat4>
{
    let mut edge_transformations = std::collections::HashMap::new();
    let indices = cpu_mesh.indices.as_ref().unwrap();
    for f in 0..indices.len()/3 {
        let mut fun = |i1, i2| {
            let p1 = vec3(cpu_mesh.positions[i1 * 3], cpu_mesh.positions[i1 * 3 + 1], cpu_mesh.positions[i1 * 3 + 2]);
            let p2 = vec3(cpu_mesh.positions[i2 * 3], cpu_mesh.positions[i2 * 3 + 1], cpu_mesh.positions[i2 * 3 + 2]);
            let scale = Mat4::from_nonuniform_scale((p1-p2).magnitude(), 1.0, 1.0);
            let rotation = rotation_matrix_from_dir_to_dir(vec3(1.0, 0.0, 0.0), (p2-p1).normalize());
            let translation = Mat4::from_translation(p1);
            let key = if i1 < i2 {(i1, i2)} else {(i2, i1)};
            edge_transformations.insert(key, translation * rotation * scale);
        };
        let i1 = indices[3*f] as usize;
        let i2 = indices[3*f+1] as usize;
        let i3 = indices[3*f+2] as usize;
        fun(i1, i2);
        fun(i1, i3);
        fun(i2, i3);
    }
    edge_transformations.drain().map(|(_, v)| v).collect::<Vec<Mat4>>()
}