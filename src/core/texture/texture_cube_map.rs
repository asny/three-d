use crate::core::texture::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
///
/// The 6 sides of a cube map
///
pub enum CubeMapSide {
    /// Positive y
    Top,
    /// Negative y
    Bottom,
    /// Positive x
    Right,
    /// Negative x
    Left,
    /// Negative z
    Front,
    /// Positive z
    Back,
}

///
/// Iterator over the 6 side of a cube map.
///
pub struct CubeMapSideIterator {
    index: usize,
}

impl CubeMapSideIterator {
    ///
    /// Creates a new iterator over the 6 side of a cube map.
    ///
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl<'a> Iterator for CubeMapSideIterator {
    type Item = CubeMapSide;
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        match self.index {
            1 => Some(CubeMapSide::Right),
            2 => Some(CubeMapSide::Left),
            3 => Some(CubeMapSide::Top),
            4 => Some(CubeMapSide::Bottom),
            5 => Some(CubeMapSide::Front),
            6 => Some(CubeMapSide::Back),
            _ => None,
        }
    }
}

impl CubeMapSide {
    ///
    /// Iterator over the 6 side of a cube map.
    ///
    pub fn iter() -> CubeMapSideIterator {
        CubeMapSideIterator::new()
    }

    pub(in crate::core) fn to_const(&self) -> u32 {
        match self {
            CubeMapSide::Right => crate::context::TEXTURE_CUBE_MAP_POSITIVE_X,
            CubeMapSide::Left => crate::context::TEXTURE_CUBE_MAP_NEGATIVE_X,
            CubeMapSide::Top => crate::context::TEXTURE_CUBE_MAP_POSITIVE_Y,
            CubeMapSide::Bottom => crate::context::TEXTURE_CUBE_MAP_NEGATIVE_Y,
            CubeMapSide::Front => crate::context::TEXTURE_CUBE_MAP_POSITIVE_Z,
            CubeMapSide::Back => crate::context::TEXTURE_CUBE_MAP_NEGATIVE_Z,
        }
    }

    /// The up direction that should be used when rendering into this cube map side.
    pub fn up(&self) -> Vec3 {
        match self {
            CubeMapSide::Right => vec3(0.0, -1.0, 0.0),
            CubeMapSide::Left => vec3(0.0, -1.0, 0.0),
            CubeMapSide::Top => vec3(0.0, 0.0, 1.0),
            CubeMapSide::Bottom => vec3(0.0, 0.0, -1.0),
            CubeMapSide::Front => vec3(0.0, -1.0, 0.0),
            CubeMapSide::Back => vec3(0.0, -1.0, 0.0),
        }
    }

    /// The direction from origo towards the center of this cube map side.
    pub fn direction(&self) -> Vec3 {
        match self {
            CubeMapSide::Right => vec3(1.0, 0.0, 0.0),
            CubeMapSide::Left => vec3(-1.0, 0.0, 0.0),
            CubeMapSide::Top => vec3(0.0, 1.0, 0.0),
            CubeMapSide::Bottom => vec3(0.0, -1.0, 0.0),
            CubeMapSide::Front => vec3(0.0, 0.0, 1.0),
            CubeMapSide::Back => vec3(0.0, 0.0, -1.0),
        }
    }
}

///
/// A texture that covers all 6 sides of a cube.
///
pub struct TextureCubeMap {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
    number_of_mip_maps: u32,
    is_hdr: bool,
    data_byte_size: usize,
}

