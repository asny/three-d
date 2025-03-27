use crate::core::*;
use crate::renderer::*;
use std::sync::Arc;

///
/// An illusion of a sky.
///
pub struct Skybox {
    context: Context,
    vertex_buffer: VertexBuffer<Vec3>,
    material: SkyboxMaterial,
}

impl Skybox {
    ///
    /// Creates a new skybox with the given [CpuTexture]s placed at the indicated sides of the skybox.
    /// All of the cpu textures must contain data with the same [TextureDataType].
    ///
    pub fn new(
        context: &Context,
        right: &CpuTexture,
        left: &CpuTexture,
        top: &CpuTexture,
        bottom: &CpuTexture,
        front: &CpuTexture,
        back: &CpuTexture,
    ) -> Self {
        let convert = |cpu_texture: &CpuTexture| match &cpu_texture.data {
            TextureData::RgbU8(_) | TextureData::RgbaU8(_) => {
                let mut cpu_texture = cpu_texture.clone();
                cpu_texture.data.to_linear_srgb();
                Some(cpu_texture)
            }
            _ => None,
        };
        Self::new_with_texture(
            context,
            Arc::new(TextureCubeMap::new(
                context,
                convert(right).as_ref().unwrap_or(right),
                convert(left).as_ref().unwrap_or(left),
                convert(top).as_ref().unwrap_or(top),
                convert(bottom).as_ref().unwrap_or(bottom),
                convert(front).as_ref().unwrap_or(front),
                convert(back).as_ref().unwrap_or(back),
            )),
        )
    }

    ///
    /// Creates a new skybox with a cube texture generated from the equirectangular texture given as input.
    ///
    pub fn new_from_equirectangular(context: &Context, cpu_texture: &CpuTexture) -> Self {
        let texture = match cpu_texture.data {
            TextureData::RgbaU8(_) | TextureData::RgbU8(_) => {
                let mut cpu_texture = cpu_texture.clone();
                cpu_texture.data.to_linear_srgb();
                TextureCubeMap::new_from_equirectangular::<u8>(context, &cpu_texture)
            }
            TextureData::RgU8(_) | TextureData::RU8(_) => {
                TextureCubeMap::new_from_equirectangular::<u8>(context, cpu_texture)
            }
            TextureData::RgbaF16(_)
            | TextureData::RgbF16(_)
            | TextureData::RgF16(_)
            | TextureData::RF16(_) => {
                TextureCubeMap::new_from_equirectangular::<f16>(context, cpu_texture)
            }
            TextureData::RgbaF32(_)
            | TextureData::RgbF32(_)
            | TextureData::RgF32(_)
            | TextureData::RF32(_) => {
                TextureCubeMap::new_from_equirectangular::<f32>(context, cpu_texture)
            }
        };

        Self::new_with_texture(context, Arc::new(texture))
    }

    ///
    /// Creates a new skybox with the given [TextureCubeMap].
    /// The colors are assumed to be in linear sRGB (`RgbU8`), linear sRGB with an alpha channel (`RgbaU8`) or HDR color space.
    ///
    pub fn new_with_texture(context: &Context, texture: Arc<TextureCubeMap>) -> Self {
        let vertex_buffer = VertexBuffer::new_with_data(
            context,
            &[
                vec3(1.0, 1.0, -1.0),
                vec3(-1.0, 1.0, -1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(-1.0, 1.0, 1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(-1.0, 1.0, -1.0),
                vec3(-1.0, -1.0, -1.0),
                vec3(1.0, -1.0, -1.0),
                vec3(1.0, -1.0, 1.0),
                vec3(1.0, -1.0, 1.0),
                vec3(-1.0, -1.0, 1.0),
                vec3(-1.0, -1.0, -1.0),
                vec3(1.0, -1.0, -1.0),
                vec3(-1.0, -1.0, -1.0),
                vec3(1.0, 1.0, -1.0),
                vec3(-1.0, 1.0, -1.0),
                vec3(1.0, 1.0, -1.0),
                vec3(-1.0, -1.0, -1.0),
                vec3(-1.0, -1.0, 1.0),
                vec3(1.0, -1.0, 1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(-1.0, 1.0, 1.0),
                vec3(-1.0, -1.0, 1.0),
                vec3(1.0, -1.0, -1.0),
                vec3(1.0, 1.0, -1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(1.0, -1.0, 1.0),
                vec3(1.0, -1.0, -1.0),
                vec3(-1.0, 1.0, -1.0),
                vec3(-1.0, -1.0, -1.0),
                vec3(-1.0, 1.0, 1.0),
                vec3(-1.0, -1.0, 1.0),
                vec3(-1.0, 1.0, 1.0),
                vec3(-1.0, -1.0, -1.0),
            ],
        );

        Skybox {
            context: context.clone(),
            vertex_buffer,
            material: SkyboxMaterial { texture },
        }
    }

    ///
    /// Returns a reference to the cube map texture
    ///
    pub fn texture(&self) -> &Arc<TextureCubeMap> {
        &self.material.texture
    }
}

impl<'a> IntoIterator for &'a Skybox {
    type Item = &'a dyn Object;
    type IntoIter = std::iter::Once<&'a dyn Object>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl Geometry for Skybox {
    fn draw(&self, viewer: &dyn Viewer, program: &Program, render_states: RenderStates) {
        program.use_uniform("view", viewer.view());
        program.use_uniform("projection", viewer.projection());
        program.use_vertex_attribute("position", &self.vertex_buffer);
        program.draw_arrays(render_states, viewer.viewport(), 36);
    }

    fn vertex_shader_source(&self) -> String {
        include_str!("shaders/skybox.vert").to_owned()
    }

    fn id(&self) -> GeometryId {
        GeometryId::Skybox
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::INFINITE
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
    ) {
        if let Err(e) = render_with_material(&self.context, viewer, &self, material, lights) {
            panic!("{}", e.to_string());
        }
    }

    fn render_with_effect(
        &self,
        material: &dyn Effect,
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        if let Err(e) = render_with_effect(
            &self.context,
            viewer,
            self,
            material,
            lights,
            color_texture,
            depth_texture,
        ) {
            panic!("{}", e.to_string());
        }
    }
}

impl Object for Skybox {
    fn render(&self, viewer: &dyn Viewer, lights: &[&dyn Light]) {
        if let Err(e) = render_with_material(&self.context, viewer, self, &self.material, lights) {
            panic!("{}", e.to_string());
        }
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}
