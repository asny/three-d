use crate::renderer::*;

///
/// A 2D effect that is applied to the entire screen.
/// Can for example be used for adding an effect on top of a rendered image.
/// The effect is defined by the [PostMaterial] given at construction.
///
pub struct Effect<T: PostMaterial> {
    screen_quad: ScreenQuad,
    /// The [PostMaterial] that defines the effect.
    pub material: T,
}

impl<T: PostMaterial> Effect<T> {
    ///
    /// Creates a new effect defined by the given [PostMaterial].
    ///
    pub fn new(context: &Context, material: T) -> Self {
        Self {
            screen_quad: ScreenQuad::new(context),
            material,
        }
    }

    ///
    /// Get the texture transform applied to the uv coordinates of the effect.
    ///
    pub fn texture_transform(&mut self) -> &Mat3 {
        self.screen_quad.texture_transform()
    }

    ///
    /// Set the texture transform applied to the uv coordinates of the effect.
    ///
    pub fn set_texture_transform(&mut self, texture_transform: Mat3) {
        self.screen_quad.set_texture_transform(texture_transform);
    }

    ///
    /// Applies the calculations defined by the [PostMaterial] given at construction and output it to the current screen/render target.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    pub fn render(
        &self,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        self.screen_quad.render_with_post_material(
            &self.material,
            camera,
            lights,
            color_texture,
            depth_texture,
        )
    }
}
