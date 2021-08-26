use crate::core::*;
use crate::renderer::*;

#[derive(Clone)]
pub struct Model2D {
    model: Model,
    context: Context,
}

impl Model2D {
    pub fn new(context: &Context, cpu_mesh: &CPUMesh) -> Result<Self> {
        let model = Model::new(context, cpu_mesh)?;
        unsafe {
            COUNT += 1;
        }
        Ok(Self {
            model,
            context: context.clone(),
        })
    }

    ///
    /// Returns the local to world transformation of this mesh.
    ///
    pub fn transformation(&self) -> &Mat4 {
        self.model.transformation()
    }

    ///
    /// Set the local to world transformation of this mesh.
    ///
    pub fn set_transformation(&mut self, transformation: Mat4) {
        self.model.set_transformation(transformation);
    }

    pub fn render_with_color(&self, color: Color, viewport: Viewport) -> Result<()> {
        self.model
            .render_with_color(color, self.camera2d(viewport)?)
    }

    pub fn render_with_texture(&self, texture: &impl Texture, viewport: Viewport) -> Result<()> {
        self.model
            .render_with_texture(texture, self.camera2d(viewport)?)
    }

    fn uniform_buffer(&self) -> Result<UniformBuffer> {
        let mut uniform_buffer = UniformBuffer::new(&self.context, &[16, 16, 16, 3, 1])?;
        uniform_buffer.update(0, &Mat4::identity().to_slice())?;
        uniform_buffer.update(1, &Mat4::identity().to_slice())?;
        uniform_buffer.update(2, &Mat4::identity().to_slice())?;
        uniform_buffer.update(3, &vec3(0.0, 0.0, 0.0).to_slice())?;
        Ok(uniform_buffer)
    }

    fn camera2d(&self, viewport: Viewport) -> Result<&Camera> {
        unsafe {
            if let Some(ref mut camera) = CAMERA2D {
                camera.set_viewport(viewport)?;
                camera.set_orthographic_projection(viewport.height as f32, 0.0, 10.0)?;
                camera.set_view(
                    vec3(
                        viewport.width as f32 * 0.5,
                        viewport.height as f32 * 0.5,
                        -1.0,
                    ),
                    vec3(
                        viewport.width as f32 * 0.5,
                        viewport.height as f32 * 0.5,
                        0.0,
                    ),
                    vec3(0.0, -1.0, 0.0),
                )?;
                Ok(camera)
            } else {
                CAMERA2D = Some(Camera::new_orthographic(
                    &self.context,
                    viewport,
                    vec3(
                        viewport.width as f32 * 0.5,
                        viewport.height as f32 * 0.5,
                        -1.0,
                    ),
                    vec3(
                        viewport.width as f32 * 0.5,
                        viewport.height as f32 * 0.5,
                        0.0,
                    ),
                    vec3(0.0, -1.0, 0.0),
                    viewport.height as f32,
                    0.0,
                    10.0,
                )?);
                Ok(CAMERA2D.as_ref().unwrap())
            }
        }
    }
}

impl Drop for Model2D {
    fn drop(&mut self) {
        unsafe {
            COUNT -= 1;
            if COUNT == 0 {
                CAMERA2D = None;
            }
        }
    }
}

static mut COUNT: u32 = 0;
static mut CAMERA2D: Option<Camera> = None;
