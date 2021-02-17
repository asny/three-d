use crate::core::*;
use std::f32::consts::PI;

const NO_VIEW_ANGLES: usize  = 8;

pub struct Imposters {
    context: Context,
    program: program::Program,
    center_buffer: VertexBuffer,
    rotation_buffer: VertexBuffer,
    positions_buffer: VertexBuffer,
    uvs_buffer: VertexBuffer,
    instance_count: u32,
    texture: Texture2DArray
}

impl Imposters {
    pub fn new(context: &Context) -> Result<Self, Error>
    {
        let uvs = vec![
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0
        ];
        let positions_buffer = VertexBuffer::new_with_static_f32(&context, &[])?;
        let uvs_buffer = VertexBuffer::new_with_static_f32(&context, &uvs)?;

        let program = program::Program::from_source(context,
                                                    include_str!("shaders/imposter.vert"),
                                                    include_str!("shaders/imposter.frag"))?;

        let center_buffer = VertexBuffer::new_with_dynamic_f32(context, &[])?;
        let rotation_buffer = VertexBuffer::new_with_dynamic_f32(context, &[])?;
        let texture = Texture2DArray::new(context, 1, 1, NO_VIEW_ANGLES,
                Interpolation::Nearest, Interpolation::Nearest, None,
                                                Wrapping::ClampToEdge,Wrapping::ClampToEdge, Format::RGBA8)?;

        Ok(Imposters {context: context.clone(), texture, program, center_buffer, rotation_buffer, positions_buffer, uvs_buffer, instance_count:0 })
    }

    pub fn update_texture<F: Fn(Viewport, &Camera) -> Result<(), Error>>(&mut self, render: F, aabb: (Vec3, Vec3), max_texture_size: usize) -> Result<(), Error>
    {
        let (min, max) = aabb;
        let width = f32::sqrt(f32::powi(max.x - min.x, 2) + f32::powi(max.z - min.z, 2));
        let height = max.y - min.y;
        let center = 0.5 * min + 0.5 * max;
        let mut camera = camera::Camera::new_orthographic(&self.context, center + vec3(0.0, 0.0, -1.0),
                          center, vec3(0.0, 1.0, 0.0), width, height, 4.0*(width+height));

        let texture_width = (max_texture_size as f32 * (width / height).min(1.0)) as usize;
        let texture_height = (max_texture_size as f32 * (height / width).min(1.0)) as usize;
        self.texture = Texture2DArray::new(&self.context, texture_width, texture_height, NO_VIEW_ANGLES,
                Interpolation::Nearest, Interpolation::Nearest, None,
                                                Wrapping::ClampToEdge,Wrapping::ClampToEdge, Format::RGBA8)?;
        let depth_texture = Texture2DArray::new(&self.context, texture_width, texture_height, NO_VIEW_ANGLES,
                Interpolation::Nearest, Interpolation::Nearest, None,
                                                      Wrapping::ClampToEdge,Wrapping::ClampToEdge, Format::Depth32F)?;
        let render_target = RenderTargetArray::new(&self.context,&self.texture, &depth_texture)?;

        for i in 0..NO_VIEW_ANGLES {
            let angle = i as f32 * 2.0 * PI / NO_VIEW_ANGLES as f32;
            camera.set_view(center + width * vec3(f32::sin(-angle), 0.0, f32::cos(-angle)),
                            center, vec3(0.0, 1.0, 0.0));
                              render_target.write(Some(&vec4(0.0, 0.0, 0.0, 0.0)), Some(1.0), &[i],
                              0, || {render(Viewport::new_at_origo(texture_width, texture_height), &camera)?; Ok(())})?;
        }

        let xmin = center.x - 0.5 * width;
        let xmax = center.x + 0.5 * width;
        let ymin = min.y;
        let ymax = max.y;
        self.positions_buffer.fill_with_dynamic_f32(&vec![
            xmin, ymin, 0.0,
            xmax, ymin, 0.0,
            xmax, ymax, 0.0,
            xmax, ymax, 0.0,
            xmin, ymax, 0.0,
            xmin, ymin, 0.0
        ]);
        Ok(())
    }

    pub fn update_positions(&mut self, positions: &[f32], angles_in_radians: &[f32])
    {
        self.center_buffer.fill_with_dynamic_f32(positions);
        self.rotation_buffer.fill_with_dynamic_f32(angles_in_radians);
        self.instance_count = positions.len() as u32/3;
    }

    pub fn render(&self, viewport: Viewport, camera: &camera::Camera) -> Result<(), Error>
    {
        let render_states = RenderStates {
            cull: CullType::Back,
            blend: Some(BlendParameters {
                source_rgb_multiplier: BlendMultiplierType::SrcAlpha,
                source_alpha_multiplier: BlendMultiplierType::SrcAlpha,
                destination_rgb_multiplier: BlendMultiplierType::OneMinusSrcAlpha,
                destination_alpha_multiplier: BlendMultiplierType::OneMinusSrcAlpha,
                rgb_equation: BlendEquationType::Add,
                alpha_equation: BlendEquationType::Add
            }),
            ..Default::default()
        };
        self.program.add_uniform_int("no_views", &(NO_VIEW_ANGLES as i32))?;
        self.program.use_uniform_block(camera.matrix_buffer(), "Camera");

        self.program.use_texture(&self.texture, "tex")?;

        self.program.use_attribute_vec3_float(&self.positions_buffer, "position")?;
        self.program.use_attribute_vec2_float(&self.uvs_buffer, "uv_coordinate")?;

        self.program.use_attribute_vec3_float_divisor(&self.center_buffer, "center", 1)?;
        self.program.use_attribute_float_divisor(&self.rotation_buffer, "theta", 1)?;
        self.program.draw_arrays_instanced(render_states, viewport, 6, self.instance_count);
        Ok(())
    }
}