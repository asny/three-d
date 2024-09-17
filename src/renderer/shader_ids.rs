//!
//! Each shader is allocated a unique ID within its category so that they can be cached between frames.
//! If you implement custom shaders, ensure to give them an ID not already allocated here.
//! Reductions of the public use ID ranges will be considered a breaking change and result in the appropriate version increment.
//! The allocation of internal use IDs should be considered unstable.
//!

use crate::texture::{ColorTexture, DepthTexture};

use open_enum::open_enum;

// TODO: Utilizing the open_enum crate, convert all ID usage to these types (breaking change for externally implemented shaders)
// NOTE: It may be possible to eventually also create a proc macro that automatically allocates IDs based on the width of their subfields

///
/// Recursive macro to assemble multiple booleans into a single bitfield (as a variable width int literal)
///
macro_rules! bitfield_bit {
    ($field:ident << $shift:expr) => {
        // Base case, unwrapping a boolean into an int literal and shifting by the computed amount
        ((if $field { 1 } else { 0 }) << $shift)
    };

    ($field:ident, $($fields:ident),+ << $shift:expr) => {
        // Recursive case, breaking off the first bit and adding one to the shift of the remainder
        bitfield_bit!($field << $shift)
            | bitfield_bit!($($fields),+ << $shift + 1)
    };
}

///
/// Generates a function accepting bit parameters, ordering from least significant bit to most significant bit
/// bitfield_bit abstracts out the process of converting a tuple of bools into a single int literal
///
macro_rules! enum_bitfield {
    ($base_name:ident, $name:ident($($field:ident),+ $(,)?)) => {
        #[allow(non_snake_case)]
        #[inline]
        pub(crate) fn $name($($field: bool),+) -> Self {
            Self(
                Self::$base_name.0
                    | bitfield_bit!($($field),+ << 0)
            )
        }
    };
}

///
/// Generates a function accepting effect parameters, which self-describe which bits they set
///
macro_rules! enum_effectfield {
    ($base_name:ident, $name:ident($($texture:ident: Option<$textureType:ty>),+ $(,)?)) => {
        // Effect field function taking optional effects, which default to zero if unset
        #[allow(non_snake_case)]
        #[inline]
        pub(crate) fn $name($($texture: Option<$textureType>),*) -> Self {
            Self(
                Self::$base_name.0
                $(  | $texture.map(|t| t.id()).unwrap_or(0))*
            )
        }
    };

    ($base_name:ident, $name:ident(Option<...Default>)) => {
        // Default for effect field is to accept both color and depth options
        enum_effectfield!($base_name, $name(color_texture: Option<ColorTexture>, depth_texture: Option<DepthTexture>));
    };

    ($base_name:ident, $name:ident($($texture:ident: $textureType:ty),+ $(,)?)) => {
        // Effect field function taking non-optional effects, which have no default
        #[allow(non_snake_case)]
        #[inline]
        pub(crate) fn $name($($texture: $textureType),*) -> Self {
            Self(
                Self::$base_name.0
                $(  | $texture.id())*
            )
        }
    };

    ($base_name:ident, $name:ident(...Default)) => {
        // Default for effect field is to accept both color and depth options
        enum_effectfield!($base_name, $name(color_texture: ColorTexture, depth_texture: DepthTexture));
    };
}

///
/// ID Space for geometry shaders
/// IDs 0x0000 through 0x7FFF are reserved for public use
///
#[open_enum]
#[repr(u16)]
pub(crate) enum GeometryId {
    Screen = 0x8000,
    Skybox = 0x8001,
    TerrainPatchBase = 0x8002, // To 0x8003
    Sprites = 0x8004,
    WaterPatch = 0x8005,
    MeshBase = 0x8010,           // To 0x801F
    ParticleSystemBase = 0x8040, // To 0x807F
    InstancedMeshBase = 0x8080,  // To 0x80FF
}

