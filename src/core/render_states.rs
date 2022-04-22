//!
//! Definitions of the input state needed for any draw call.
//!

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
    pub depth_test: DepthTest,

    ///
    /// Defines which type of blending to use for a render call.
    /// Blending allows combining each color channel of a render call with the color already in the
    /// color channels of the render target.
    /// This is usually used to simulate transparency.
    ///
    pub blend: Blend,

    ///
    /// Defines whether the triangles that are backfacing, frontfacing or both should be skipped in a render call.
    ///
    pub cull: Cull,
}

impl Default for RenderStates {
    fn default() -> Self {
        Self {
            write_mask: WriteMask::default(),
            depth_test: DepthTest::default(),
            blend: Blend::default(),
            cull: Cull::default(),
        }
    }
}

///
/// Defines whether the triangles that are backfacing, frontfacing, both or none should be rendered in a render call.
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cull {
    /// Render both front- and backfacing triangles.
    None,
    /// Render only frontfacing triangles.
    Back,
    /// Render only backfacing triangles.
    Front,
    /// Render nothing.
    FrontAndBack,
}

impl Default for Cull {
    fn default() -> Self {
        Self::None
    }
}

///
/// Determines whether or not a fragment/pixel from the current render call should be discarded
/// when comparing its depth with the depth of the current fragment/pixel.
///
/// **Note:** Depth test is disabled if the render call is not writing to a depth texture.
///
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DepthTest {
    Never,
    Less,
    Equal,
    LessOrEqual,
    Greater,
    NotEqual,
    GreaterOrEqual,
    Always,
}

impl Default for DepthTest {
    fn default() -> Self {
        Self::Less
    }
}

///
/// Defines which channels (red, green, blue, alpha and depth) to write to in a render call.
///
#[allow(missing_docs)]
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
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Blend {
    Enabled {
        source_rgb_multiplier: BlendMultiplierType,
        source_alpha_multiplier: BlendMultiplierType,
        destination_rgb_multiplier: BlendMultiplierType,
        destination_alpha_multiplier: BlendMultiplierType,
        rgb_equation: BlendEquationType,
        alpha_equation: BlendEquationType,
    },
    Disabled,
}

impl Blend {
    ///
    /// Standard OpenGL transparency blending parameters which, for the usual case of being able to see through objects, does not work on web.
    /// In that case, use [Blend::TRANSPARENCY] instead which works the same way on desktop and web.
    ///
    pub const STANDARD_TRANSPARENCY: Self = Self::Enabled {
        source_rgb_multiplier: BlendMultiplierType::SrcAlpha,
        source_alpha_multiplier: BlendMultiplierType::One,
        destination_rgb_multiplier: BlendMultiplierType::OneMinusSrcAlpha,
        destination_alpha_multiplier: BlendMultiplierType::Zero,
        rgb_equation: BlendEquationType::Add,
        alpha_equation: BlendEquationType::Add,
    };

    ///
    /// Transparency blending parameters that works on both desktop and web. For the standard OpenGL parameters, see [Blend::STANDARD_TRANSPARENCY].
    ///
    pub const TRANSPARENCY: Self = Self::Enabled {
        source_rgb_multiplier: BlendMultiplierType::SrcAlpha,
        source_alpha_multiplier: BlendMultiplierType::Zero,
        destination_rgb_multiplier: BlendMultiplierType::OneMinusSrcAlpha,
        destination_alpha_multiplier: BlendMultiplierType::One,
        rgb_equation: BlendEquationType::Add,
        alpha_equation: BlendEquationType::Add,
    };

    ///
    /// Adds the color of the render target with the output color of the render call.
    ///
    pub const ADD: Self = Self::Enabled {
        source_rgb_multiplier: BlendMultiplierType::One,
        source_alpha_multiplier: BlendMultiplierType::One,
        destination_rgb_multiplier: BlendMultiplierType::One,
        destination_alpha_multiplier: BlendMultiplierType::One,
        rgb_equation: BlendEquationType::Add,
        alpha_equation: BlendEquationType::Add,
    };
}

impl Default for Blend {
    fn default() -> Self {
        Self::Disabled
    }
}

///
/// Value multiplied with the source or target color or alpha value in [Blend].
///
#[allow(missing_docs)]
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
/// How the source and target color or alpha value are combined in [Blend].
///
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BlendEquationType {
    Add,
    Subtract,
    ReverseSubtract,
    Max,
    Min,
}
