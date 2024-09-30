use std::collections::HashMap;

use swash::*;
use three_d::*;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Text!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );

    let font_data = include_bytes!("GrapeNuts-Regular.ttf");
    let font = FontRef::from_index(font_data, 0).expect("Failed to load font");
    let map = new(font);

    let text = "Hello, World!";

    let models = text
        .char_indices()
        .map(move |(i, c)| {
            let mesh = map.get(&c.into()).unwrap();
            let mut m = Gm::new(Mesh::new(&context, &mesh), ColorMaterial::default());
            m.set_transformation(
                Mat4::from_translation(vec3(-1.25 + 0.2 * i as f32, 0.0, 0.0))
                    * Mat4::from_scale(0.0002),
            );
            m
        })
        .collect::<Vec<_>>();
    window.render_loop(move |frame_input| {
        camera.set_viewport(frame_input.viewport);
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, models.iter(), &[]);
        FrameOutput::default()
    });
}

fn new(font: FontRef) -> HashMap<u32, CpuMesh> {
    use scale::*;
    let mut context = ScaleContext::new();
    let mut scaler = context.builder(font).build();
    let mut map = HashMap::new();

    font.charmap().enumerate(|c, id| {
        if let Some(outline) = scaler.scale_outline(id) {
            use crate::zeno::{Command, PathData};
            use lyon::math::Point;
            use lyon::path::Path;
            use lyon::tessellation::*;

            let mut builder = Path::builder();
            for command in outline.path().commands() {
                match command {
                    Command::MoveTo(p) => {
                        builder.begin(Point::new(p.x, p.y));
                    }
                    Command::LineTo(p) => {
                        builder.line_to(Point::new(p.x, p.y));
                    }
                    Command::CurveTo(p1, p2, p3) => {
                        builder.cubic_bezier_to(
                            Point::new(p1.x, p1.y),
                            Point::new(p2.x, p2.y),
                            Point::new(p3.x, p3.y),
                        );
                    }
                    Command::QuadTo(p1, p2) => {
                        builder.quadratic_bezier_to(Point::new(p1.x, p1.y), Point::new(p2.x, p2.y));
                    }
                    Command::Close => builder.close(),
                }
            }
            let path = builder.build();

            let mut tessellator = FillTessellator::new();
            let mut geometry: VertexBuffers<Vec3, u32> = VertexBuffers::new();
            let options = FillOptions::default();
            tessellator
                .tessellate_path(
                    &path,
                    &options,
                    &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                        vec3(vertex.position().x, vertex.position().y, 0.0)
                    }),
                )
                .unwrap();

            map.insert(
                c,
                CpuMesh {
                    positions: Positions::F32(geometry.vertices),
                    indices: Indices::U32(geometry.indices),
                    ..Default::default()
                },
            );
        }
    });

    map
}
