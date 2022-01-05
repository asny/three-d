use crate::core::texture::*;
use crate::core::*;

///
/// A color texture cube map that can be rendered into.
///
/// **Note:** [DepthTest] is disabled if not also writing to a [DepthTargetTextureCubeMap].
/// Use a [RenderTargetCubeMap] to write to both color and depth.
///
pub struct ColorTargetTextureCubeMap<T: TextureDataType> {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
    number_of_mip_maps: u32,
    format: Format,
    _dummy: T,
}

impl<T: TextureDataType> ColorTargetTextureCubeMap<T> {
    ///
    /// Creates a new color target cube map.
    ///
    pub fn new(
        context: &Context,
        width: u32,
        height: u32,
        min_filter: Interpolation,
        mag_filter: Interpolation,
        mip_map_filter: Option<Interpolation>,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
        wrap_r: Wrapping,
        format: Format,
    ) -> ThreeDResult<Self> {
        let id = generate(context)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(mip_map_filter, width, height);
        set_parameters(
            context,
            &id,
            consts::TEXTURE_CUBE_MAP,
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
        context.bind_texture(consts::TEXTURE_CUBE_MAP, &id);
        context.tex_storage_2d(
            consts::TEXTURE_CUBE_MAP,
            number_of_mip_maps,
            T::internal_format(format)?,
            width,
            height,
        );
        let tex = Self {
            context: context.clone(),
            id,
            width,
            height,
            number_of_mip_maps,
            format,
            _dummy: T::default(),
        };
        tex.generate_mip_maps();
        Ok(tex)
    }

    ///
    /// Creates a new cube texture generated from the equirectangular texture given as input.
    ///
    pub fn new_from_equirectangular<T_: TextureDataType>(
        context: &Context,
        cpu_texture: &CPUTexture<T_>,
    ) -> ThreeDResult<Self> {
        let texture = Self::new(
            &context,
            cpu_texture.width / 4,
            cpu_texture.width / 4,
            Interpolation::Linear,
            Interpolation::Linear,
            Some(Interpolation::Linear),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Format::RGBA,
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
                return vec2(uv.x, 1.0 - uv.y);
            }
            
            void main()
            {		
                vec2 uv = sample_spherical_map(normalize(pos));
                outColor = vec4(texture(equirectangularMap, uv).rgb, 1.0);
            }";
            let program = ImageCubeEffect::new(context, fragment_shader_source)?;
            let render_target = RenderTargetCubeMap::new_color(context, &texture)?;
            let viewport = Viewport::new_at_origo(texture.width(), texture.height());
            let projection = perspective(degrees(90.0), viewport.aspect(), 0.1, 10.0);
            program.use_texture("equirectangularMap", &map)?;
            program.apply_all(
                &render_target,
                ClearState::default(),
                RenderStates::default(),
                projection,
                viewport,
            )?;
        }
        Ok(texture)
    }

    pub fn write(
        &self,
        side: CubeMapSide,
        clear_state: ClearState,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        RenderTargetCubeMap::new_color(&self.context, &self)?.write(side, clear_state, render)
    }

    pub fn write_to_mip_level(
        &self,
        side: CubeMapSide,
        mip_level: u32,
        clear_state: ClearState,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        RenderTargetCubeMap::new_color(&self.context, &self)?.write_to_mip_level(
            side,
            mip_level,
            clear_state,
            render,
        )
    }

    pub(in crate::core) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.context
                .bind_texture(consts::TEXTURE_CUBE_MAP, &self.id);
            self.context.generate_mipmap(consts::TEXTURE_CUBE_MAP);
        }
    }

    pub(in crate::core) fn bind_as_color_target(
        &self,
        side: CubeMapSide,
        channel: u32,
        mip_level: u32,
    ) {
        self.context.framebuffer_texture_2d(
            consts::DRAW_FRAMEBUFFER,
            consts::COLOR_ATTACHMENT0 + channel,
            side.to_const(),
            &self.id,
            mip_level,
        );
    }
}

impl<T: TextureDataType> TextureCube for ColorTargetTextureCubeMap<T> {
    fn bind(&self, location: u32) {
        bind_at(&self.context, &self.id, consts::TEXTURE_CUBE_MAP, location);
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn format(&self) -> Format {
        self.format
    }
    fn is_hdr(&self) -> bool {
        T::bits_per_channel() > 8
    }
}

impl<T: TextureDataType> Drop for ColorTargetTextureCubeMap<T> {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}
