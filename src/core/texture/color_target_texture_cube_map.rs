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
        Ok(Self {
            context: context.clone(),
            id,
            width,
            height,
            number_of_mip_maps,
            format,
            _dummy: T::default(),
        })
    }

    ///
    /// Creates a new cube texture generated from the equirectangular texture given as input.
    ///
    pub fn new_from_equirectangular<T_: TextureDataType>(
        context: &Context,
        cpu_texture: &CPUTexture<T_>,
    ) -> ThreeDResult<Self> {
        let map = Texture2D::new(context, cpu_texture)?;
        let texture = Self::new(
            &context,
            cpu_texture.height,
            cpu_texture.height,
            Interpolation::Linear,
            Interpolation::Linear,
            Some(Interpolation::Linear),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Format::RGBA,
        )?;

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

        texture.write_to_all(
            0,
            ClearState::default(),
            fragment_shader_source,
            |program| program.use_texture("equirectangularMap", &map),
        )?;
        Ok(texture)
    }

    pub fn write(
        &self,
        color_layer: u32,
        mip_level: u32,
        clear_state: ClearState,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        RenderTargetCubeMap::new_color(&self.context, &self)?.write(
            color_layer,
            0,
            mip_level,
            clear_state,
            render,
        )
    }

    pub fn write_to_all(
        &self,
        mip_level: u32,
        clear_state: ClearState,
        fragment_shader_source: &str,
        use_uniforms: impl Fn(&Program) -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        let vertex_buffer =
            VertexBuffer::new_with_static(&self.context, &CPUMesh::cube().positions)?;

        let mut camera = Camera::new_perspective(
            &self.context,
            Viewport::new_at_origo(self.width(), self.height()),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 0.0, -1.0),
            vec3(0.0, 1.0, 0.0),
            degrees(90.0),
            0.1,
            10.0,
        )?;
        let program = Program::from_source(
            &self.context,
            "layout (std140) uniform Camera
            {
                mat4 viewProjection;
                mat4 view;
                mat4 projection;
                vec3 position;
                float padding;
            } camera;
            
            in vec3 position;
            out vec3 pos;
            
            void main()
            {
                pos = position;
                gl_Position = camera.viewProjection * vec4(position, 1.0);
            }",
            fragment_shader_source,
        )?;
        for i in 0..6 {
            match i {
                0 => camera.set_view(
                    vec3(0.0, 0.0, 0.0),
                    vec3(1.0, 0.0, 0.0),
                    vec3(0.0, -1.0, 0.0),
                ),
                1 => camera.set_view(
                    vec3(0.0, 0.0, 0.0),
                    vec3(-1.0, 0.0, 0.0),
                    vec3(0.0, -1.0, 0.0),
                ),
                2 => camera.set_view(
                    vec3(0.0, 0.0, 0.0),
                    vec3(0.0, 1.0, 0.0),
                    vec3(0.0, 0.0, 1.0),
                ),
                3 => camera.set_view(
                    vec3(0.0, 0.0, 0.0),
                    vec3(0.0, -1.0, 0.0),
                    vec3(0.0, 0.0, -1.0),
                ),
                4 => camera.set_view(
                    vec3(0.0, 0.0, 0.0),
                    vec3(0.0, 0.0, 1.0),
                    vec3(0.0, -1.0, 0.0),
                ),
                5 => camera.set_view(
                    vec3(0.0, 0.0, 0.0),
                    vec3(0.0, 0.0, -1.0),
                    vec3(0.0, -1.0, 0.0),
                ),
                _ => unreachable!(),
            }?;
            program.use_uniform_block("Camera", camera.uniform_buffer());
            program.use_attribute_vec3("position", &vertex_buffer)?;
            self.write(i, mip_level, clear_state, || {
                use_uniforms(&program)?;
                program.draw_arrays(RenderStates::default(), camera.viewport(), 36);
                Ok(())
            })?;
        }
        Ok(())
    }

    pub(in crate::core) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.context
                .bind_texture(consts::TEXTURE_CUBE_MAP, &self.id);
            self.context.generate_mipmap(consts::TEXTURE_CUBE_MAP);
        }
    }

    pub(in crate::core) fn bind_as_color_target(&self, layer: u32, channel: u32, mip_level: u32) {
        self.context.framebuffer_texture_2d(
            consts::DRAW_FRAMEBUFFER,
            consts::COLOR_ATTACHMENT0 + channel,
            consts::TEXTURE_CUBE_MAP_POSITIVE_X + layer,
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
