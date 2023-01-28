//!
//! A collection of materials implementing the [Material] and/or [PostMaterial] trait.
//!
//! A material together with a [geometry] can be rendered directly (using [Geometry::render_with_material] or [Geometry::render_with_post_material]).
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

use std::sync::Arc;

///
/// A reference to a 2D texture and a texture transformation.
///
#[derive(Clone)]
pub struct Texture2DRef {
    /// A reference to the texture.
    pub texture: Arc<Texture2D>,
    /// A transformation applied to the uv coordinates before reading a texel value at those uv coordinates.
    /// This is primarily used in relation to texture atlasing.
    pub transformation: Mat3,
}

impl std::ops::Deref for Texture2DRef {
    type Target = Texture2D;
    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}

impl std::convert::From<Arc<Texture2D>> for Texture2DRef {
    fn from(texture: Arc<Texture2D>) -> Self {
        Self {
            texture,
            transformation: Mat3::identity(),
        }
    }
}

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
/// The possible attributes that a material needs to be able to calculate the color of a fragment.
/// The attributes are provided by a [geometry], ie. calculated in the vertex shader and then sent to the fragment shader,
/// for all the attributes that return true from [Material::requires_attribute] or [PostMaterial::requires_attribute].
/// So to use an attribute for a material, implement the [Material::requires_attribute] or [PostMaterial::requires_attribute] to return true for the attribute
/// and add the relevant shader code to the fragment shader source. The relevant shader code is documented for each enum type.
///
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug)]
pub enum MaterialAttribute {
    /// Position in world space: `in vec3 pos;`
    Position,
    /// Normal: `in vec3 nor;`,
    Normal,
    /// Tangent and bitangent: `in vec3 tang; in vec3 bitang;`
    Tangents,
    /// UV coordinates: `in vec2 uvs;`
    UvCoordinates,
    /// Color: `in vec4 col;`
    Color,
}

///
/// Describes the set of attributes provided by a [geometry] and possibly consumed by a [Material], ie. calculated in the vertex shader and then sent to the fragment shader.
/// To use an attribute in a fragment shader add the relevant shader code to the fragment shader source (documented for each attribute).
///
#[derive(Clone, Copy, Debug)]
pub struct FragmentAttributes {
    /// Position in world space: `in vec3 pos;`
    pub position: bool,
    /// Normal: `in vec3 nor;`,
    pub normal: bool,
    /// Tangent and bitangent: `in vec3 tang; in vec3 bitang;`
    pub tangents: bool,
    /// UV coordinates: `in vec2 uvs;`
    pub uv: bool,
    /// Color: `in vec4 col;`
    pub color: bool,
}

impl FragmentAttributes {
    /// All attributes
    pub const ALL: Self = Self {
        position: true,
        normal: true,
        tangents: true,
        uv: true,
        color: true,
    };
    /// No attributes
    pub const NONE: Self = Self {
        position: false,
        normal: false,
        tangents: false,
        uv: false,
        color: false,
    };

    ///
    /// Returns an error if this do not contain all of the given attributes.
    ///
    pub fn ensure_contains_all(&self, required: FragmentAttributes) -> Result<(), RendererError> {
        if required.position && !self.position {
            Err(RendererError::MissingFragmentAttribute(
                "position".to_owned(),
            ))?;
        }
        if required.normal && !self.normal {
            Err(RendererError::MissingFragmentAttribute("normal".to_owned()))?;
        }
        if required.tangents && !self.tangents {
            Err(RendererError::MissingFragmentAttribute(
                "tangents".to_owned(),
            ))?;
        }
        if required.uv && !self.uv {
            Err(RendererError::MissingFragmentAttribute(
                "uv coordinates".to_owned(),
            ))?;
        }
        if required.color && !self.color {
            Err(RendererError::MissingFragmentAttribute("color".to_owned()))?;
        }
        Ok(())
    }
}

pub struct FragmentShader {
    pub source: String,
    pub attributes: FragmentAttributes,
}

