use crate::core::*;
use crate::renderer::*;
use std::f32::consts::PI;

const NO_VIEW_ANGLES: u32 = 8;

///
/// A level-of-detail technique to replace rendering high-poly meshes.
/// Should only be used where details cannot be seen, for example when the objects are far away.
/// A mesh is rendered from different angles into a set of textures and the textures are then
/// rendered continuously instead of the high-poly meshes.
///
pub struct Imposters {
    context: Context,
    sprites: Sprites,
    material: ImpostersMaterial,
}

impl Imposters {
    ///
    /// Constructs a new [Imposters] and render the imposter texture from the given objects.
    ///
    pub fn new(
        context: &Context,
        positions: &[f32],
        objects: &[&dyn Object],
        lights: &[&dyn Light],
        max_texture_size: u32,
    ) -> ThreeDResult<Self> {
        let texture = Texture2DArray::<u8>::new_empty(
            context,
            1,
            1,
            NO_VIEW_ANGLES,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Format::RGBA,
        )?;

        let mut imposters = Imposters {
            context: context.clone(),
            sprites: Sprites::new(context, positions)?,
            material: ImpostersMaterial { texture },
        };
        imposters.update_texture(objects, lights, max_texture_size)?;
        Ok(imposters)
    }

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

        if aabb.is_empty() {
            return Ok(());
        }

        let (min, max) = (aabb.min(), aabb.max());
        let width = f32::sqrt(f32::powi(max.x - min.x, 2) + f32::powi(max.z - min.z, 2));
        let height = max.y - min.y;
        let center = 0.5 * min + 0.5 * max;

        self.sprites.set_transformation(
            Mat4::from_translation(center)
                * Mat4::from_nonuniform_scale(0.5 * width, 0.5 * height, 0.0),
        );
        self.material =
            ImpostersMaterial::new(&self.context, aabb, objects, lights, max_texture_size)?;
        Ok(())
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

    fn transformation(&self) -> Mat4 {
        self.sprites.transformation()
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
    pub texture: Texture2DArray<u8>,
}

impl ImpostersMaterial {
    pub fn new(
        context: &Context,
        aabb: AxisAlignedBoundingBox,
        objects: &[&dyn Object],
        lights: &[&dyn Light],
        max_texture_size: u32,
    ) -> ThreeDResult<Self> {
        let (min, max) = (aabb.min(), aabb.max());
        let width = f32::sqrt(f32::powi(max.x - min.x, 2) + f32::powi(max.z - min.z, 2));
        let height = max.y - min.y;
        let texture_width = (max_texture_size as f32 * (width / height).min(1.0)) as u32;
        let texture_height = (max_texture_size as f32 * (height / width).min(1.0)) as u32;
        let viewport = Viewport::new_at_origo(texture_width, texture_height);
        let center = 0.5 * min + 0.5 * max;
        let mut camera = Camera::new_orthographic(
            context,
            viewport,
            center + vec3(0.0, 0.0, -1.0),
            center,
            vec3(0.0, 1.0, 0.0),
            height,
            0.0,
            4.0 * (width + height),
        )?;
        let texture = Texture2DArray::<u8>::new_empty(
            context,
            texture_width,
            texture_height,
            NO_VIEW_ANGLES,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Format::RGBA,
        )?;
        let depth_texture = DepthTargetTexture2DArray::new(
            context,
            texture_width,
            texture_height,
            NO_VIEW_ANGLES,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            DepthFormat::Depth32F,
        )?;
        {
            let render_target = RenderTargetArray::new(context, &texture, &depth_texture)?;
            for i in 0..NO_VIEW_ANGLES {
                let angle = i as f32 * 2.0 * PI / NO_VIEW_ANGLES as f32;
                camera.set_view(
                    center + width * vec3(f32::sin(-angle), 0.0, f32::cos(-angle)),
                    center,
                    vec3(0.0, 1.0, 0.0),
                )?;
                render_target.write(
                    &[i],
                    0,
                    ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0),
                    || {
                        render_pass(&camera, objects, lights)?;
                        Ok(())
                    },
                )?;
            }
        }
        Ok(Self { texture })
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
        program.use_uniform_int("no_views", &(NO_VIEW_ANGLES as i32))?;
        program.use_uniform_block("Camera", camera.uniform_buffer());
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
