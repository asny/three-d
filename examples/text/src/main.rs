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

    let x = window.viewport().width as f32 * 0.5;
    let mut camera = Camera::new_orthographic(
        window.viewport(),
        vec3(x, 0.0, 2.0),
        vec3(x, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        window.viewport().height as f32,
        0.1,
        10.0,
    );

    let font_data = include_bytes!("GrapeNuts-Regular.ttf");
    let font = FontRef::from_index(font_data, 0).expect("Failed to load font");
    let mut text_generator = TextGenerator::new(font);

    let text = "Hello, World!";

    let text_mesh = text_generator.generate(text);
    let mut mesh = Gm::new(Mesh::new(&context, &text_mesh), ColorMaterial::default());
    mesh.set_transformation(Mat4::from_scale(0.25));

    window.render_loop(move |frame_input| {
        camera.set_viewport(frame_input.viewport);
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, &mesh, &[]);
        FrameOutput::default()
    });
}

struct TextGenerator<'a> {
    map: HashMap<GlyphId, CpuMesh>,
    font: FontRef<'a>,
}

impl<'a> TextGenerator<'a> {
    pub fn new(font: FontRef<'a>) -> Self {
        use scale::*;
        let mut context = ScaleContext::new();
        let mut scaler = context.builder(font).build();
        let mut map = HashMap::new();

        font.charmap().enumerate(|_, id| {
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
                            builder.quadratic_bezier_to(
                                Point::new(p1.x, p1.y),
                                Point::new(p2.x, p2.y),
                            );
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
                    id,
                    CpuMesh {
                        positions: Positions::F32(geometry.vertices),
                        indices: Indices::U32(geometry.indices),
                        ..Default::default()
                    },
                );
            }
        });
        Self { map, font }
    }

    pub fn generate(&mut self, text: &str) -> CpuMesh {
        let mut shape_context = shape::ShapeContext::new();
        let shaper = shape_context
            .builder(self.font)
            .script(text::Script::Latin)
            .build();
        self.generate_with_shaper(text, shaper)
    }

    pub fn generate_with_shaper(
        &mut self,
        text: &str,
        mut shaper: swash::shape::Shaper<'_>,
    ) -> CpuMesh {
        let mut cursor = 0.0;
        let mut positions = Vec::new();
        let mut indices = Vec::new();

        shaper.add_str(&text);
        shaper.shape_with(|cluster| {
            for glyph in cluster.glyphs {
                let mesh = self.map.get(&glyph.id).unwrap();

                let index_offset = positions.len() as u32;
                let Indices::U32(mesh_indices) = &mesh.indices else {
                    unreachable!()
                };
                indices.extend(mesh_indices.iter().map(|i| i + index_offset));

                let position = vec3(cursor + glyph.x, glyph.y, 0.0);
                let Positions::F32(mesh_positions) = &mesh.positions else {
                    unreachable!()
                };
                positions.extend(mesh_positions.iter().map(|p| p + position));
            }
            cursor += cluster.advance();
        });

        CpuMesh {
            positions: Positions::F32(positions),
            indices: Indices::U32(indices),
            ..Default::default()
        }
    }
}
