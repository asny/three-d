//!
//! A collection of materials implementing the [Material] and/or [PostMaterial] trait.
//!
//! A material together with a [geometry] can be rendered directly (using `render_with_material` or `render_with_post_material`).
//! A [Material] can also be combined into an [object] (see [Gm]) and be used in a render call, for example [RenderTarget::render].
//!

use crate::renderer::*;

pub use three_d_asset::material::{
    GeometryFunction, LightingModel, NormalDistributionFunction, PbrMaterial as CpuMaterial,
};

mod color_material;
#[doc(inline)]
pub use color_material::*;

mod depth_material;
#[doc(inline)]
pub use depth_material::*;

mod normal_material;
#[doc(inline)]
pub use normal_material::*;

mod orm_material;
#[doc(inline)]
pub use orm_material::*;

mod position_material;
#[doc(inline)]
pub use position_material::*;

mod uv_material;
#[doc(inline)]
pub use uv_material::*;

mod water_material;
#[doc(inline)]
pub use water_material::*;

mod physical_material;
#[doc(inline)]
pub use physical_material::*;

mod deferred_physical_material;
#[doc(inline)]
pub use deferred_physical_material::*;

mod skybox_material;
#[doc(inline)]
pub(in crate::renderer) use skybox_material::*;

mod isosurface_material;
#[doc(inline)]
pub use isosurface_material::*;

mod fog_material;
#[doc(inline)]
pub use fog_material::*;

mod fxaa_material;
#[doc(inline)]
pub use fxaa_material::*;

mod copy_material;
#[doc(inline)]
pub use copy_material::*;

///
/// Defines the material type which is needed to render the objects in the correct order.
/// For example, transparent objects need to be rendered back to front, whereas opaque objects need to be rendered front to back.
///
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug)]
pub enum MaterialType {
    /// Forward opaque
    Opaque,
    /// Forward transparent
    Transparent,
    /// Deferred opaque
    Deferred,
}

///
/// Represents a material that, together with a [geometry], can be rendered using [Geometry::render_with_material].
/// Alternatively, a geometry and a material can be combined in a [Gm],
/// thereby creating an [Object] which can be used in a render call, for example [RenderTarget::render].
///
/// The material can use an attribute by adding the folowing to the fragment shader source code.
/// - position (in world space): `in vec3 pos;`
/// - normal: `in vec3 nor;`,
/// - tangent: `in vec3 tang;`
/// - bitangent: `in vec3 bitang;`
/// - uv coordinates: `in vec2 uvs;`
/// - color: `in vec4 col;`
/// The rendering will fail if the material requires one of these attributes and the [geometry] does not provide it.
///
pub trait Material {
    ///
    /// Returns the fragment shader source for this material. Should output the final fragment color.
    ///
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &[&dyn Light]) -> String;

    ///
    /// Sends the uniform data needed for this material to the fragment shader.
    ///
    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &[&dyn Light]);

    ///
    /// Returns the render states needed to render with this material.
    ///
    fn render_states(&self) -> RenderStates;

    ///
    /// Returns the type of material.
    ///
    fn material_type(&self) -> MaterialType;
}

///
/// Implement this for a [Material] that can be created from a [CpuMaterial].
///
pub trait FromCpuMaterial: std::marker::Sized {
    ///
    /// Creates a new material that can be used for rendering from a [CpuMaterial].
    ///
    fn from_cpu_material(context: &Context, cpu_material: &CpuMaterial) -> Self;
}

///
/// Implement this for a [Material] that can be created from a [CpuVoxelGrid].
///
pub trait FromCpuVoxelGrid: std::marker::Sized {
    ///
    /// Creates a new material that can be used for rendering from a [CpuVoxelGrid].
    ///
    fn from_cpu_voxel_grid(context: &Context, cpu_voxel_grid: &CpuVoxelGrid) -> Self;
}

impl<T: Material + ?Sized> Material for &T {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        (*self).fragment_shader_source(use_vertex_colors, lights)
    }
    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &[&dyn Light]) {
        (*self).use_uniforms(program, camera, lights)
    }
    fn render_states(&self) -> RenderStates {
        (*self).render_states()
    }
    fn material_type(&self) -> MaterialType {
        (*self).material_type()
    }
}

impl<T: Material + ?Sized> Material for &mut T {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        (**self).fragment_shader_source(use_vertex_colors, lights)
    }
    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &[&dyn Light]) {
        (**self).use_uniforms(program, camera, lights)
    }
    fn render_states(&self) -> RenderStates {
        (**self).render_states()
    }
    fn material_type(&self) -> MaterialType {
        (**self).material_type()
    }
}

impl<T: Material> Material for Box<T> {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        self.as_ref()
            .fragment_shader_source(use_vertex_colors, lights)
    }
    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &[&dyn Light]) {
        self.as_ref().use_uniforms(program, camera, lights)
    }
    fn render_states(&self) -> RenderStates {
        self.as_ref().render_states()
    }
    fn material_type(&self) -> MaterialType {
        self.as_ref().material_type()
    }
}

