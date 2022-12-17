// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Normals".to_string(),
        min_size: (512, 512),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    // Model source: https://github.com/KhronosGroup/glTF-Sample-Models/tree/master/2.0/
    let mut loaded = three_d_asset::io::load_async(&[
        "examples/assets/gltf/NormalTangentTest.glb",
        "examples/assets/gltf/NormalTangentMirrorTest.glb",
    ])
    .await
    .unwrap();

    let mut cpu_model: CpuModel = loaded.deserialize("NormalTangentTest.glb").unwrap();
    cpu_model
        .geometries
        .iter_mut()
        .for_each(|m| m.compute_tangents());

    let mut model_with_computed_tangents = Model::<PhysicalMaterial>::new(&context, &cpu_model)
        .unwrap()
        .remove(0);
    model_with_computed_tangents.set_transformation(Mat4::from_translation(vec3(1.4, 1.2, 0.0)));

    let mut instanced_model_with_computed_tangents = InstancedModel::<PhysicalMaterial>::new(
        &context,
        &Instances {
            transformations: vec![Mat4::identity()],
            ..Default::default()
        },
        &cpu_model,
    )
    .unwrap()
    .remove(0);
    instanced_model_with_computed_tangents
        .set_transformation(Mat4::from_translation(vec3(1.4, -1.2, 0.0)));

    let cpu_model: CpuModel = loaded.deserialize("NormalTangentMirrorTest.glb").unwrap();

    let mut model_with_loaded_tangents = Model::<PhysicalMaterial>::new(&context, &cpu_model)
        .unwrap()
        .remove(0);
    model_with_loaded_tangents.set_transformation(Mat4::from_translation(vec3(-1.4, 1.2, 0.0)));
    let mut instanced_model_with_loaded_tangents = InstancedModel::<PhysicalMaterial>::new(
        &context,
        &Instances {
            transformations: vec![Mat4::identity()],
            ..Default::default()
        },
        &cpu_model,
    )
    .unwrap()
    .remove(0);
    instanced_model_with_loaded_tangents
        .set_transformation(Mat4::from_translation(vec3(-1.4, -1.2, 0.0)));

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 7.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE);
    let directional = DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(0.0, -1.0, 0.0));

    // main loop
    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        // Draw
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
            .render(
                &camera,
                model_with_computed_tangents
                    .into_iter()
                    .chain(&model_with_loaded_tangents)
                    .chain(&instanced_model_with_computed_tangents)
                    .chain(&instanced_model_with_loaded_tangents),
                &[&ambient, &directional],
            );
        FrameOutput::default()
    });
}
