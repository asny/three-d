
#[derive(Debug, Copy, Clone)]
pub struct RenderStates {
    pub depth_write: bool,
    pub depth_test: DepthTestType,
    pub cull: CullType,
    pub blend: Option<BlendParameters>
}

impl Default for RenderStates {
    fn default() -> Self {
        Self {
            depth_write: true,
            depth_test: DepthTestType::None,
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
    None,
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
pub struct BlendParameters {
    pub source_rgb_multiplier: BlendMultiplierType,
    pub source_alpha_multiplier: BlendMultiplierType,
    pub destination_rgb_multiplier: BlendMultiplierType,
    pub destination_alpha_multiplier: BlendMultiplierType,
    pub rgb_equation: BlendEquationType,
    pub alpha_equation: BlendEquationType
}

impl BlendParameters {
    pub fn new(equation: BlendEquationType, source_multiplier: BlendMultiplierType, destination_multiplier: BlendMultiplierType) -> BlendParameters {
        BlendParameters {
            source_rgb_multiplier: source_multiplier,
            source_alpha_multiplier: source_multiplier,
            destination_rgb_multiplier: destination_multiplier,
            destination_alpha_multiplier: destination_multiplier,
            rgb_equation: equation,
            alpha_equation: equation
        }
    }
}

impl Default for BlendParameters {
    fn default() -> Self {
        Self {
            source_rgb_multiplier: BlendMultiplierType::One,
            source_alpha_multiplier: BlendMultiplierType::One,
            destination_rgb_multiplier: BlendMultiplierType::Zero,
            destination_alpha_multiplier: BlendMultiplierType::Zero,
            rgb_equation: BlendEquationType::Add,
            alpha_equation: BlendEquationType::Add
        }
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