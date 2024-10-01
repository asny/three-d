use crate::*;
use lyon::math::Point;
use lyon::path::Path;
use lyon::tessellation::*;
use std::collections::HashMap;
use swash::zeno::{Command, PathData};
use swash::{scale::ScaleContext, shape::ShapeContext, GlyphId};

pub use swash::FontRef;

pub struct TextOptions {
    pub size: f32,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self { size: 0.0 }
    }
}

///
/// A utility struct for generating a [CpuMesh] from a text string with a given font.
///
pub struct TextGenerator<'a> {
    map: HashMap<GlyphId, CpuMesh>,
    font: FontRef<'a>,
    line_height: f32,
    options: TextOptions,
}

impl<'a> TextGenerator<'a> {
    ///
    /// Creates a new TextGenerator with the given font and options.
    ///
    pub fn new(font: FontRef<'a>, options: TextOptions) -> Self {
        let mut context = ScaleContext::new();
        let mut scaler = context.builder(font).size(options.size).build();
        let mut map = HashMap::new();
        let mut line_height: f32 = 0.0;
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

                let mesh = CpuMesh {
                    positions: Positions::F32(geometry.vertices),
                    indices: Indices::U32(geometry.indices),
                    ..Default::default()
                };
                line_height = line_height.max(mesh.compute_aabb().size().y);
                map.insert(id, mesh);
            }
        });
        Self {
            map,
            font,
            line_height,
            options,
        }
    }

    ///
    /// Generates a [CpuMesh] from the given text string.
    ///
    pub fn generate(&self, text: &str) -> CpuMesh {
        let mut shape_context = ShapeContext::new();
        let mut shaper = shape_context
            .builder(self.font)
            .size(self.options.size)
            .build();
        let mut positions = Vec::new();
        let mut indices = Vec::new();
        let mut y = 0.0;
        let mut x = 0.0;

        shaper.add_str(text);
        shaper.shape_with(|cluster| {
            let t = text.get(cluster.source.to_range());
            if matches!(t, Some("\n")) {
                // Move to the next line
                y -= self.line_height * 1.2; // Add 20% extra space between lines
                x = 0.0;
            }
            for glyph in cluster.glyphs {
                let mesh = self.map.get(&glyph.id).unwrap();

                let index_offset = positions.len() as u32;
                let Indices::U32(mesh_indices) = &mesh.indices else {
                    unreachable!()
                };
                indices.extend(mesh_indices.iter().map(|i| i + index_offset));

                let position = vec3(x + glyph.x, y + glyph.y, 0.0);
                let Positions::F32(mesh_positions) = &mesh.positions else {
                    unreachable!()
                };
                positions.extend(mesh_positions.iter().map(|p| p + position));
            }
            x += cluster.advance();
        });

        CpuMesh {
            positions: Positions::F32(positions),
            indices: Indices::U32(indices),
            ..Default::default()
        }
    }
}
