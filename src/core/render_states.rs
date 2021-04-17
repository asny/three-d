///
/// A set of render specific states that has to be specified at each render call.
///
#[derive(Debug, Copy, Clone)]
pub struct RenderStates {
    ///
    /// Defines which channels (red, green, blue, alpha and depth) to write to in a render call.
    ///
    pub write_mask: WriteMask,

    ///
    /// Defines the depth test in a render call.
    /// The depth test determines whether or not a fragment from the current render call should be discarded
    /// when comparing its depth with the depth of the current fragment.
    ///
    pub depth_test: DepthTestType,

    ///
    /// Defines which type of blending to use for a render call.
    /// Blending allows combining each color channel of a render call with the color already in the
    /// color channels of the render target.
    /// This is usually used to simulate transparency.
    ///
    pub blend: Option<BlendParameters>,
}

impl Default for RenderStates {
    fn default() -> Self {
        Self {
            write_mask: WriteMask::default(),
            depth_test: DepthTestType::Less,
            blend: None,
        }
    }
}

///
/// Defines whether the triangles that are backfacing, frontfacing or both should be skipped in a render call.
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CullType {
    None,
    Back,
    Front,
    FrontAndBack,
}

///
/// Defines the method for depth test in a render call.
/// Depth test determines whether or not a fragment/pixel from the current render call should be discarded
/// when comparing its depth with the depth of the current fragment/pixel.
/// **Note:** Depth test is disabled if the render call is not writing to a depth texture.
/// This means depth testing does not work when writing to a [ColorTargetTexture2D](crate::ColorTargetTexture2D) or [ColorTargetTexture2DArray](crate::ColorTargetTexture2DArray),
/// but do work when writing to the [Screen](crate::Screen), a [RenderTarget](crate::RenderTarget), [RenderTargetArray](crate::RenderTargetArray),
/// [DepthTargetTexture2D](crate::DepthTargetTexture2D) or [DepthTargetTexture2DArray](crate::DepthTargetTexture2DArray).
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DepthTestType {
    Never,
    Less,
    Equal,
    LessOrEqual,
    Greater,
    NotEqual,
    GreaterOrEqual,
    Always,
}

///
/// Defines which channels (red, green, blue, alpha and depth) to write to in a render call.
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WriteMask {
    pub red: bool,
    pub green: bool,
    pub blue: bool,
    pub alpha: bool,
    pub depth: bool,
}

impl WriteMask {
    ///
    /// Writes to all channels (red, green, blue, alpha and depth).
    ///
    pub const COLOR_AND_DEPTH: Self = Self {
        red: true,
        green: true,
        blue: true,
        alpha: true,
        depth: true,
    };

    ///
    /// Writes to all color channels (red, green, blue and alpha).
    ///
    pub const COLOR: Self = Self {
        red: true,
        green: true,
        blue: true,
        alpha: true,
        depth: false,
    };

    ///
    /// Writes to the depth channel only.
    ///
    pub const DEPTH: Self = Self {
        red: false,
        green: false,
        blue: false,
        alpha: false,
        depth: true,
    };

    ///
    /// Do not write to any channels.
    ///
    pub const NONE: Self = Self {
        red: false,
        green: false,
        blue: false,
        alpha: false,
        depth: false,
    };
}

impl Default for WriteMask {
    fn default() -> Self {
        Self::COLOR_AND_DEPTH
    }
}

///
/// Defines which type of blending to use for a render call.
/// Blending allows combining each color channel of a render call with the color already in the
/// color channels of the render target.
/// This is usually used to simulate transparency.
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BlendParameters {
    pub source_rgb_multiplier: BlendMultiplierType,
    pub source_alpha_multiplier: BlendMultiplierType,
    pub destination_rgb_multiplier: BlendMultiplierType,
    pub destination_alpha_multiplier: BlendMultiplierType,
    pub rgb_equation: BlendEquationType,
    pub alpha_equation: BlendEquationType,
}

impl BlendParameters {
    ///
    /// Usual transparency blending parameters.
    ///
    pub const TRANSPARENCY: Self = Self {
        source_rgb_multiplier: BlendMultiplierType::SrcAlpha,
        source_alpha_multiplier: BlendMultiplierType::One,
        destination_rgb_multiplier: BlendMultiplierType::OneMinusSrcAlpha,
        destination_alpha_multiplier: BlendMultiplierType::Zero,
        rgb_equation: BlendEquationType::Add,
        alpha_equation: BlendEquationType::Add,
    };

    ///
    /// Adds the color of the render target with the output color of the render call.
    ///
    pub const ADD: Self = Self {
        source_rgb_multiplier: BlendMultiplierType::One,
        source_alpha_multiplier: BlendMultiplierType::One,
        destination_rgb_multiplier: BlendMultiplierType::One,
        destination_alpha_multiplier: BlendMultiplierType::One,
        rgb_equation: BlendEquationType::Add,
        alpha_equation: BlendEquationType::Add,
    };
}

impl Default for BlendParameters {
    fn default() -> Self {
        Self::TRANSPARENCY
    }
}

///
/// Value multiplied with the source or target color or alpha value in [blend parameters](crate::BlendParameters).
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BlendMultiplierType {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    SrcAlphaSaturate,
}

///
/// How the source and target color or alpha value are combined in [blend parameters](crate::BlendParameters).
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BlendEquationType {
    Add,
    Subtract,
    ReverseSubtract,
    Max,
    Min,
}
