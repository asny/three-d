use crate::core::*;
use crate::renderer::*;

///
/// Forward render pipeline which can render objects (implementing the [Drawable] trait).
/// Forward rendering directly draws to the given render target (for example the screen) and is therefore the same as calling [Drawable::render] directly.
///
pub struct ForwardPipeline {
    context: Context,
}

impl ForwardPipeline {
    ///
    /// Constructor.
    ///
    pub fn new(context: &Context) -> Result<Self> {
        Ok(Self {
            context: context.clone(),
        })
    }

    ///
    /// Render the objects with the given surface materials and the given set of lights.
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    ///
    #[deprecated = "Use render_pass instead"]
    pub fn light_pass(
        &self,
        camera: &Camera,
        objects: &[(&dyn ShadedGeometry, &Material)],
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<()> {
        for (geo, mat) in objects.iter().filter(|(g, _)| camera.in_frustum(g.aabb())) {
            geo.render_with_lighting(
                camera,
                mat,
                LightingModel::Blinn,
                ambient_light,
                directional_lights,
                spot_lights,
                point_lights,
            )?;
        }
        Ok(())
    }

    pub fn render_pass<T: Object>(
        &self,
        camera: &Camera,
        objects: &[T],
        lights: &Lights,
    ) -> Result<()> {
        for object in objects.iter().filter(|o| camera.in_frustum(o.aabb())) {
            object.render(camera, lights)?;
        }
        Ok(())
    }

    pub fn depth_pass<T: Shadable>(&self, camera: &Camera, geometries: &[T]) -> Result<()> {
        let depth_material = DepthMaterial {
            render_states: RenderStates {
                write_mask: WriteMask::DEPTH,
                ..Default::default()
            },
            ..Default::default()
        };
        for geometry in geometries {
            geometry.render_forward(&depth_material, camera, &Lights::default())?;
        }
        Ok(())
    }

    pub fn depth_pass_texture<T: Shadable>(
        &self,
        camera: &Camera,
        objects: &[T],
    ) -> Result<DepthTargetTexture2D> {
        let depth_texture = DepthTargetTexture2D::new(
            &self.context,
            camera.viewport().width,
            camera.viewport().height,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            DepthFormat::Depth32F,
        )?;
        depth_texture.write(Some(1.0), || self.depth_pass(&camera, objects))?;
        Ok(depth_texture)
    }
}
