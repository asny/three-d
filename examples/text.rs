#[tokio::main]
async fn main() {
    run().await;
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Text!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 15.0, 15.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(60.0),
        0.1,
        1000.0,
    );
    let mut control = FlyControl::new(0.1);

    let axes = Axes::new(&context, 0.1, 1.0);

    let font = Font::default();
    let effect = TextEffect {
        text: 'a',
        size: 40.,
    };

    let texture2d = font.rasterize(effect, &context);

    let material = ColorMaterial {
        color: Color::WHITE,
        texture: Some(std::sync::Arc::new(texture2d).into()),
        ..Default::default()
    };

    let billboards = Sprites::new(&context, &[vec3(-20.0, 0.0, -5.0)], None);

    window.render_loop(move |mut frame_input: FrameInput| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                axes.into_iter().chain(&Gm {
                    geometry: &billboards,
                    material: &material,
                }),
                &[],
            );

        FrameOutput::default()
    });
    // let font = Font::default();
    // let text =
}

// /// A ttf font.
// #[derive(Clone, Debug)]
// pub struct Font {
//     /// A `fontdue` ttf font.
//     pub font: fontdue::Font,
// }

// impl Font {
//     /// Loads a new ttf font from a file.
//     pub fn new(path: &str, scale: f32) -> Font {
//         let mut data = Vec::new();
//         let mut file = File::open(path).unwrap();
//         let _ = file.read_to_end(&mut data).unwrap();

//         let settings = fontdue::FontSettings {
//             scale,
//             ..fontdue::FontSettings::default()
//         };

//         let font = fontdue::Font::from_bytes(data, settings).unwrap();

//         Font { font }
//     }

//     /// Instanciate a default font.
//     pub fn default() -> Font {
//         let data = include_bytes!("assets/Roboto-Regular.ttf") as &[u8];

//         let settings = fontdue::FontSettings {
//             scale: 40.0,
//             ..fontdue::FontSettings::default()
//         };

//         let font = fontdue::Font::from_bytes(data, settings).unwrap();

//         Font { font }
//     }

//     /// The underlying rusttype font.
//     #[inline]
//     pub fn font(&self) -> &fontdue::Font {
//         &self.font
//     }

//     pub fn rasterize(&self, effect: TextEffect, context: &Context) -> Texture2D {
//         let (metrics, bitmap) = self.font.rasterize(effect.text, effect.size);

//         Texture2D::new(
//             context,
//             &CpuTexture {
//                 name: "text".to_string(), // not necessary for text rendering?
//                 data: TextureData::RU8(bitmap),
//                 width: metrics.width as u32,
//                 height: metrics.height as u32,
//                 min_filter: Interpolation::Linear,
//                 mag_filter: Interpolation::Linear,
//                 mip_map_filter: Some(Interpolation::Linear),
//                 wrap_s: Wrapping::ClampToEdge,
//                 wrap_t: Wrapping::ClampToEdge,
//             },
//         )
//     }
// }

// ///
// /// An effect contains a cohesive text to be rendered.
// ///
// #[derive(Clone, Debug)]
// pub struct TextEffect {
//     pub text: char,
//     pub size: f32,
// }
