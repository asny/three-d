use crate::core::*;
use crate::renderer::*;

///
/// Forward render pipeline which can render objects (implementing the [Object] trait).
/// Forward rendering directly draws to the given render target (for example the screen) and is therefore the same as calling [Object::render] directly.
///
pub struct ForwardPipeline {
    context: Context,
}

impl ForwardPipeline {
    ///
    /// Constructor.
    ///
    pub fn new(context: &Context) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
        })
    }

    ///
    /// Render the objects. Also avoids rendering objects outside the camera frustum and render the objects in the order given by [cmp_render_order].
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    ///
    pub fn render_pass(
        &self,
        camera: &Camera,
        objects: &[&dyn Object],
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        render_pass(camera, objects, lights)
    }

    ///
    /// Render the distance from the camera to the objects in each pixel into a depth texture. Also, do not render transparent objects and objects outside the camera frustum.
    /// Must be called in a render target render function, where a depth texture is bound, for example in the callback function of [Screen::write] or [DepthTargetTexture2D::write].
    ///
    pub fn depth_pass(&self, camera: &Camera, objects: &[&dyn Object]) -> ThreeDResult<()> {
        let depth_material = DepthMaterial {
            render_states: RenderStates {
                write_mask: WriteMask::DEPTH,
                ..Default::default()
            },
            ..Default::default()
        };
        for object in objects
            .iter()
            .filter(|o| !o.is_transparent() && camera.in_frustum(&o.aabb()))
        {
            object.render_with_material(&depth_material, camera, &[])?;
        }
        Ok(())
    }

    ///
    /// Creates a new [DepthTargetTexture2D], applies a [ForwardPipeline::depth_pass] and returns the texture.
    ///
    pub fn depth_pass_texture(
        &self,
        camera: &Camera,
        objects: &[&dyn Object],
    ) -> ThreeDResult<DepthTargetTexture2D> {
        let mut depth_texture = DepthTargetTexture2D::new(
            &self.context,
            camera.viewport().width,
            camera.viewport().height,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            DepthFormat::Depth32F,
        )?;
        depth_texture
            .as_depth_target()
            .clear(1.0)?
            .write(|| self.depth_pass(&camera, objects))?;
        Ok(depth_texture)
    }
}
