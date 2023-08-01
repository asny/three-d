// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Logo!".to_string(),
        max_size: Some((512, 512)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 2.2),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(60.0),
        0.1,
        10.0,
    );

    let mut loaded = if let Ok(loaded) =
        three_d_asset::io::load_async(&["../assets/rust_logo.png"]).await
    {
        loaded
    } else {
        three_d_asset::io::load_async(&[
            "https://asny.github.io/three-d/assets/rust_logo.png",
        ])
        .await
        .expect("failed to download the necessary assets, to enable running this example offline, place the relevant assets in a folder called 'assets' next to the three-d source")
    };
    let image = Texture2D::new(&context, &loaded.deserialize("").unwrap());

    let positions = vec![
        vec3(0.55, -0.4, 0.0),  // bottom right
        vec3(-0.55, -0.4, 0.0), // bottom left
        vec3(0.0, 0.6, 0.0),    // top
    ];
    let colors = vec![
        Srgba::new(255, 0, 0, 255), // bottom right
        Srgba::new(0, 255, 0, 255), // bottom left
        Srgba::new(0, 0, 255, 255), // top
    ];
    let cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        ..Default::default()
    };

    // Construct a model, with a default color material, thereby transferring the mesh data to the GPU
    let model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());

    window.render_loop(move |frame_input| {
        camera.set_viewport(frame_input.viewport);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
            .apply_screen_material(&LogoMaterial { image: &image }, &camera, &[])
            .render(&camera, &model, &[]);

        FrameOutput::default()
    });
}

struct LogoMaterial<'a> {
    image: &'a Texture2D,
}

impl Material for LogoMaterial<'_> {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        include_str!("shader.frag").to_string()
    }

    fn id(&self) -> u16 {
        0b1u16
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            uv: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(&self, program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {
        program.use_texture("image", &self.image);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::COLOR,
            blend: Blend::TRANSPARENCY,
            ..Default::default()
        }
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Transparent
    }
}
