
use crate::*;

const VERTEX_SHADER_SOURCE: &str = r#"
    uniform vec2 u_screen_size;
    in vec2 a_pos;
    in vec4 a_srgba; // 0-255 sRGB
    in vec2 a_tc;
    out vec4 v_rgba;
    out vec2 v_tc;

    // 0-1 linear  from  0-255 sRGB
    vec3 linear_from_srgb(vec3 srgb) {
        bvec3 cutoff = lessThan(srgb, vec3(10.31475));
        vec3 lower = srgb / vec3(3294.6);
        vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
        return mix(higher, lower, cutoff);
    }

    vec4 linear_from_srgba(vec4 srgba) {
        return vec4(linear_from_srgb(srgba.rgb), srgba.a / 255.0);
    }

    void main() {
        gl_Position = vec4(
            2.0 * a_pos.x / u_screen_size.x - 1.0,
            1.0 - 2.0 * a_pos.y / u_screen_size.y,
            0.0,
            1.0);
        // egui encodes vertex colors in gamma spaces, so we must decode the colors here:
        v_rgba = linear_from_srgba(a_srgba);
        v_tc = a_tc;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    uniform sampler2D u_sampler;
    in vec4 v_rgba;
    in vec2 v_tc;
    layout (location = 0) out vec4 f_color;

    void main() {
        // The texture sampler is sRGB aware, and glium already expects linear rgba output
        // so no need for any sRGB conversions here:
        f_color = v_rgba * texture(u_sampler, v_tc);
    }
"#;

pub struct Painter {
    context: Context,
    program: Program
}

impl Painter {
    pub fn new(context: &Context) -> Result<Painter, Error> {
        Ok(Painter {
            context: context.clone(),
            program: Program::from_source(context, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?
        })
    }

    /// Main entry-point for painting a frame
    pub fn paint_meshes(
        &self,
        width: usize,
        height: usize,
        pixels_per_point: f32,
        //clear_color: egui::Rgba, // TODO
        cipped_meshes: Vec<egui::ClippedMesh>,
        egui_texture: &egui::Texture) -> Result<(), Error>
    {
        let texture =
            Texture2D::new_with_u8(&self.context, Interpolation::Linear, Interpolation::Linear, None,
                                   Wrapping::ClampToEdge, Wrapping::ClampToEdge,
                                   &Image {bytes: egui_texture.pixels.clone(), width: egui_texture.width as u32, height: egui_texture.height as u32 })?;

        for egui::ClippedMesh(clip_rect, mesh) in cipped_meshes {
            self.paint_mesh(width, height, pixels_per_point, clip_rect, &mesh, &texture)?;
        }
        Ok(())
    }

    #[inline(never)] // Easier profiling
    pub fn paint_mesh(
        &self,
        width: usize,
        height: usize,
        pixels_per_point: f32,
        clip_rect: egui::Rect,
        mesh: &egui::paint::Mesh,
        texture: &Texture2D
    ) -> Result<(), Error> {
        debug_assert!(mesh.is_valid());

        let mut positions = Vec::new();
        let mut colors = Vec::new();
        let mut uvs = Vec::new();
        for v in mesh.vertices.iter() {
            positions.push(v.pos.x);
            positions.push(v.pos.y);
            positions.push(0.0);
            uvs.push(v.uv.x);
            uvs.push(v.uv.y);
            colors.push(v.color.r() as f32);
            colors.push(v.color.g() as f32);
            colors.push(v.color.b() as f32);
            colors.push(v.color.a() as f32);
        }
        let indices: Vec<u32> = mesh.indices.iter().map(|idx| *idx as u32).collect();

        let position_buffer = VertexBuffer::new_with_static_f32(&self.context, &positions)?;
        let uv_buffer = VertexBuffer::new_with_static_f32(&self.context, &uvs)?;
        let color_buffer = VertexBuffer::new_with_static_f32(&self.context, &colors)?;
        let index_buffer = ElementBuffer::new_with_u32(&self.context, &indices)?;

        // Transform clip rect to physical pixels:
        let clip_min_x = pixels_per_point * clip_rect.min.x;
        let clip_min_y = pixels_per_point * clip_rect.min.y;
        let clip_max_x = pixels_per_point * clip_rect.max.x;
        let clip_max_y = pixels_per_point * clip_rect.max.y;

        // Make sure clip rect can fit withing an `u32`:
        let clip_min_x = egui::emath::clamp(clip_min_x, 0.0..=width as f32);
        let clip_min_y = egui::emath::clamp(clip_min_y, 0.0..=height as f32);
        let clip_max_x = egui::emath::clamp(clip_max_x, clip_min_x..=width as f32);
        let clip_max_y = egui::emath::clamp(clip_max_y, clip_min_y..=height as f32);

        let clip_min_x = clip_min_x.round() as i32;
        let clip_min_y = clip_min_y.round() as i32;
        let clip_max_x = clip_max_x.round() as i32;
        let clip_max_y = clip_max_y.round() as i32;

        let viewport = Viewport {x: clip_min_x, y: height as i32 - clip_max_y, width: (clip_max_x - clip_min_x) as usize, height: (clip_max_y - clip_min_y) as usize};

        let render_states = RenderStates { blend: Some(BlendParameters {
            source_rgb_multiplier: BlendMultiplierType::One,
            destination_rgb_multiplier: BlendMultiplierType::OneMinusSrcAlpha,
            source_alpha_multiplier: BlendMultiplierType::OneMinusDstAlpha,
            destination_alpha_multiplier: BlendMultiplierType::One,
            ..Default::default()
        }), depth_test: DepthTestType::Always, ..Default::default()};

        self.program.use_texture(texture, "u_sampler")?;
        self.program.add_uniform_vec2("u_screen_size", &vec2(width as f32, height as f32))?;

        self.program.use_attribute_vec3_float(&position_buffer, "a_pos")?;
        self.program.use_attribute_vec4_float(&color_buffer, "a_srgba")?;
        self.program.use_attribute_vec2_float(&uv_buffer, "a_tc")?;

        self.program.draw_elements(render_states, viewport, &index_buffer);
        Ok(())
    }
}
