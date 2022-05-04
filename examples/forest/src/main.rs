// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Forest!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(2800.0, 240.0, 1700.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(60.0),
        0.1,
        10000.0,
    )
    .unwrap();
    let mut control = FlyControl::new(0.1);

    let mut loaded = three_d_io::Loader::load_async(&[
        "examples/assets/Gledista_Triacanthos.obj",
        "examples/assets/Gledista_Triacanthos.mtl",
        "examples/assets/maps/gleditsia_triacanthos_flowers_color.jpg",
        "examples/assets/maps/gleditsia_triacanthos_flowers_mask.jpg",
        "examples/assets/maps/gleditsia_triacanthos_bark_reflect.jpg",
        "examples/assets/maps/gleditsia_triacanthos_bark2_a1.jpg",
        "examples/assets/maps/gleditsia_triacanthos_leaf_color_b1.jpg",
        "examples/assets/maps/gleditsia_triacanthos_leaf_mask.jpg",
    ])
    .await
    .unwrap();
    // Tree
    let (mut meshes, materials) = loaded.obj(".obj").unwrap();
    let mut models = Vec::new();
    for mut mesh in meshes.drain(..) {
        mesh.compute_normals();
        let mut model = Model::new_with_material(
            &context,
            &mesh,
            PhysicalMaterial::new(
                &context,
                &materials
                    .iter()
                    .find(|m| Some(&m.name) == mesh.material_name.as_ref())
                    .unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
        model.material.render_states.cull = Cull::Back;
        models.push(model);
    }

    // Lights
    let ambient = AmbientLight::new(&context, 0.3, Color::WHITE).unwrap();
    let directional =
        DirectionalLight::new(&context, 4.0, Color::WHITE, &vec3(-1.0, -1.0, -1.0)).unwrap();

    // Imposters
    let mut aabb = AxisAlignedBoundingBox::EMPTY;
    models.iter().for_each(|m| {
        aabb.expand_with_aabb(&m.aabb());
    });
    let size = aabb.size();
    let t = 100;
    let mut positions = Vec::new();
    for x in -t..t + 1 {
        for y in -t..t + 1 {
            if x != 0 || y != 0 {
                positions.push(vec3(size.x * x as f32, 0.0, size.y * y as f32));
            }
        }
    }

    let imposters = Imposters::new(
        &context,
        &positions,
        &models.iter().map(|m| m as &dyn Object).collect::<Vec<_>>(),
        &[&ambient, &directional],
        256,
    )
    .unwrap();

    // Plane
    let mut plane = Model::new_with_material(
        &context,
        &CpuMesh {
            positions: Positions::F32(vec![
                vec3(-10000.0, 0.0, 10000.0),
                vec3(10000.0, 0.0, 10000.0),
                vec3(0.0, 0.0, -10000.0),
            ]),
            normals: Some(vec![
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, 1.0, 0.0),
            ]),
            ..Default::default()
        },
        PhysicalMaterial {
            albedo: Color::new_opaque(128, 200, 70),
            metallic: 0.0,
            roughness: 1.0,
            ..Default::default()
        },
    )
    .unwrap();
    plane.material.render_states.cull = Cull::Back;
    models.push(plane);

    // main loop
    window
        .render_loop(move |mut frame_input| {
            let mut redraw = frame_input.first_frame;
            redraw |= camera.set_viewport(frame_input.viewport).unwrap();

            redraw |= control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            if redraw {
                let mut models = models.iter().map(|m| m as &dyn Object).collect::<Vec<_>>();
                models.push(&imposters);
                frame_input
                    .screen()
                    .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                    .unwrap()
                    .render(&camera, &models, &[&ambient, &directional])
                    .unwrap();
            }

            FrameOutput {
                swap_buffers: redraw,
                ..Default::default()
            }
        })
        .unwrap();
}
