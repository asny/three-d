
#[derive(Debug, Copy, Clone)]
pub struct RenderStates {
    pub color_mask: ColorMask,
    pub depth_mask: bool,
    pub depth_test: DepthTestType,
    pub viewport: Viewport,
    pub cull: CullType,
    pub blend: Option<BlendParameters>
}

impl Default for RenderStates {
    fn default() -> Self {
        Self {
            color_mask: ColorMask::default(),
            depth_mask: true,
            depth_test: DepthTestType::Less,
            viewport: Viewport::default(),
            cull: CullType::None,
            blend: None
        }
     }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub width: usize,
    pub height: usize
}

impl Viewport {
    pub fn new(width: usize, height: usize) -> Self {
        Self {x: 0, y: 0, width, height}
    }

    pub fn aspect(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 1024,
            height: 1024
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

impl Default for ColorMask {
    fn default() -> Self {
        Self {
            red: true,
            green: true,
            blue: true,
            alpha: true,
        }
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