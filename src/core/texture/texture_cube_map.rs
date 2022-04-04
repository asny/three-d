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

    pub(in crate::core) fn view(&self) -> Mat4 {
        match self {
            CubeMapSide::Right => Mat4::look_at_rh(
                Point::new(0.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
                vec3(0.0, -1.0, 0.0),
            ),
            CubeMapSide::Left => Mat4::look_at_rh(
                Point::new(0.0, 0.0, 0.0),
                Point::new(-1.0, 0.0, 0.0),
                vec3(0.0, -1.0, 0.0),
            ),
            CubeMapSide::Top => Mat4::look_at_rh(
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 1.0, 0.0),
                vec3(0.0, 0.0, 1.0),
            ),
            CubeMapSide::Bottom => Mat4::look_at_rh(
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, -1.0, 0.0),
                vec3(0.0, 0.0, -1.0),
            ),
            CubeMapSide::Front => Mat4::look_at_rh(
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 0.0, 1.0),
                vec3(0.0, -1.0, 0.0),
            ),
            CubeMapSide::Back => Mat4::look_at_rh(
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 0.0, -1.0),
                vec3(0.0, -1.0, 0.0),
            ),
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
    /// Creates a new texture cube map from the given cpu texture.
    /// The cpu texture must contain 6 images all with the width and height specified in the cpu texture.
    /// The images are used in the following order; right, left, top, bottom, front, back.
    ///
    pub fn new(context: &Context, cpu_texture: &CpuTextureCube) -> ThreeDResult<Self> {
        match &cpu_texture.data {
            TextureCubeData::RU8(right, left, top, bottom, front, back) => {
                Self::new_with_data(context, cpu_texture, right, left, top, bottom, front, back)
            }
            TextureCubeData::RgU8(right, left, top, bottom, front, back) => {
                Self::new_with_data(context, cpu_texture, right, left, top, bottom, front, back)
            }
            TextureCubeData::RgbU8(right, left, top, bottom, front, back) => {
                Self::new_with_data(context, cpu_texture, right, left, top, bottom, front, back)
            }
            TextureCubeData::RgbaU8(right, left, top, bottom, front, back) => {
                Self::new_with_data(context, cpu_texture, right, left, top, bottom, front, back)
            }
            TextureCubeData::RF16(right, left, top, bottom, front, back) => {
                Self::new_with_data(context, cpu_texture, right, left, top, bottom, front, back)
            }
            TextureCubeData::RgF16(right, left, top, bottom, front, back) => {
                Self::new_with_data(context, cpu_texture, right, left, top, bottom, front, back)
            }
            TextureCubeData::RgbF16(right, left, top, bottom, front, back) => {
                Self::new_with_data(context, cpu_texture, right, left, top, bottom, front, back)
            }
            TextureCubeData::RgbaF16(right, left, top, bottom, front, back) => {
                Self::new_with_data(context, cpu_texture, right, left, top, bottom, front, back)
            }
            TextureCubeData::RF32(right, left, top, bottom, front, back) => {
                Self::new_with_data(context, cpu_texture, right, left, top, bottom, front, back)
            }
            TextureCubeData::RgF32(right, left, top, bottom, front, back) => {
                Self::new_with_data(context, cpu_texture, right, left, top, bottom, front, back)
            }
            TextureCubeData::RgbF32(right, left, top, bottom, front, back) => {
                Self::new_with_data(context, cpu_texture, right, left, top, bottom, front, back)
            }
            TextureCubeData::RgbaF32(right, left, top, bottom, front, back) => {
                Self::new_with_data(context, cpu_texture, right, left, top, bottom, front, back)
            }
        }
    }

    fn new_with_data<T: TextureDataType>(
        context: &Context,
        cpu_texture: &CpuTextureCube,
        right_data: &[T],
        left_data: &[T],
        top_data: &[T],
        bottom_data: &[T],
        front_data: &[T],
        back_data: &[T],
    ) -> ThreeDResult<Self> {
        let mut texture = Self::new_empty::<T>(
            context,
            cpu_texture.width,
            cpu_texture.height,
            cpu_texture.min_filter,
            cpu_texture.mag_filter,
            cpu_texture.mip_map_filter,
            cpu_texture.wrap_s,
            cpu_texture.wrap_t,
            cpu_texture.wrap_r,
        )?;
        texture.fill(
            right_data,
            left_data,
            top_data,
            bottom_data,
            front_data,
            back_data,
        )?;
        Ok(texture)
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
    ) -> ThreeDResult<Self> {
        let id = generate(context)?;
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
        )?;
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
        context.error_check()?;
        Ok(texture)
    }

    ///
    /// Fills the cube map texture with the given pixel data for the 6 images.
    ///
    /// # Errors
    /// Returns an error if the length of the data for all 6 images does not correspond to the width, height and format specified at construction.
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
    ) -> ThreeDResult<()> {
        check_data_length(self.width, self.height, 1, self.data_byte_size, right_data)?;
        check_data_length(self.width, self.height, 1, self.data_byte_size, left_data)?;
        check_data_length(self.width, self.height, 1, self.data_byte_size, top_data)?;
        check_data_length(self.width, self.height, 1, self.data_byte_size, bottom_data)?;
        check_data_length(self.width, self.height, 1, self.data_byte_size, front_data)?;
        check_data_length(self.width, self.height, 1, self.data_byte_size, back_data)?;
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
        self.context.error_check()
    }

    ///
    /// Creates a new cube texture generated from the equirectangular texture given as input.
    ///
    pub fn new_from_equirectangular<T: PrimitiveDataType + TextureDataType>(
        context: &Context,
        cpu_texture: &CpuTexture,
    ) -> ThreeDResult<Self> {
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
        )?;

        {
            let map = Texture2D::new(context, cpu_texture)?;
            let fragment_shader_source = "uniform sampler2D equirectangularMap;
            const vec2 invAtan = vec2(0.1591, 0.3183);
            
            in vec3 pos;
            layout (location = 0) out vec4 outColor;
            
            vec2 sample_spherical_map(vec3 v)
            {
                vec2 uv = vec2(atan(v.z, v.x), asin(v.y));
                uv *= invAtan;
                uv += 0.5;
                return uv;
            }
            
            void main()
            {		
                vec2 uv = sample_spherical_map(normalize(pos));
                outColor = vec4(texture(equirectangularMap, uv).rgb, 1.0);
            }";
            let effect = ImageCubeEffect::new(context, fragment_shader_source)?;
            let render_target = RenderTargetCubeMap::new_color(context, &mut texture)?;

            for side in CubeMapSide::iter() {
                effect.use_texture("equirectangularMap", &map)?;
                let viewport = Viewport::new_at_origo(texture_size, texture_size);
                render_target.write(side, ClearState::default(), || {
                    effect.render(side, RenderStates::default(), viewport)
                })?;
            }
        }
        Ok(texture)
    }

    ///
    /// Writes whatever rendered in the `render` closure into the color texture at the cube map side given by the input parameter `side`.
    /// Before writing, the texture side is cleared based on the given clear state.
    ///
    pub fn write(
        &mut self,
        side: CubeMapSide,
        clear_state: ClearState,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        RenderTargetCubeMap::new_color(&self.context.clone(), self)?.write(
            side,
            clear_state,
            render,
        )
    }

    ///
    /// Writes whatever rendered in the `render` closure into the given mip level of the color texture at the cube map side given by the input parameter `side`.
    /// Before writing, the texture side is cleared based on the given clear state.
    ///
    pub fn write_to_mip_level(
        &mut self,
        side: CubeMapSide,
        mip_level: u32,
        clear_state: ClearState,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        RenderTargetCubeMap::new_color(&self.context.clone(), self)?.write_to_mip_level(
            side,
            mip_level,
            clear_state,
            render,
        )
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
