
#[derive(Debug, Copy, Clone)]
pub struct RenderStates {
    pub color_mask: ColorMask,
    pub depth_mask: bool,
    pub depth_test: DepthTestType,
    pub cull: CullType,
    pub blend: Option<BlendParameters>
}

impl Default for RenderStates {
    fn default() -> Self {
        Self {
            color_mask: ColorMask::default(),
            depth_mask: true,
            depth_test: DepthTestType::Less,
            cull: CullType::None,
            blend: None
        }
     }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CullType {
    None,
    Back,
    Front,
    FrontAndBack
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DepthTestType {
    Never,
    Less,
    Equal,
    LessOrEqual,
    Greater,
    NotEqual,
    GreaterOrEqual,
    Always
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ColorMask {
    pub red: bool,
    pub green: bool,
    pub blue: bool,
    pub alpha: bool
}

impl ColorMask {
    pub fn enabled() -> Self {
        Self {
            red: true,
            green: true,
            blue: true,
            alpha: true,
        }
    }

    pub fn disabled() -> Self {
        Self {
            red: false,
            green: false,
            blue: false,
            alpha: false,
        }
    }
}

impl Default for ColorMask {
    fn default() -> Self {
        Self::enabled()
     }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BlendParameters {
    pub source_rgb_multiplier: BlendMultiplierType,
    pub source_alpha_multiplier: BlendMultiplierType,
    pub destination_rgb_multiplier: BlendMultiplierType,
    pub destination_alpha_multiplier: BlendMultiplierType,
    pub rgb_equation: BlendEquationType,
    pub alpha_equation: BlendEquationType
}

impl BlendParameters {
    pub fn transparency() -> Self {
        Self {
            source_rgb_multiplier: BlendMultiplierType::SrcAlpha,
            source_alpha_multiplier: BlendMultiplierType::One,
            destination_rgb_multiplier: BlendMultiplierType::OneMinusSrcAlpha,
            destination_alpha_multiplier: BlendMultiplierType::Zero,
            rgb_equation: BlendEquationType::Add,
            alpha_equation: BlendEquationType::Add
        }
    }

    pub fn add() -> Self {
        Self {
            source_rgb_multiplier: BlendMultiplierType::One,
            source_alpha_multiplier: BlendMultiplierType::One,
            destination_rgb_multiplier: BlendMultiplierType::One,
            destination_alpha_multiplier: BlendMultiplierType::One,
            rgb_equation: BlendEquationType::Add,
            alpha_equation: BlendEquationType::Add
        }
    }
}

impl Default for BlendParameters {
    fn default() -> Self {
        Self::transparency()
     }
}

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
    SrcAlphaSaturate
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BlendEquationType {
    Add,
    Subtract,
    ReverseSubtract,
    Max,
    Min
}