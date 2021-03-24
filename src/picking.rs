
use crate::math::*;
use crate::core::*;
use crate::camera::*;

pub trait Pickable {
    fn pick(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        transformation: &Mat4,
        camera: &Camera,
    ) -> Result<(), Error>;
}


pub struct Picker {

}

impl Picker {
    pub fn pick(context: &Context, position: Vec3, target: Vec3, objects: &[(&dyn Pickable, RenderStates, &Mat4)]) -> Result<Vec3, Error>  {
        let viewport = Viewport::new_at_origo(1, 1);
        let dir = (target - position).normalize();
        let up = if dir.dot(vec3(1.0, 0.0, 0.0)).abs() > 0.99 {
            dir.cross(vec3(0.0, 1.0, 0.0))
        } else {
            dir.cross(vec3(1.0, 0.0, 0.0))
        };
        let max_depth = position.distance(target);
        let camera = Camera::new_orthographic(context, position, target, up, 0.1, 0.1, max_depth)?;
        let texture = ColorTargetTexture2D::new(
            context,
            viewport.width,
            viewport.height,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Format::R32F,
        )?;
        let render_target = RenderTarget::new_color(context, &texture)?;
        render_target.write(&ClearState {red: Some(1.0),.. ClearState::none()}, || {
                for (object, render_states, transformation) in objects {
                    let mut state = *render_states;
                    state.write_mask = WriteMask {red: true, ..WriteMask::NONE};
                    state.blend = Some(BlendParameters{
                        source_rgb_multiplier: BlendMultiplierType::One,
                        source_alpha_multiplier: BlendMultiplierType::One,
                        destination_rgb_multiplier: BlendMultiplierType::One,
                        destination_alpha_multiplier: BlendMultiplierType::One,
                        rgb_equation: BlendEquationType::Min,
                        alpha_equation: BlendEquationType::Min,
                    });
                    object.pick(state, viewport, transformation, &camera)?;
                }
                Ok(())
            })?;
        let depth = render_target.read_color_with_f32(viewport)?[0] * max_depth;
        println!("depth: {}", depth);
        Ok(position + dir * depth)
    }
}