///
/// Represents a material that, together with a [geometry], can be rendered using [Geometry::render_with_material].
/// Alternatively, a geometry and a material can be combined in a [Gm],
/// thereby creating an [Object] which can be used in a render call, for example [RenderTarget::render].
///
pub trait Material {
    ///
    /// Takes a [FragmentAttributes] struct describing which attributes could be provided by the [geometry] and a list of lights.
    /// Returns a [FragmentShader], ie. the fragment shader source for this material
    /// and a [FragmentAttributes] struct that describes which fragment attributes are required for rendering with this material.
    /// Returns an error if this material requires a fragment attribute and the [geometry] does not provide it.
    ///
    fn fragment_shader_source(
        &self,
        provided_attributes: FragmentAttributes,
        lights: &[&dyn Light],
    ) -> Result<FragmentShader, RendererError>;

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
    fn fragment_shader_source(
        &self,
        provided_attributes: FragmentAttributes,
        lights: &[&dyn Light],
    ) -> Result<FragmentShader, RendererError> {
        (*self).fragment_shader_source(provided_attributes, lights)
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
    fn fragment_shader_source(
        &self,
        provided_attributes: FragmentAttributes,
        lights: &[&dyn Light],
    ) -> Result<FragmentShader, RendererError> {
        (**self).fragment_shader_source(provided_attributes, lights)
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
    fn fragment_shader_source(
        &self,
        provided_attributes: FragmentAttributes,
        lights: &[&dyn Light],
    ) -> Result<FragmentShader, RendererError> {
        self.as_ref()
            .fragment_shader_source(provided_attributes, lights)
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
    fn fragment_shader_source(
        &self,
        provided_attributes: FragmentAttributes,
        lights: &[&dyn Light],
    ) -> Result<FragmentShader, RendererError> {
        self.as_ref()
            .fragment_shader_source(provided_attributes, lights)
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
    fn fragment_shader_source(
        &self,
        provided_attributes: FragmentAttributes,
        lights: &[&dyn Light],
    ) -> Result<FragmentShader, RendererError> {
        self.as_ref()
            .fragment_shader_source(provided_attributes, lights)
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
    fn fragment_shader_source(
        &self,
        provided_attributes: FragmentAttributes,
        lights: &[&dyn Light],
    ) -> Result<FragmentShader, RendererError> {
        self.borrow()
            .fragment_shader_source(provided_attributes, lights)
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
    fn fragment_shader_source(
        &self,
        provided_attributes: FragmentAttributes,
        lights: &[&dyn Light],
    ) -> Result<FragmentShader, RendererError> {
        self.read()
            .unwrap()
            .fragment_shader_source(provided_attributes, lights)
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

impl ColorTexture<'_> {
    ///
    /// Returns the fragment shader source for using this texture in a shader.
    ///
    pub fn fragment_shader_source(&self) -> String {
        match self {
            Self::Single(_) => "
                uniform sampler2D colorMap;
                vec4 sample_color(vec2 uv)
                {
                    return texture(colorMap, uv);
                }"
            .to_owned(),
            Self::Array { .. } => "
                uniform sampler2DArray colorMap;
                uniform int colorLayers[4];
                vec4 sample_color(vec2 uv)
                {
                    return texture(colorMap, vec3(uv, colorLayers[0]));
                }
                vec4 sample_layer(vec2 uv, int index)
                {
                    return texture(colorMap, vec3(uv, colorLayers[index]));
                }"
            .to_owned(),
            Self::CubeMap { .. } => unimplemented!(),
        }
    }

    ///
    /// Sends the uniform data needed for this texture to the fragment shader.
    ///
    pub fn use_uniforms(&self, program: &Program) {
        match self {
            Self::Single(texture) => program.use_texture("colorMap", texture),
            Self::Array { texture, layers } => {
                let mut la: [i32; 4] = [0; 4];
                layers
                    .iter()
                    .enumerate()
                    .for_each(|(i, l)| la[i] = *l as i32);
                program.use_uniform_array("colorLayers", &la);
                program.use_texture_array("colorMap", texture);
            }
            Self::CubeMap { .. } => unimplemented!(),
        }
    }

    ///
    /// The resolution of the underlying texture if there is any.
    ///
    pub fn resolution(&self) -> (u32, u32) {
        match self {
            Self::Single(texture) => (texture.width(), texture.height()),
            Self::Array { texture, .. } => (texture.width(), texture.height()),
            Self::CubeMap { texture, .. } => (texture.width(), texture.height()),
        }
    }
}

impl DepthTexture<'_> {
    ///
    /// Returns the fragment shader source for using this texture in a shader.
    ///
    pub fn fragment_shader_source(&self) -> String {
        match self {
            Self::Single { .. } => "
                uniform sampler2D depthMap;
                float sample_depth(vec2 uv)
                {
                    return texture(depthMap, uv).x;
                }"
            .to_owned(),
            Self::Array { .. } => "
                uniform sampler2DArray depthMap;
                uniform int depthLayer;
                float sample_depth(vec2 uv)
                {
                    return texture(depthMap, vec3(uv, depthLayer)).x;
                }"
            .to_owned(),
            Self::CubeMap { .. } => {
                unimplemented!()
            }
        }
    }

    ///
    /// Sends the uniform data needed for this texture to the fragment shader.
    ///
    pub fn use_uniforms(&self, program: &Program) {
        match self {
            Self::Single(texture) => program.use_depth_texture("depthMap", texture),
            Self::Array { texture, layer } => {
                program.use_uniform("depthLayer", layer);
                program.use_depth_texture_array("depthMap", texture);
            }
            Self::CubeMap { .. } => unimplemented!(),
        }
    }

    ///
    /// The resolution of the underlying texture if there is any.
    ///
    pub fn resolution(&self) -> (u32, u32) {
        match self {
            Self::Single(texture) => (texture.width(), texture.height()),
            Self::Array { texture, .. } => (texture.width(), texture.height()),
            Self::CubeMap { texture, .. } => (texture.width(), texture.height()),
        }
    }
}

///
/// Similar to [Material], the difference is that this type of material needs the rendered color texture and/or depth texture of the scene to be applied.
/// Therefore this type of material is always applied one at a time and after the scene has been rendered with the regular [Material].
///
pub trait PostMaterial {
    ///
    /// Returns the fragment shader source for this material. Should output the final fragment color.
    /// Can take attributes from the vertex shader as input, see [PostMaterial::requires_attribute] and [MaterialAttribute] for more information.
    ///
    fn fragment_shader_source(
        &self,
        provided_attributes: FragmentAttributes,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> Result<FragmentShader, RendererError>;

    ///
    /// Returns whether this material requires the given [MaterialAttribute] to render.
    /// The render call will panic if the material requires an attribute and the [geometry] does not provide it.
    /// See [MaterialAttribute] for more information.
    ///

    ///
    /// Sends the uniform data needed for this material to the fragment shader.
    ///
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    );

    ///
    /// Returns the render states needed to render with this material.
    ///
    fn render_states(&self) -> RenderStates;

    ///
    /// Returns the type of material.
    ///
    fn material_type(&self) -> MaterialType;
}

impl<T: PostMaterial + ?Sized> PostMaterial for &T {
    fn fragment_shader_source(
        &self,
        provided_attributes: FragmentAttributes,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> Result<FragmentShader, RendererError> {
        (*self).fragment_shader_source(provided_attributes, lights, color_texture, depth_texture)
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        (*self).use_uniforms(program, camera, lights, color_texture, depth_texture)
    }
    fn render_states(&self) -> RenderStates {
        (*self).render_states()
    }

    fn material_type(&self) -> MaterialType {
        (*self).material_type()
    }
}

impl<T: PostMaterial + ?Sized> PostMaterial for &mut T {
    fn fragment_shader_source(
        &self,
        provided_attributes: FragmentAttributes,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> Result<FragmentShader, RendererError> {
        (**self).fragment_shader_source(provided_attributes, lights, color_texture, depth_texture)
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        (**self).use_uniforms(program, camera, lights, color_texture, depth_texture)
    }
    fn render_states(&self) -> RenderStates {
        (**self).render_states()
    }

    fn material_type(&self) -> MaterialType {
        (**self).material_type()
    }
}

impl<T: PostMaterial> PostMaterial for Box<T> {
    fn fragment_shader_source(
        &self,
        provided_attributes: FragmentAttributes,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> Result<FragmentShader, RendererError> {
        self.as_ref().fragment_shader_source(
            provided_attributes,
            lights,
            color_texture,
            depth_texture,
        )
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        self.as_ref()
            .use_uniforms(program, camera, lights, color_texture, depth_texture)
    }
    fn render_states(&self) -> RenderStates {
        self.as_ref().render_states()
    }

    fn material_type(&self) -> MaterialType {
        self.as_ref().material_type()
    }
}

impl<T: PostMaterial> PostMaterial for std::rc::Rc<T> {
    fn fragment_shader_source(
        &self,
        provided_attributes: FragmentAttributes,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> Result<FragmentShader, RendererError> {
        self.as_ref().fragment_shader_source(
            provided_attributes,
            lights,
            color_texture,
            depth_texture,
        )
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        self.as_ref()
            .use_uniforms(program, camera, lights, color_texture, depth_texture)
    }
    fn render_states(&self) -> RenderStates {
        self.as_ref().render_states()
    }

    fn material_type(&self) -> MaterialType {
        self.as_ref().material_type()
    }
}

impl<T: PostMaterial> PostMaterial for std::sync::Arc<T> {
    fn fragment_shader_source(
        &self,
        provided_attributes: FragmentAttributes,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> Result<FragmentShader, RendererError> {
        self.as_ref().fragment_shader_source(
            provided_attributes,
            lights,
            color_texture,
            depth_texture,
        )
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        self.as_ref()
            .use_uniforms(program, camera, lights, color_texture, depth_texture)
    }
    fn render_states(&self) -> RenderStates {
        self.as_ref().render_states()
    }

    fn material_type(&self) -> MaterialType {
        self.as_ref().material_type()
    }
}

impl<T: PostMaterial> PostMaterial for std::cell::RefCell<T> {
    fn fragment_shader_source(
        &self,
        provided_attributes: FragmentAttributes,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> Result<FragmentShader, RendererError> {
        self.borrow().fragment_shader_source(
            provided_attributes,
            lights,
            color_texture,
            depth_texture,
        )
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        self.borrow()
            .use_uniforms(program, camera, lights, color_texture, depth_texture)
    }
    fn render_states(&self) -> RenderStates {
        self.borrow().render_states()
    }

    fn material_type(&self) -> MaterialType {
        self.borrow().material_type()
    }
}

impl<T: PostMaterial> PostMaterial for std::sync::RwLock<T> {
    fn fragment_shader_source(
        &self,
        provided_attributes: FragmentAttributes,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> Result<FragmentShader, RendererError> {
        self.read().unwrap().fragment_shader_source(
            provided_attributes,
            lights,
            color_texture,
            depth_texture,
        )
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        self.read()
            .unwrap()
            .use_uniforms(program, camera, lights, color_texture, depth_texture)
    }
    fn render_states(&self) -> RenderStates {
        self.read().unwrap().render_states()
    }

    fn material_type(&self) -> MaterialType {
        self.read().unwrap().material_type()
    }
}
