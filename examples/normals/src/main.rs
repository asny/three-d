// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    run(args.get(1).map(|a| std::path::PathBuf::from(a))).await;
}

use three_d::*;

pub async fn run(screenshot: Option<std::path::PathBuf>) {
    let window = Window::new(WindowSettings {
        title: "Normals".to_string(),
        min_size: (512, 512),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    // Model source: https://github.com/KhronosGroup/glTF-Sample-Models/tree/master/2.0/
    let mut loaded = Loader::load_async(&[
        "examples/assets/gltf/NormalTangentTest.glb",
        "examples/assets/gltf/NormalTangentMirrorTest.glb",
    ])
    .await
    .unwrap();

    let (mut cpu_meshes, cpu_materials) = loaded.gltf("NormalTangentTest.glb").unwrap();
    let material = PhysicalMaterial::new(&context, &cpu_materials[0]).unwrap();
    cpu_meshes[0].compute_tangents().unwrap();

    let mut model_with_computed_tangents =
        Model::new_with_material(&context, &cpu_meshes[0], material.clone()).unwrap();
    model_with_computed_tangents.set_transformation(Mat4::from_translation(vec3(1.4, 1.2, 0.0)));

    let mut instanced_model_with_computed_tangents = InstancedModel::new_with_material(
        &context,
        &[ModelInstance::default()],
        &cpu_meshes[0],
        material.clone(),
    )
    .unwrap();
    instanced_model_with_computed_tangents
        .set_transformation(Mat4::from_translation(vec3(1.4, -1.2, 0.0)));

    let (cpu_meshes, cpu_materials) = loaded.gltf("NormalTangentMirrorTest.glb").unwrap();
    let material = PhysicalMaterial::new(&context, &cpu_materials[0]).unwrap();

    let mut model_with_loaded_tangents =
        Model::new_with_material(&context, &cpu_meshes[0], material.clone()).unwrap();
    model_with_loaded_tangents.set_transformation(Mat4::from_translation(vec3(-1.4, 1.2, 0.0)));

    let mut instanced_model_with_loaded_tangents = InstancedModel::new_with_material(
        &context,
        &[ModelInstance::default()],
        &cpu_meshes[0],
        material.clone(),
    )
    .unwrap();
    instanced_model_with_loaded_tangents
        .set_transformation(Mat4::from_translation(vec3(-1.4, -1.2, 0.0)));

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(0.0, 0.0, 7.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE).unwrap();
    let directional =
        DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(0.0, -1.0, 0.0)).unwrap();

    // main loop
    window
        .render_loop(move |mut frame_input| {
            camera.set_viewport(frame_input.viewport).unwrap();
            control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            // Draw
            Screen::write(
                &context,
                ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0),
                || {
                    let lights: [&dyn Light; 2] = [&ambient, &directional];
                    model_with_computed_tangents.render(&camera, &lights)?;
                    model_with_loaded_tangents.render(&camera, &lights)?;
                    instanced_model_with_computed_tangents.render(&camera, &lights)?;
                    instanced_model_with_loaded_tangents.render(&camera, &lights)?;
                    Ok(())
                },
            )
            .unwrap();

            if let Some(ref screenshot) = screenshot {
                // To automatically generate screenshots of the examples, can safely be ignored.
                FrameOutput {
                    screenshot: Some(screenshot.clone()),
                    exit: true,
                    ..Default::default()
                }
            } else {
                FrameOutput::default()
            }
        })
        .unwrap();
}