impl TextureCubeMap {
    ///
    /// Creates a new cube map texture from the given [CpuTexture]s.
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
        match &front.data {
            TextureData::RU8(front_data) => Self::new_with_data(
                context,
                front,
                right.wrap_s,
                ru8_data(right),
                ru8_data(left),
                ru8_data(top),
                ru8_data(bottom),
                front_data,
                ru8_data(back),
            ),
            TextureData::RgU8(front_data) => Self::new_with_data(
                context,
                front,
                right.wrap_s,
                rgu8_data(right),
                rgu8_data(left),
                rgu8_data(top),
                rgu8_data(bottom),
                front_data,
                rgu8_data(back),
            ),
            TextureData::RgbU8(front_data) => Self::new_with_data(
                context,
                front,
                right.wrap_s,
                rgbu8_data(right),
                rgbu8_data(left),
                rgbu8_data(top),
                rgbu8_data(bottom),
                front_data,
                rgbu8_data(back),
            ),
            TextureData::RgbaU8(front_data) => Self::new_with_data(
                context,
                front,
                right.wrap_s,
                rgbau8_data(right),
                rgbau8_data(left),
                rgbau8_data(top),
                rgbau8_data(bottom),
                front_data,
                rgbau8_data(back),
            ),
            TextureData::RF16(front_data) => Self::new_with_data(
                context,
                front,
                right.wrap_s,
                rf16_data(right),
                rf16_data(left),
                rf16_data(top),
                rf16_data(bottom),
                front_data,
                rf16_data(back),
            ),
            TextureData::RgF16(front_data) => Self::new_with_data(
                context,
                front,
                right.wrap_s,
                rgf16_data(right),
                rgf16_data(left),
                rgf16_data(top),
                rgf16_data(bottom),
                front_data,
                rgf16_data(back),
            ),
            TextureData::RgbF16(front_data) => Self::new_with_data(
                context,
                front,
                right.wrap_s,
                rgbf16_data(right),
                rgbf16_data(left),
                rgbf16_data(top),
                rgbf16_data(bottom),
                front_data,
                rgbf16_data(back),
            ),
            TextureData::RgbaF16(front_data) => Self::new_with_data(
                context,
                front,
                right.wrap_s,
                rgbaf16_data(right),
                rgbaf16_data(left),
                rgbaf16_data(top),
                rgbaf16_data(bottom),
                front_data,
                rgbaf16_data(back),
            ),
            TextureData::RF32(front_data) => Self::new_with_data(
                context,
                front,
                right.wrap_s,
                rf32_data(right),
                rf32_data(left),
                rf32_data(top),
                rf32_data(bottom),
                front_data,
                rf32_data(back),
            ),
            TextureData::RgF32(front_data) => Self::new_with_data(
                context,
                front,
                right.wrap_s,
                rgf32_data(right),
                rgf32_data(left),
                rgf32_data(top),
                rgf32_data(bottom),
                front_data,
                rgf32_data(back),
            ),
            TextureData::RgbF32(front_data) => Self::new_with_data(
                context,
                front,
                right.wrap_s,
                rgbf32_data(right),
                rgbf32_data(left),
                rgbf32_data(top),
                rgbf32_data(bottom),
                front_data,
                rgbf32_data(back),
            ),
            TextureData::RgbaF32(front_data) => Self::new_with_data(
                context,
                front,
                right.wrap_s,
                rgbaf32_data(right),
                rgbaf32_data(left),
                rgbaf32_data(top),
                rgbaf32_data(bottom),
                front_data,
                rgbaf32_data(back),
            ),
        }
    }

    fn new_with_data<T: TextureDataType>(
        context: &Context,
        cpu_texture: &CpuTexture,
        wrap_r: Wrapping,
        right_data: &[T],
        left_data: &[T],
        top_data: &[T],
        bottom_data: &[T],
        front_data: &[T],
        back_data: &[T],
    ) -> Self {
        let mut texture = Self::new_empty::<T>(
            context,
            cpu_texture.width,
            cpu_texture.height,
            cpu_texture.min_filter,
            cpu_texture.mag_filter,
            cpu_texture.mip_map_filter,
            cpu_texture.wrap_s,
            cpu_texture.wrap_t,
            wrap_r,
        );
        texture.fill(
            right_data,
            left_data,
            top_data,
            bottom_data,
            front_data,
            back_data,
        );
        texture
    }

    ///
    /// Creates a new texture cube map.
    ///
    pub fn new_empty<T: TextureDataType>(
        context: &Context,
        width: u32,
        height: u32,
        min_filter: Interpolation,
        mag_filter: Interpolation,
        mip_map_filter: Option<Interpolation>,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
        wrap_r: Wrapping,
    ) -> Self {
        let id = generate(context);
        let number_of_mip_maps = calculate_number_of_mip_maps(mip_map_filter, width, height, None);
        let texture = Self {
            context: context.clone(),
            id,
            width,
            height,
            number_of_mip_maps,
            is_hdr: std::mem::size_of::<T>() as u32 / T::size() > 1,
            data_byte_size: std::mem::size_of::<T>(),
        };
        texture.bind();
        set_parameters(
            context,
            crate::context::TEXTURE_CUBE_MAP,
            min_filter,
            mag_filter,
            if number_of_mip_maps == 1 {
                None
            } else {
                mip_map_filter
            },
            wrap_s,
            wrap_t,
            Some(wrap_r),
        );
        unsafe {
            context.tex_storage_2d(
                crate::context::TEXTURE_CUBE_MAP,
                number_of_mip_maps as i32,
                T::internal_format(),
                width as i32,
                height as i32,
            );
        }
        texture.generate_mip_maps();
        texture
    }

    ///
    /// Fills the cube map texture with the given pixel data for the 6 images.
    ///
    /// # Panic
    /// Will panic if the length of the data for all 6 images does not correspond to the width, height and format specified at construction.
    /// It is therefore necessary to create a new texture if the texture size or format has changed.
    ///
    pub fn fill<T: TextureDataType>(
        &mut self,
        right_data: &[T],
        left_data: &[T],
        top_data: &[T],
        bottom_data: &[T],
        front_data: &[T],
        back_data: &[T],
    ) {
        check_data_length::<T>(
            self.width,
            self.height,
            1,
            self.data_byte_size,
            right_data.len(),
        );
        check_data_length::<T>(
            self.width,
            self.height,
            1,
            self.data_byte_size,
            left_data.len(),
        );
        check_data_length::<T>(
            self.width,
            self.height,
            1,
            self.data_byte_size,
            top_data.len(),
        );
        check_data_length::<T>(
            self.width,
            self.height,
            1,
            self.data_byte_size,
            bottom_data.len(),
        );
        check_data_length::<T>(
            self.width,
            self.height,
            1,
            self.data_byte_size,
            front_data.len(),
        );
        check_data_length::<T>(
            self.width,
            self.height,
            1,
            self.data_byte_size,
            back_data.len(),
        );
        self.bind();
        for i in 0..6 {
            let data = match i {
                0 => right_data,
                1 => left_data,
                2 => top_data,
                3 => bottom_data,
                4 => front_data,
                5 => back_data,
                _ => unreachable!(),
            };
            unsafe {
                self.context.tex_sub_image_2d(
                    crate::context::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                    0,
                    0,
                    0,
                    self.width as i32,
                    self.height as i32,
                    format_from_data_type::<T>(),
                    T::data_type(),
                    crate::context::PixelUnpackData::Slice(to_byte_slice(data)),
                );
            }
        }
        self.generate_mip_maps();
    }

    ///
    /// Creates a new cube texture generated from the equirectangular texture given as input.
    ///
    pub fn new_from_equirectangular<T: PrimitiveDataType + TextureDataType>(
        context: &Context,
        cpu_texture: &CpuTexture,
    ) -> Self {
        let texture_size = cpu_texture.width / 4;
        let mut texture = Self::new_empty::<[T; 4]>(
            &context,
            texture_size,
            texture_size,
            Interpolation::Linear,
            Interpolation::Linear,
            Some(Interpolation::Linear),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );

        {
            let map = Texture2D::new(context, cpu_texture);
            let fragment_shader_source = "
            uniform sampler2D equirectangularMap;
            in vec3 pos;
            layout (location = 0) out vec4 outColor;
            
            void main()
            {
                vec3 v = normalize(pos);
                vec2 uv = vec2(0.1591 * atan(v.z, v.x) + 0.5, 0.3183 * asin(v.y) + 0.5);
                outColor = texture(equirectangularMap, uv);
            }";

            for side in CubeMapSide::iter() {
                let viewport = Viewport::new_at_origo(texture_size, texture_size);
                texture
                    .as_color_target(&[side], None)
                    .clear(ClearState::default())
                    .write(|| {
                        apply_cube_effect(
                            context,
                            side,
                            fragment_shader_source,
                            RenderStates::default(),
                            viewport,
                            |program| {
                                program.use_texture("equirectangularMap", &map);
                            },
                        );
                    });
            }
        }
        texture
    }

    ///
    /// Returns a [ColorTarget] which can be used to clear, write to and read from the given side and mip level of this texture.
    /// Combine this together with a [DepthTarget] with [RenderTarget::new] to be able to write to both a depth and color target at the same time.
    /// If `None` is specified as the mip level, the 0 level mip level is used and mip maps are generated after a write operation if a mip map filter is specified.
    /// Otherwise, the given mip level is used and no mip maps are generated.
    ///
    /// **Note:** [DepthTest] is disabled if not also writing to a depth texture.
    ///
    pub fn as_color_target<'a>(
        &'a mut self,
        sides: &'a [CubeMapSide],
        mip_level: Option<u32>,
    ) -> ColorTarget<'a> {
        ColorTarget::new_texture_cube_map(&self.context, self, sides, mip_level)
    }

    /// The width of this texture.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of this texture.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Whether this cube map contain HDR (high dynamic range) data.
    pub fn is_hdr(&self) -> bool {
        self.is_hdr
    }

    pub(in crate::core) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.bind();
            unsafe {
                self.context
                    .generate_mipmap(crate::context::TEXTURE_CUBE_MAP);
            }
        }
    }

    pub(in crate::core) fn bind_as_color_target(
        &self,
        side: CubeMapSide,
        channel: u32,
        mip_level: u32,
    ) {
        unsafe {
            self.context.framebuffer_texture_2d(
                crate::context::DRAW_FRAMEBUFFER,
                crate::context::COLOR_ATTACHMENT0 + channel,
                side.to_const(),
                Some(self.id),
                mip_level as i32,
            );
        }
    }

    pub(in crate::core) fn bind(&self) {
        unsafe {
            self.context
                .bind_texture(crate::context::TEXTURE_CUBE_MAP, Some(self.id));
        }
    }
}

impl Drop for TextureCubeMap {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_texture(self.id);
        }
    }
}
