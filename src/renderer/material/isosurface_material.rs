use crate::core::*;
use crate::renderer::*;

///
/// A material that renders the isosurface defined by the voxel data in the [IsosurfaceMaterial::voxels] and the [IsosurfaceMaterial::threshold].
/// The surface is defined by all the points in the volume where the red channel of the voxel data is equal to the threshold.
/// This material should be applied to a cube with center in origo, for example [CpuMesh::cube].
///
#[derive(Clone)]
pub struct IsosurfaceMaterial {
    /// The voxel data that defines the isosurface.
    pub voxels: std::sync::Arc<Texture3D>,
    /// Threshold (in the range [0..1]) that defines the surface in the voxel data.
    pub threshold: f32,
    /// Base surface color.
    pub color: Srgba,
    /// A value in the range `[0..1]` specifying how metallic the surface is.
    pub metallic: f32,
    /// A value in the range `[0..1]` specifying how rough the surface is.
    pub roughness: f32,
    /// The size of the cube that is used to render the voxel data. The texture is scaled to fill the entire cube.
    pub size: Vec3,
    /// The lighting model used when rendering this material
    pub lighting_model: LightingModel,
}

impl Material for IsosurfaceMaterial {
    fn id(&self) -> u16 {
        0b1u16 << 15 | 0b1100u16
    }

    fn fragment_shader_source(&self, lights: &[&dyn Light]) -> String {
        let mut source = lights_shader_source(lights, self.lighting_model);
        source.push_str(ToneMapping::fragment_shader_source());
        source.push_str(ColorMapping::fragment_shader_source());
        source.push_str(include_str!("shaders/isosurface_material.frag"));
        source
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            position: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &[&dyn Light]) {
        camera.tone_mapping.use_uniforms(program);
        camera.color_mapping.use_uniforms(program);
        for (i, light) in lights.iter().enumerate() {
            light.use_uniforms(program, i as u32);
        }
        program.use_uniform("cameraPosition", camera.position());
        program.use_uniform("surfaceColor", self.color.to_linear_srgb());
        program.use_uniform("metallic", self.metallic);
        program.use_uniform_if_required("roughness", self.roughness);
        program.use_uniform("size", self.size);
        program.use_uniform("threshold", self.threshold);
        program.use_uniform(
            "h",
            vec3(
                1.0 / self.voxels.width() as f32,
                1.0 / self.voxels.height() as f32,
                1.0 / self.voxels.depth() as f32,
            ),
        );
        program.use_texture_3d("tex", &self.voxels);
    }
    fn render_states(&self) -> RenderStates {
        RenderStates {
            blend: Blend::TRANSPARENCY,
            ..Default::default()
        }
    }
    fn material_type(&self) -> MaterialType {
        MaterialType::Transparent
    }
}

impl FromCpuVoxelGrid for IsosurfaceMaterial {
    fn from_cpu_voxel_grid(context: &Context, cpu_voxel_grid: &CpuVoxelGrid) -> Self {
        Self {
            voxels: std::sync::Arc::new(Texture3D::new(context, &cpu_voxel_grid.voxels)),
            lighting_model: LightingModel::Blinn,
            size: cpu_voxel_grid.size,
            threshold: 0.15,
            color: Srgba::WHITE,
            roughness: 1.0,
            metallic: 0.0,
        }
    }
}
