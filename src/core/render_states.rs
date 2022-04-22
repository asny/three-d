//!
//! Definitions of the input state needed for any draw call.
//!

use crate::core::*;

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

impl RenderStates {
    pub(in crate::core) fn set(&self, context: &Context) -> ThreeDResult<()> {
        self.cull.set(context);
        self.write_mask.set(context);
        self.depth_test.set(context, self.write_mask.depth);
        self.blend.set(context);
        context.error_check()
    }
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

impl Cull {
    pub(in crate::core) fn set(&self, context: &Context) {
        unsafe {
            match self {
                Cull::None => {
                    context.disable(crate::context::CULL_FACE);
                }
                Cull::Back => {
                    context.enable(crate::context::CULL_FACE);
                    context.cull_face(crate::context::BACK);
                }
                Cull::Front => {
                    context.enable(crate::context::CULL_FACE);
                    context.cull_face(crate::context::FRONT);
                }
                Cull::FrontAndBack => {
                    context.enable(crate::context::CULL_FACE);
                    context.cull_face(crate::context::FRONT_AND_BACK);
                }
            }
        }
    }
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

impl DepthTest {
    fn set(&self, context: &Context, depth_mask: bool) {
        set_depth(context, Some(*self), depth_mask);
    }
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

    pub(in crate::core) fn set(&self, context: &Context) {
        unsafe {
            context.color_mask(self.red, self.green, self.blue, self.alpha);
            set_depth(context, None, self.depth);
        }
    }
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

    pub(in crate::core) fn set(&self, context: &Context) {
        unsafe {
            if let Blend::Enabled {
                source_rgb_multiplier,
                source_alpha_multiplier,
                destination_rgb_multiplier,
                destination_alpha_multiplier,
                rgb_equation,
                alpha_equation,
            } = *self
            {
                context.enable(crate::context::BLEND);
                context.blend_func_separate(
                    Self::blend_const_from_multiplier(source_rgb_multiplier),
                    Self::blend_const_from_multiplier(destination_rgb_multiplier),
                    Self::blend_const_from_multiplier(source_alpha_multiplier),
                    Self::blend_const_from_multiplier(destination_alpha_multiplier),
                );
                context.blend_equation_separate(
                    Self::blend_const_from_equation(rgb_equation),
                    Self::blend_const_from_equation(alpha_equation),
                );
            } else {
                context.disable(crate::context::BLEND);
            }
        }
    }

    fn blend_const_from_multiplier(multiplier: BlendMultiplierType) -> u32 {
        match multiplier {
            BlendMultiplierType::Zero => crate::context::ZERO,
            BlendMultiplierType::One => crate::context::ONE,
            BlendMultiplierType::SrcColor => crate::context::SRC_COLOR,
            BlendMultiplierType::OneMinusSrcColor => crate::context::ONE_MINUS_SRC_COLOR,
            BlendMultiplierType::DstColor => crate::context::DST_COLOR,
            BlendMultiplierType::OneMinusDstColor => crate::context::ONE_MINUS_DST_COLOR,
            BlendMultiplierType::SrcAlpha => crate::context::SRC_ALPHA,
            BlendMultiplierType::OneMinusSrcAlpha => crate::context::ONE_MINUS_SRC_ALPHA,
            BlendMultiplierType::DstAlpha => crate::context::DST_ALPHA,
            BlendMultiplierType::OneMinusDstAlpha => crate::context::ONE_MINUS_DST_ALPHA,
            BlendMultiplierType::SrcAlphaSaturate => crate::context::SRC_ALPHA_SATURATE,
        }
    }

    fn blend_const_from_equation(equation: BlendEquationType) -> u32 {
        match equation {
            BlendEquationType::Add => crate::context::FUNC_ADD,
            BlendEquationType::Subtract => crate::context::FUNC_SUBTRACT,
            BlendEquationType::ReverseSubtract => crate::context::FUNC_REVERSE_SUBTRACT,
            BlendEquationType::Min => crate::context::MIN,
            BlendEquationType::Max => crate::context::MAX,
        }
    }
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

fn set_depth(context: &Context, depth_test: Option<DepthTest>, depth_mask: bool) {
    unsafe {
        if depth_mask == false && depth_test == Some(DepthTest::Always) {
            context.disable(crate::context::DEPTH_TEST);
        } else {
            context.enable(crate::context::DEPTH_TEST);
            context.depth_mask(depth_mask);
            if let Some(depth_test) = depth_test {
                match depth_test {
                    DepthTest::Never => {
                        context.depth_func(crate::context::NEVER);
                    }
                    DepthTest::Less => {
                        context.depth_func(crate::context::LESS);
                    }
                    DepthTest::Equal => {
                        context.depth_func(crate::context::EQUAL);
                    }
                    DepthTest::LessOrEqual => {
                        context.depth_func(crate::context::LEQUAL);
                    }
                    DepthTest::Greater => {
                        context.depth_func(crate::context::GREATER);
                    }
                    DepthTest::NotEqual => {
                        context.depth_func(crate::context::NOTEQUAL);
                    }
                    DepthTest::GreaterOrEqual => {
                        context.depth_func(crate::context::GEQUAL);
                    }
                    DepthTest::Always => {
                        context.depth_func(crate::context::ALWAYS);
                    }
                }
            }
        }
    }
}