impl GeometryId {
    enum_bitfield!(TerrainPatchBase, TerrainPatch(normal_tangent));
    enum_bitfield!(MeshBase, Mesh(normal, tangents, uv, color));
    enum_bitfield!(
        ParticleSystemBase,
        ParticleSystem(normal, tangents, uv, color, instance_color, instance_uv)
    );
    enum_bitfield!(
        InstancedMeshBase,
        InstancedMesh(
            normal,
            tangents,
            uv,
            color,
            instance_color,
            instance_transformation,
            instance_uv,
        )
    );
}

///
/// ID Space for effect and material shaders
/// IDs 0x0000 through 0x4FFF are reserved for public use
///
#[open_enum]
#[repr(u16)]
pub(crate) enum EffectMaterialId {
    LightingPassEffectBase = 0x5000, // To 0x503F
    WaterEffectBase = 0x5800,        // To 0x583F
    CopyEffectBase = 0x6000,         // To 0x603F
    ScreenEffectBase = 0x6800,       // To 0x683F
    FogEffectBase = 0x7000,          // To 0x703F
    FxaaEffectBase = 0x7800,         // To 0x7838 (has holes)

    ColorMaterialBase = 0x8000, // To 0x8001
    DepthMaterial = 0x8002,
    PositionMaterial = 0x8003,
    SkyboxMaterial = 0x8004,
    UVMaterial = 0x8005,
    NormalMaterialBase = 0x8006, // To 0x8007
    IsosurfaceMaterial = 0x800C,
    ImpostersMaterial = 0x800D,
    BrdfMaterial = 0x800E,
    IrradianceMaterial = 0x800F,
    ORMMaterialBase = 0x8010,              // To 0x8013
    PhysicalMaterialBase = 0x8020,         // To 0x803F
    DeferredPhysicalMaterialBase = 0x8040, // To 0x807F
    PrefilterMaterial = 0x8080,
}

impl EffectMaterialId {
    enum_effectfield!(LightingPassEffectBase, LightingPassEffect(...Default));
    enum_effectfield!(WaterEffectBase, WaterEffect(...Default));
    enum_effectfield!(CopyEffectBase, CopyEffect(Option<...Default>));
    enum_effectfield!(ScreenEffectBase, ScreenEffect(Option<...Default>));
    enum_effectfield!(FogEffectBase, FogEffect(...Default));
    enum_effectfield!(FxaaEffectBase, FxaaEffect(color_texture: ColorTexture));

    enum_bitfield!(ColorMaterialBase, ColorMaterial(texture));
    enum_bitfield!(NormalMaterialBase, NormalMaterial(normal_texture));
    enum_bitfield!(
        ORMMaterialBase,
        ORMMaterial(metallic_roughness_texture, occlusion_texture)
    );
    enum_bitfield!(
        PhysicalMaterialBase,
        PhysicalMaterial(
            albedo_texture,
            metallic_roughness_texture,
            occlusion_texture,
            normal_texture,
            emissive_texture,
        )
    );
    enum_bitfield!(
        DeferredPhysicalMaterialBase,
        DeferredPhysicalMaterial(
            albedo_texture,
            metallic_roughness_texture,
            occlusion_texture,
            normal_texture,
            emissive_texture,
            alpha_cutout,
        )
    );
}

///
/// ID space for lighting shaders
/// IDs 0x00 through 0x7F are reserved for public use
///
#[open_enum]
#[repr(u8)]
pub(crate) enum LightId {
    AmbientLightBase = 0x80,     // To 0x81
    DirectionalLightBase = 0x82, // To 0x83
    PointLight = 0x84,
    SpotLightBase = 0x86, // To 0x87
}

impl LightId {
    enum_bitfield!(AmbientLightBase, AmbientLight(environment));
    enum_bitfield!(DirectionalLightBase, DirectionalLight(shadow_texture));
    enum_bitfield!(SpotLightBase, SpotLight(shadow_texture));
}
