use crate::*;
use std::f32::consts::PI;

const NO_VIEW_ANGLES: usize  = 8;

pub struct Imposter {
    program: program::Program,
    center_buffer: VertexBuffer,
    rotation_buffer: VertexBuffer,
    positions_buffer: VertexBuffer,
    uvs_buffer: VertexBuffer,
    instance_count: u32,
    texture: Texture2DArray
}

impl Imposter {
    pub fn new<F: Fn(&Camera) -> Result<(), Error>>(gl: &Gl, render: F, aabb: (Vec3, Vec3), max_texture_size: usize) -> Result<Self, Error>
    {
        let (min, max) = aabb;
        let width = f32::sqrt(f32::powi(max.x - min.x, 2) + f32::powi(max.z - min.z, 2));
        let height = max.y - min.y;
        let center = 0.5 * min + 0.5 * max;
        let mut camera = camera::Camera::new_orthographic(gl, center + vec3(0.0, 0.0, -1.0),
                          center, vec3(0.0, 1.0, 0.0), width, height, 4.0*(width+height));

        let texture_width = (max_texture_size as f32 * (width / height).min(1.0)) as usize;
        let texture_height = (max_texture_size as f32 * (height / width).min(1.0)) as usize;
        let texture = Texture2DArray::new(gl, texture_width, texture_height, NO_VIEW_ANGLES,
                Interpolation::Nearest, Interpolation::Nearest, None,
                                                Wrapping::ClampToEdge,Wrapping::ClampToEdge, Format::RGBA8)?;
        let depth_texture = Texture2DArray::new(gl, texture_width, texture_height, NO_VIEW_ANGLES,
                Interpolation::Nearest, Interpolation::Nearest, None,
                                                      Wrapping::ClampToEdge,Wrapping::ClampToEdge, Format::Depth32F)?;

        state::depth_write(&gl, true);
        state::depth_test(&gl, state::DepthTestType::LessOrEqual);
        state::cull(&gl, state::CullType::None);
        state::blend(&gl, state::BlendType::None);

        for i in 0..NO_VIEW_ANGLES {
            let angle = i as f32 * 2.0 * PI / NO_VIEW_ANGLES as f32;
            camera.set_view(center + width * vec3(f32::sin(-angle), 0.0, f32::cos(-angle)),
                            center, vec3(0.0, 1.0, 0.0));
            RenderTarget::write_array(gl, 0, 0, texture_width, texture_height,
                              Some(&vec4(0.0, 0.0, 0.0, 0.0)), Some(1.0),
                              Some(&texture), Some(&depth_texture),
                              1, &|_| { i },
                              i, || {render(&camera)?; Ok(())})?;
        }

        let xmin = center.x - 0.5 * width;
        let xmax = center.x + 0.5 * width;
        let ymin = min.y;
        let ymax = max.y;
        let positions = vec![
            xmin, ymin, 0.0,
            xmax, ymin, 0.0,
            xmax, ymax, 0.0,
            xmax, ymax, 0.0,
            xmin, ymax, 0.0,
            xmin, ymin, 0.0
        ];
        let uvs = vec![
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0
        ];
        let positions_buffer = VertexBuffer::new_with_static_f32(&gl, &positions)?;
        let uvs_buffer = VertexBuffer::new_with_static_f32(&gl, &uvs)?;

        let program = program::Program::from_source(gl,
                                                    include_str!("shaders/imposter.vert"),
                                                    include_str!("shaders/imposter.frag"))?;

        let center_buffer = VertexBuffer::new_with_dynamic_f32(gl, &[])?;
        let rotation_buffer = VertexBuffer::new_with_dynamic_f32(gl, &[])?;

        Ok(Imposter {texture, program, center_buffer, rotation_buffer, positions_buffer, uvs_buffer, instance_count:0 })
    }

    pub fn update_positions(&mut self, positions: &[f32], angles_in_radians: &[f32])
    {
        self.center_buffer.fill_with_dynamic_f32(positions);
        self.rotation_buffer.fill_with_dynamic_f32(angles_in_radians);
        self.instance_count = positions.len() as u32/3;
    }

    pub fn render(&self, camera: &camera::Camera) -> Result<(), Error>
    {
        self.program.add_uniform_int("no_views", &(NO_VIEW_ANGLES as i32))?;
        self.program.use_uniform_block(camera.matrix_buffer(), "Camera");

        self.program.use_texture(&self.texture, "tex")?;

        self.program.use_attribute_vec3_float(&self.positions_buffer, "position")?;
        self.program.use_attribute_vec2_float(&self.uvs_buffer, "uv_coordinate")?;

        self.program.use_attribute_vec3_float_divisor(&self.center_buffer, "center", 1)?;
        self.program.use_attribute_float_divisor(&self.rotation_buffer, "theta", 1)?;
        self.program.draw_arrays_instanced(6, self.instance_count);
        Ok(())
    }
}