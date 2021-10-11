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
        objects: &[(&dyn Geometry, &PhysicalMaterial)],
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<()> {
        let mut lights: Vec<&dyn Light> = Vec::new();
        if let Some(light) = ambient_light {
            lights.push(light)
        }
        for light in directional_lights {
            lights.push(light);
        }
        for light in spot_lights {
            lights.push(light);
        }
        for light in point_lights {
            lights.push(light);
        }
        for (geo, mat) in objects.iter().filter(|(g, _)| g.in_frustum(&camera)) {
            geo.render_forward(
                &LitForwardMaterial {
                    material: mat,
                    lights: &lights,
                },
                camera,
            )?;
        }
        Ok(())
    }

    pub fn render_pass<T: Geometry + Drawable>(
        &self,
        camera: &Camera,
        objects: &[T],
    ) -> Result<()> {
        for object in objects.iter().filter(|o| o.in_frustum(camera)) {
            object.render(camera)?;
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
            geometry.render_forward(&depth_material, camera)?;
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
