use crate::*;
use lyon::math::Point;
use lyon::path::Path;
use lyon::tessellation::*;
use std::collections::HashMap;
use swash::zeno::{Command, PathData};
use swash::{scale::ScaleContext, shape::ShapeContext, FontRef, GlyphId};

///
/// Options for text layout.
///
#[derive(Debug, Clone, Copy)]
pub struct TextLayoutOptions {
    ///
    /// The line height multiplier where 1.0 corresponds to the maximum height of the font.
    /// Default is 1.2.
    ///
    pub line_height: f32,
}

impl Default for TextLayoutOptions {
    fn default() -> Self {
        Self { line_height: 1.2 }
    }
}

///
/// A utility struct for generating a [CpuMesh] from a text string with a given font.
///
pub struct TextGenerator<'a> {
    map: HashMap<GlyphId, CpuMesh>,
    font: FontRef<'a>,
    max_height: f32,
    size: f32,
}

impl<'a> TextGenerator<'a> {
    ///
    /// Creates a new TextGenerator with the given font and size in pixels per em.
    /// The index indicates the specific font in a font collection. Set to 0 if unsure.
    ///
    pub fn new(font_bytes: &'a [u8], font_index: u32, size: f32) -> Result<Self, RendererError> {
        let font = FontRef::from_index(font_bytes, font_index as usize)
            .ok_or(RendererError::MissingFont(font_index))?;
        let mut context = ScaleContext::new();
        let mut scaler = context.builder(font).size(size).build();
        let mut map = HashMap::new();
        let mut max_height: f32 = 0.0;
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
                if tessellator
                    .tessellate_path(
                        &path,
                        &options,
                        &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                            vec3(vertex.position().x, vertex.position().y, 0.0)
                        }),
                    )
                    .is_ok()
                {
                    let mesh = CpuMesh {
                        positions: Positions::F32(geometry.vertices),
                        indices: Indices::U32(geometry.indices),
                        ..Default::default()
                    };
                    max_height = max_height.max(mesh.compute_aabb().size().y);
                    map.insert(id, mesh);
                }
            }
        });
        Ok(Self {
            map,
            font,
            max_height,
            size,
        })
    }

    ///
    /// Generates a [CpuMesh] from the given text string.
    ///
    pub fn generate(&self, text: &str, options: TextLayoutOptions) -> CpuMesh {
        let mut shape_context = ShapeContext::new();
        let mut shaper = shape_context.builder(self.font).size(self.size).build();
        let mut positions = Vec::new();
        let mut indices = Vec::new();
        let mut position = vec2(0.0, 0.0);

        shaper.add_str(text);
        shaper.shape_with(|cluster| {
            let t = text.get(cluster.source.to_range());
            if matches!(t, Some("\n")) {
                // Move to the next line
                position.y -= self.max_height * options.line_height;
                position.x = 0.0;
            }
            for glyph in cluster.glyphs {
                let mesh = self.map.get(&glyph.id).unwrap();

                let index_offset = positions.len() as u32;
                let Indices::U32(mesh_indices) = &mesh.indices else {
                    unreachable!()
                };
                indices.extend(mesh_indices.iter().map(|i| i + index_offset));

                let position_offset = (position + vec2(glyph.x, glyph.y)).extend(0.0);
                let Positions::F32(mesh_positions) = &mesh.positions else {
                    unreachable!()
                };
                positions.extend(mesh_positions.iter().map(|p| p + position_offset));
            }
            position.x += cluster.advance();
        });

        CpuMesh {
            positions: Positions::F32(positions),
            indices: Indices::U32(indices),
            ..Default::default()
        }
    }
}