impl<T: Material> Material for std::rc::Rc<T> {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        self.as_ref()
            .fragment_shader_source(use_vertex_colors, lights)
    }
    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &[&dyn Light]) {
        self.as_ref().use_uniforms(program, camera, lights)
    }
    fn render_states(&self) -> RenderStates {
        self.as_ref().render_states()
    }
    fn material_type(&self) -> MaterialType {
        self.as_ref().material_type()
    }
}

impl<T: Material> Material for std::sync::Arc<T> {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        self.as_ref()
            .fragment_shader_source(use_vertex_colors, lights)
    }
    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &[&dyn Light]) {
        self.as_ref().use_uniforms(program, camera, lights)
    }
    fn render_states(&self) -> RenderStates {
        self.as_ref().render_states()
    }
    fn material_type(&self) -> MaterialType {
        self.as_ref().material_type()
    }
}

impl<T: Material> Material for std::cell::RefCell<T> {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        self.borrow()
            .fragment_shader_source(use_vertex_colors, lights)
    }
    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &[&dyn Light]) {
        self.borrow().use_uniforms(program, camera, lights)
    }
    fn render_states(&self) -> RenderStates {
        self.borrow().render_states()
    }
    fn material_type(&self) -> MaterialType {
        self.borrow().material_type()
    }
}

impl<T: Material> Material for std::sync::RwLock<T> {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        self.read()
            .unwrap()
            .fragment_shader_source(use_vertex_colors, lights)
    }
    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &[&dyn Light]) {
        self.read().unwrap().use_uniforms(program, camera, lights)
    }
    fn render_states(&self) -> RenderStates {
        self.read().unwrap().render_states()
    }
    fn material_type(&self) -> MaterialType {
        self.read().unwrap().material_type()
    }
}

fn is_transparent(cpu_material: &CpuMaterial) -> bool {
    cpu_material.albedo.a != 255
        || cpu_material
            .albedo_texture
            .as_ref()
            .map(|t| match &t.data {
                TextureData::RgbaU8(data) => data.iter().any(|d| d[3] != 255),
                TextureData::RgbaF16(data) => data.iter().any(|d| d[3] < f16::from_f32(0.99)),
                TextureData::RgbaF32(data) => data.iter().any(|d| d[3] < 0.99),
                _ => false,
            })
            .unwrap_or(false)
}

///
/// A reference to a texture containing colors.
///
#[derive(Clone, Copy)]
pub enum ColorTexture<'a> {
    /// No texture
    None,
    /// A single texture 2D texture
    Single(&'a Texture2D),
    /// An array 2D textures and an index into the array
    Array(&'a Texture2DArray, u32),
}

impl ColorTexture<'_> {
    ///
    /// Returns the fragment shader source for using this texture in a shader.
    ///
    pub fn fragment_shader_source(&self) -> Option<String> {
        match self {
            Self::None => None,
            Self::Single(_) => Some(
                "
                uniform sampler2D colorMap;
                vec4 sample_color(vec2 uv)
                {
                    return texture(colorMap, uv);
                }"
                .to_owned(),
            ),
            Self::Array(_, _) => Some(
                "
                uniform sampler2DArray colorMap;
                uniform int colorLayer;
                vec4 sample_color(vec2 uv)
                {
                    return texture(colorMap, vec3(uv, colorLayer));
                }"
                .to_owned(),
            ),
        }
    }

    ///
    /// Sends the uniform data needed for this texture to the fragment shader.
    ///
    pub fn use_uniforms(&self, program: &Program) {
        match self {
            Self::None => {}
            Self::Single(tex) => program.use_texture("colorMap", tex),
            Self::Array(tex, layer) => {
                program.use_uniform("colorLayer", layer);
                program.use_texture_array("colorMap", tex);
            }
        }
    }

    ///
    /// The resolution of the underlying texture if there is any.
    ///
    pub fn resolution(&self) -> Option<(u32, u32)> {
        match self {
            Self::None => None,
            Self::Single(tex) => Some((tex.width(), tex.height())),
            Self::Array(tex, _) => Some((tex.width(), tex.height())),
        }
    }
}

