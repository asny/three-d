
use crate::*;
pub use egui;

pub struct GUI {
    context: Context,
    egui_context: egui::CtxRef,
    program: Program,
    texture_version: u64,
    texture: Option<Texture2D>
}

impl GUI {
    pub fn new(context: &Context) -> Result<Self, Error> {
        Ok(GUI {
            egui_context: egui::CtxRef::default(),
            context: context.clone(),
            texture_version: 0,
            texture: None,
            program: Program::from_source(context, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?
        })
    }

    pub fn render<F: FnOnce(&egui::CtxRef)>(&mut self, frame_input: &FrameInput, callback: F) -> Result<(), Error> {

        /*TODO:for event in frame_input.events {
            match event {
                glutin::event::Event::WindowEvent { event, .. } => {
                    input_to_egui(event, clipboard.as_mut(), &mut input_state, control_flow);
                    display.gl_window().window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
                }
                _ => (),
            }
        };*/

        let pixels_per_point = frame_input.viewport.width / frame_input.window_width;

        let input_state = egui::RawInput {
            scroll_delta: egui::Vec2::ZERO,
            screen_size: Default::default(),
            screen_rect: Some(egui::Rect::from_min_size(
                Default::default(),
                egui::Vec2 {x: frame_input.window_width as f32, y: frame_input.window_height as f32},
            )),
            pixels_per_point: Some(pixels_per_point as f32),
            time: None,
            predicted_dt: 1.0 / 60.0,
            modifiers: egui::Modifiers::default(),
            events: vec![],
        };
        self.egui_context.begin_frame(input_state);
        callback(&self.egui_context);

        let (_, shapes) = self.egui_context.end_frame();
        let clipped_meshes = self.egui_context.tessellate(shapes);

        let egui_texture = self.egui_context.texture();

        if self.texture.is_none() || self.texture_version != egui_texture.version {
            self.texture = Some(Texture2D::new_with_u8(&self.context, Interpolation::Linear, Interpolation::Linear, None,
                                   Wrapping::ClampToEdge, Wrapping::ClampToEdge,
                                   &Image {bytes: egui_texture.pixels.clone(), width: egui_texture.width as u32, height: egui_texture.height as u32 })?);
            self.texture_version = egui_texture.version;
        };

        for egui::ClippedMesh(clip_rect, mesh) in clipped_meshes {
            println!("{:?}", clip_rect);
            println!("{:?}", mesh);
            self.paint_mesh(frame_input.window_width, frame_input.window_height, pixels_per_point, clip_rect, &mesh, self.texture.as_ref().unwrap())?;
        }
        Ok(())
    }

    fn paint_mesh(
        &self,
        width: usize,
        height: usize,
        pixels_per_point: usize,
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
        let clip_min_x = pixels_per_point as f32 * clip_rect.min.x;
        let clip_min_y = pixels_per_point as f32 * clip_rect.min.y;
        let clip_max_x = pixels_per_point as f32 * clip_rect.max.x;
        let clip_max_y = pixels_per_point as f32 * clip_rect.max.y;

        // Make sure clip rect can fit withing an `u32`:
        let clip_min_x = egui::emath::clamp(clip_min_x, 0.0..=(width * pixels_per_point) as f32);
        let clip_min_y = egui::emath::clamp(clip_min_y, 0.0..=(height * pixels_per_point) as f32);
        let clip_max_x = egui::emath::clamp(clip_max_x, clip_min_x..=(width * pixels_per_point) as f32);
        let clip_max_y = egui::emath::clamp(clip_max_y, clip_min_y..=(height * pixels_per_point) as f32);

        let clip_min_x = clip_min_x.round() as i32;
        let clip_min_y = clip_min_y.round() as i32;
        let clip_max_x = clip_max_x.round() as i32;
        let clip_max_y = clip_max_y.round() as i32;

        let viewport = Viewport {x: clip_min_x, y: (height * pixels_per_point) as i32 - clip_max_y, width: (clip_max_x - clip_min_x) as usize, height: (clip_max_y - clip_min_y) as usize};

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

const VERTEX_SHADER_SOURCE: &str = r#"
    uniform vec2 u_screen_size;
    in vec2 a_pos;
    in vec2 a_tc;
    in vec4 a_srgba;
    out vec4 v_rgba;
    out vec2 v_tc;
    // 0-1 linear  from  0-255 sRGB
    vec3 linear_from_srgb(vec3 srgb) {
        bvec3 cutoff = lessThan(srgb, vec3(10.31475));
        vec3 lower = srgb / vec3(3294.6);
        vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
        return mix(higher, lower, vec3(cutoff));
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
    layout (location = 0) out vec4 color;
    // 0-255 sRGB  from  0-1 linear
    vec3 srgb_from_linear(vec3 rgb) {
        bvec3 cutoff = lessThan(rgb, vec3(0.0031308));
        vec3 lower = rgb * vec3(3294.6);
        vec3 higher = vec3(269.025) * pow(rgb, vec3(1.0 / 2.4)) - vec3(14.025);
        return mix(higher, lower, vec3(cutoff));
    }
    vec4 srgba_from_linear(vec4 rgba) {
        return vec4(srgb_from_linear(rgba.rgb), 255.0 * rgba.a);
    }
    void main() {
        // The texture is set up with `SRGB8_ALPHA8`, so no need to decode here!
        vec4 texture_rgba = texture(u_sampler, v_tc);
        /// Multiply vertex color with texture color (in linear space).
        color = v_rgba * texture_rgba;
        // We must gamma-encode again since WebGL doesn't support linear blending in the framebuffer.
        color = srgba_from_linear(v_rgba * texture_rgba) / 255.0;
        // WebGL doesn't support linear blending in the framebuffer,
        // so we apply this hack to at least get a bit closer to the desired blending:
        color.a = pow(color.a, 1.6); // Empiric nonsense
    }
"#;


/*const VERTEX_SHADER_SOURCE: &str = r#"
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

    // 0-255 sRGB  from  0-1 linear
    vec3 srgb_from_linear(vec3 rgb) {
        bvec3 cutoff = lessThan(rgb, vec3(0.0031308));
        vec3 lower = rgb * vec3(3294.6);
        vec3 higher = vec3(269.025) * pow(rgb, vec3(1.0 / 2.4)) - vec3(14.025);
        return mix(higher, lower, vec3(cutoff));
    }
    vec4 srgba_from_linear(vec4 rgba) {
        return vec4(srgb_from_linear(rgba.rgb), 255.0 * rgba.a);
    }

    void main() {
        // The texture sampler is sRGB aware, and glium already expects linear rgba output
        // so no need for any sRGB conversions here:
        f_color = v_rgba * linear_from_srgba(texture(u_sampler, v_tc));
        f_color = srgba_from_linear(f_color);
    }
"#;*/
