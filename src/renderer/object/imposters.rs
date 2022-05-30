use crate::core::*;
use crate::renderer::*;
use std::f32::consts::PI;

const NO_VIEW_ANGLES: u32 = 8;

///
/// A level-of-detail technique to replace rendering of high-poly meshes.
/// Should only be used where details cannot be seen, for example when the objects are far away.
/// A set of objects are rendered from different angles into a set of textures and the textures are then
/// rendered continuously instead of the expensive objects.
///
pub struct Imposters {
    sprites: Sprites,
    material: ImpostersMaterial,
}

impl Imposters {
    ///
    /// Constructs a new [Imposters] and render the imposter texture from the given objects with the given lights.
    /// The imposters are placed at the given positions.
    ///
    pub fn new(
        context: &Context,
        positions: &[Vec3],
        objects: &[&dyn Object],
        lights: &[&dyn Light],
        max_texture_size: u32,
    ) -> ThreeDResult<Self> {
        let mut aabb = AxisAlignedBoundingBox::EMPTY;
        objects
            .iter()
            .for_each(|o| aabb.expand_with_aabb(&o.aabb()));
        let mut sprites = Sprites::new(context, positions, Some(vec3(0.0, 1.0, 0.0)))?;
        sprites.set_transformation(get_sprite_transform(aabb));
        Ok(Imposters {
            sprites,
            material: ImpostersMaterial::new(context, aabb, objects, lights, max_texture_size)?,
        })
    }

    ///
    /// Set the positions of the imposters.
    ///
    pub fn set_positions(&mut self, positions: &[Vec3]) -> ThreeDResult<()> {
        self.sprites.set_centers(positions)
    }

    ///
    /// Render the imposter texture from the given objects with the given lights.
    /// Use this if you want to update the look of the imposters.
    ///
    pub fn update_texture(
        &mut self,
        objects: &[&dyn Object],
        lights: &[&dyn Light],
        max_texture_size: u32,
    ) -> ThreeDResult<()> {
        let mut aabb = AxisAlignedBoundingBox::EMPTY;
        objects
            .iter()
            .for_each(|o| aabb.expand_with_aabb(&o.aabb()));
        self.sprites.set_transformation(get_sprite_transform(aabb));
        self.material
            .update(aabb, objects, lights, max_texture_size)?;
        Ok(())
    }
}

fn get_sprite_transform(aabb: AxisAlignedBoundingBox) -> Mat4 {
    if aabb.is_empty() {
        Mat4::identity()
    } else {
        let (min, max) = (aabb.min(), aabb.max());
        let width = f32::sqrt(f32::powi(max.x - min.x, 2) + f32::powi(max.z - min.z, 2));
        let height = max.y - min.y;
        let center = 0.5 * min + 0.5 * max;
        Mat4::from_translation(center) * Mat4::from_nonuniform_scale(0.5 * width, 0.5 * height, 0.0)
    }
}

impl Geometry for Imposters {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        self.sprites.render_with_material(material, camera, lights)
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.sprites.aabb()
    }
}

impl Object for Imposters {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) -> ThreeDResult<()> {
        self.render_with_material(&self.material, camera, lights)
    }

    fn is_transparent(&self) -> bool {
        self.material.is_transparent()
    }
}

struct ImpostersMaterial {
    context: Context,
    texture: Texture2DArray,
}

impl ImpostersMaterial {
    pub fn new(
        context: &Context,
        aabb: AxisAlignedBoundingBox,
        objects: &[&dyn Object],
        lights: &[&dyn Light],
        max_texture_size: u32,
    ) -> ThreeDResult<Self> {
        let mut m = Self {
            context: context.clone(),
            texture: Texture2DArray::new_empty::<[u8; 4]>(
                context,
                1,
                1,
                NO_VIEW_ANGLES,
                Interpolation::Nearest,
                Interpolation::Nearest,
                None,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
            )?,
        };
        m.update(aabb, objects, lights, max_texture_size)?;
        Ok(m)
    }
    pub fn update(
        &mut self,
        aabb: AxisAlignedBoundingBox,
        objects: &[&dyn Object],
        lights: &[&dyn Light],
        max_texture_size: u32,
    ) -> ThreeDResult<()> {
        if !aabb.is_empty() {
            let (min, max) = (aabb.min(), aabb.max());
            let width = f32::sqrt(f32::powi(max.x - min.x, 2) + f32::powi(max.z - min.z, 2));
            let height = max.y - min.y;
            let texture_width = (max_texture_size as f32 * (width / height).min(1.0)) as u32;
            let texture_height = (max_texture_size as f32 * (height / width).min(1.0)) as u32;
            let viewport = Viewport::new_at_origo(texture_width, texture_height);
            let center = 0.5 * min + 0.5 * max;
            let mut camera = Camera::new_orthographic(
                &self.context,
                viewport,
                center + vec3(0.0, 0.0, -1.0),
                center,
                vec3(0.0, 1.0, 0.0),
                height,
                0.0,
                4.0 * (width + height),
            )?;
            self.texture = Texture2DArray::new_empty::<[f16; 4]>(
                &self.context,
                texture_width,
                texture_height,
                NO_VIEW_ANGLES,
                Interpolation::Linear,
                Interpolation::Linear,
                None,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
            )?;
            let mut depth_texture = DepthTargetTexture2D::new(
                &self.context,
                texture_width,
                texture_height,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
                DepthFormat::Depth32F,
            )?;
            for i in 0..NO_VIEW_ANGLES {
                let layers = [i];
                let angle = i as f32 * 2.0 * PI / NO_VIEW_ANGLES as f32;
                camera.set_view(
                    center + width * vec3(f32::cos(angle), 0.0, f32::sin(angle)),
                    center,
                    vec3(0.0, 1.0, 0.0),
                )?;
                RenderTarget::new(
                    self.texture.as_color_target(&layers, None),
                    depth_texture.as_depth_target(),
                )?
                .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0))?
                .render(&camera, objects, lights)?;
            }
        }
        Ok(())
    }
}

impl Material for ImpostersMaterial {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, _lights: &[&dyn Light]) -> String {
        format!(
            "{}{}",
            include_str!("../../core/shared.frag"),
            include_str!("shaders/imposter.frag")
        )
    }

    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        _lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        program.use_uniform("no_views", &(NO_VIEW_ANGLES as i32))?;
        program.use_uniform("view", camera.view())?;
        program.use_texture_array("tex", &self.texture)?;
        Ok(())
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            blend: Blend::TRANSPARENCY,
            cull: Cull::Back,
            ..Default::default()
        }
    }
    fn is_transparent(&self) -> bool {
        true
    }
}
