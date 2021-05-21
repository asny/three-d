use crate::camera::*;
use crate::core::*;
use crate::definition::*;
use crate::math::*;

///
/// An illusion of a sky.
///
pub struct Skybox {
    program: Program,
    vertex_buffer: VertexBuffer,
    texture: TextureCubeMap,
}

impl Skybox {
    pub fn new<T: TextureDataType>(
        context: &Context,
        cpu_texture: &mut CPUTexture<T>,
    ) -> Result<Skybox, Error> {
        cpu_texture.wrap_t = Wrapping::ClampToEdge;
        cpu_texture.wrap_s = Wrapping::ClampToEdge;
        cpu_texture.wrap_r = Wrapping::ClampToEdge;
        cpu_texture.mip_map_filter = None;
        let texture = TextureCubeMap::new(&context, cpu_texture)?;
        Self::new_with_texture(context, texture)
    }

    pub fn new_with_texture(context: &Context, texture: TextureCubeMap) -> Result<Skybox, Error> {
        let program = Program::from_source(
            context,
            include_str!("shaders/skybox.vert"),
            include_str!("shaders/skybox.frag"),
        )?;

        let vertex_buffer = VertexBuffer::new_with_static(context, &get_positions())?;

        Ok(Skybox {
            program,
            vertex_buffer,
            texture,
        })
    }

    ///
    /// Render the skybox.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    pub fn render(&self, viewport: Viewport, camera: &Camera) -> Result<(), Error> {
        let render_states = RenderStates {
            depth_test: DepthTestType::LessOrEqual,
            ..Default::default()
        };

        self.program.use_texture_cube(&self.texture, "texture0")?;
        self.program
            .use_uniform_block(camera.uniform_buffer(), "Camera");

        self.program
            .use_attribute_vec3(&self.vertex_buffer, "position")?;

        self.program
            .draw_arrays(render_states, CullType::Front, viewport, 36);
        Ok(())
    }

    pub fn texture(&self) -> &TextureCubeMap {
        &self.texture
    }
}

fn get_positions() -> Vec<f32> {
    vec![
        1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0,
        -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0,
        -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0,
        1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0,
        -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0,
    ]
}