///
/// A reference to a texture containing depths.
///
#[derive(Clone, Copy)]
pub enum DepthTexture<'a> {
    /// No texture
    None,
    /// A single texture 2D texture
    Single(&'a DepthTargetTexture2D),
    /// An array 2D textures and an index into the array
    Array(&'a DepthTargetTexture2DArray, u32),
}

impl DepthTexture<'_> {
    ///
    /// Returns the fragment shader source for using this texture in a shader.
    ///
    pub fn fragment_shader_source(&self) -> Option<String> {
        match self {
            Self::None => None,
            Self::Single(_) => Some(
                "
                uniform sampler2D depthMap;
                float sample_depth(vec2 uv)
                {
                    return texture(depthMap, uv).x;
                }"
                .to_owned(),
            ),
            Self::Array(_, _) => Some(
                "
                uniform sampler2DArray depthMap;
                uniform int depthLayer;
                float sample_depth(vec2 uv)
                {
                    return texture(depthMap, vec3(uv, depthLayer)).x;
                }"
                .to_owned(),
            ),
        }
    }

    ///
    /// Sends the uniform data needed for this texture to the fragment shader.
    ///
    pub fn use_uniforms(&self, program: &Program) {
        match self {
            Self::None => {}
            Self::Single(tex) => program.use_depth_texture("depthMap", tex),
            Self::Array(tex, layer) => {
                program.use_uniform("depthLayer", layer);
                program.use_depth_texture_array("depthMap", tex);
            }
        }
    }

    ///
    /// The resolution of the underlying texture if there is any.
    ///
    pub fn resolution(&self) -> Option<(u32, u32)> {
        match self {
            Self::None => None,
            Self::Single(tex) => Some((tex.width(), tex.height())),
            Self::Array(tex, _) => Some((tex.width(), tex.height())),
        }
    }
}

///
/// Similar to [Material], the difference is that this type of material needs the rendered color texture and/or depth texture of the scene to be applied.
/// Therefore this type of material is always applied one at a time and after the scene has been rendered with the regular [Material].
/// A typical example is to apply a full screen effect after the scene has been rendered. To do this, apply this material to a [ScreenQuad].
///
pub trait PostMaterial {
    ///
    /// Returns the fragment shader source for this material. Should output the final fragment color.
    ///
    fn fragment_shader_source(
        &self,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) -> String;

    ///
    /// Sends the uniform data needed for this material to the fragment shader.
    ///
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    );

    ///
    /// Returns the render states needed to render with this material.
    ///
    fn render_states(&self) -> RenderStates;
}

impl<T: PostMaterial + ?Sized> PostMaterial for &T {
    fn fragment_shader_source(
        &self,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) -> String {
        (*self).fragment_shader_source(lights, color_texture, depth_texture)
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) {
        (*self).use_uniforms(program, camera, lights, color_texture, depth_texture)
    }
    fn render_states(&self) -> RenderStates {
        (*self).render_states()
    }
}

impl<T: PostMaterial + ?Sized> PostMaterial for &mut T {
    fn fragment_shader_source(
        &self,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) -> String {
        (**self).fragment_shader_source(lights, color_texture, depth_texture)
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) {
        (**self).use_uniforms(program, camera, lights, color_texture, depth_texture)
    }
    fn render_states(&self) -> RenderStates {
        (**self).render_states()
    }
}

impl<T: PostMaterial> PostMaterial for Box<T> {
    fn fragment_shader_source(
        &self,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) -> String {
        self.as_ref()
            .fragment_shader_source(lights, color_texture, depth_texture)
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) {
        self.as_ref()
            .use_uniforms(program, camera, lights, color_texture, depth_texture)
    }
    fn render_states(&self) -> RenderStates {
        self.as_ref().render_states()
    }
}

impl<T: PostMaterial> PostMaterial for std::rc::Rc<T> {
    fn fragment_shader_source(
        &self,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) -> String {
        self.as_ref()
            .fragment_shader_source(lights, color_texture, depth_texture)
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) {
        self.as_ref()
            .use_uniforms(program, camera, lights, color_texture, depth_texture)
    }
    fn render_states(&self) -> RenderStates {
        self.as_ref().render_states()
    }
}

impl<T: PostMaterial> PostMaterial for std::sync::Arc<T> {
    fn fragment_shader_source(
        &self,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) -> String {
        self.as_ref()
            .fragment_shader_source(lights, color_texture, depth_texture)
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) {
        self.as_ref()
            .use_uniforms(program, camera, lights, color_texture, depth_texture)
    }
    fn render_states(&self) -> RenderStates {
        self.as_ref().render_states()
    }
}

impl<T: PostMaterial> PostMaterial for std::cell::RefCell<T> {
    fn fragment_shader_source(
        &self,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) -> String {
        self.borrow()
            .fragment_shader_source(lights, color_texture, depth_texture)
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) {
        self.borrow()
            .use_uniforms(program, camera, lights, color_texture, depth_texture)
    }
    fn render_states(&self) -> RenderStates {
        self.borrow().render_states()
    }
}

impl<T: PostMaterial> PostMaterial for std::sync::RwLock<T> {
    fn fragment_shader_source(
        &self,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) -> String {
        self.read()
            .unwrap()
            .fragment_shader_source(lights, color_texture, depth_texture)
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) {
        self.read()
            .unwrap()
            .use_uniforms(program, camera, lights, color_texture, depth_texture)
    }
    fn render_states(&self) -> RenderStates {
        self.read().unwrap().render_states()
    }
}
