use crate::*;
use lyon::math::Point;
use lyon::path::Path;
use lyon::tessellation::*;
use std::collections::HashMap;
use swash::scale::{ScaleContext, Scaler};
use swash::shape::{ShapeContext, Shaper};
use swash::zeno::{Command, PathData};
pub use swash::FontRef;
use swash::GlyphId;

pub struct TextGenerator<'a> {
    map: HashMap<GlyphId, CpuMesh>,
    font: FontRef<'a>,
}

impl<'a> TextGenerator<'a> {
    pub fn new(font: FontRef<'a>) -> Self {
        let mut context = ScaleContext::new();
        let scaler = context.builder(font).build();
        Self::new_with_scaler(font, scaler)
    }

    pub fn new_with_scaler(font: FontRef<'a>, mut scaler: Scaler<'_>) -> Self {
        let mut map = HashMap::new();
        font.charmap().enumerate(|_, id| {
            if let Some(outline) = scaler.scale_outline(id) {
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

    pub fn generate(&self, text: &str) -> CpuMesh {
        let mut shape_context = ShapeContext::new();
        let shaper = shape_context.builder(self.font).build();
        self.generate_with_shaper(text, shaper)
    }

    pub fn generate_with_shaper(&self, text: &str, mut shaper: Shaper<'_>) -> CpuMesh {
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
