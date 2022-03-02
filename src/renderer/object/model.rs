use crate::core::*;
use crate::renderer::*;
use std::rc::Rc;

///
/// A 3D model consisting of a triangle mesh and any material that implements the `Material` trait.
///
pub struct Model<M: Material> {
    context: Context,
    mesh: Rc<Mesh>,
    aabb: AxisAlignedBoundingBox,
    aabb_local: AxisAlignedBoundingBox,
    transformation: Mat4,
    texture_transform: Mat3,
    /// The material applied to the model
    pub material: M,
}

impl Model<ColorMaterial<std::rc::Rc<Texture2D<u8>>>> {
    ///
    /// Creates a new 3D model with a triangle mesh as geometry and a default [ColorMaterial].
    ///
    pub fn new(context: &Context, cpu_mesh: &CpuMesh) -> ThreeDResult<Self> {
        Self::new_with_material(context, cpu_mesh, ColorMaterial::default())
    }
}

impl<M: Material> Model<M> {
    ///
    /// Creates a new 3D model with a triangle mesh as geometry and the given material.
    ///
    pub fn new_with_material(
        context: &Context,
        cpu_mesh: &CpuMesh,
        material: M,
    ) -> ThreeDResult<Self> {
        let mesh = Rc::new(Mesh::new(context, cpu_mesh)?);
        let aabb = cpu_mesh.compute_aabb();
        Ok(Self {
            mesh,
            aabb,
            aabb_local: aabb.clone(),
            transformation: Mat4::identity(),
            texture_transform: Mat3::identity(),
            context: context.clone(),
            material,
        })
    }

    ///
    /// Get the texture transform applied to the uv coordinates of the model.
    ///
    pub fn texture_transform(&mut self) -> &Mat3 {
        &self.texture_transform
    }

    ///
    /// Set the texture transform applied to the uv coordinates of the model.
    ///
    pub fn set_texture_transform(&mut self, texture_transform: Mat3) {
        self.texture_transform = texture_transform;
    }

    pub(in crate::renderer) fn set_transformation_2d(&mut self, transformation: Mat3) {
        self.set_transformation(Mat4::new(
            transformation.x.x,
            transformation.x.y,
            0.0,
            transformation.x.z,
            transformation.y.x,
            transformation.y.y,
            0.0,
            transformation.y.z,
            0.0,
            0.0,
            1.0,
            0.0,
            transformation.z.x,
            transformation.z.y,
            0.0,
            transformation.z.z,
        ));
    }

    pub(super) fn vertex_shader_source(fragment_shader_source: &str) -> ThreeDResult<String> {
        let use_positions = fragment_shader_source.find("in vec3 pos;").is_some();
        let use_normals = fragment_shader_source.find("in vec3 nor;").is_some();
        let use_tangents = fragment_shader_source.find("in vec3 tang;").is_some();
        let use_uvs = fragment_shader_source.find("in vec2 uvs;").is_some();
        let use_colors = fragment_shader_source.find("in vec4 col;").is_some();
        Ok(format!(
            "{}{}{}{}{}{}{}",
            if use_positions {
                "#define USE_POSITIONS\n"
            } else {
                ""
            },
            if use_normals {
                "#define USE_NORMALS\n"
            } else {
                ""
            },
            if use_tangents {
                if fragment_shader_source.find("in vec3 bitang;").is_none() {
                    Err(CoreError::MissingBitangent)?;
                }
                "#define USE_TANGENTS\n"
            } else {
                ""
            },
            if use_uvs { "#define USE_UVS\n" } else { "" },
            if use_colors {
                "#define USE_COLORS\n"
            } else {
                ""
            },
            include_str!("../../core/shared.frag"),
            include_str!("shaders/mesh.vert"),
        ))
    }
}

impl<M: Material> Geometry for Model<M> {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.aabb
    }

    fn transformation(&self) -> Mat4 {
        self.transformation
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        let fragment_shader_source =
            material.fragment_shader_source(self.mesh.color_buffer.is_some(), lights);
        self.context.program(
            &Self::vertex_shader_source(&fragment_shader_source)?,
            &fragment_shader_source,
            |program| {
                material.use_uniforms(program, camera, lights)?;
                program.use_uniform_block("Camera", camera.uniform_buffer());
                program.use_uniform_mat4("modelMatrix", &self.transformation)?;

                if program.requires_attribute("position") {
                    program.use_attribute_vec3("position", &self.mesh.position_buffer)?;
                }
                if program.requires_attribute("uv_coordinates") {
                    program.use_uniform_mat3("textureTransform", &self.texture_transform)?;
                    let uv_buffer = self
                        .mesh
                        .uv_buffer
                        .as_ref()
                        .ok_or(CoreError::MissingMeshBuffer("uv coordinates".to_string()))?;
                    program.use_attribute_vec2("uv_coordinates", uv_buffer)?;
                }
                if program.requires_attribute("normal") {
                    let normal_buffer = self
                        .mesh
                        .normal_buffer
                        .as_ref()
                        .ok_or(CoreError::MissingMeshBuffer("normal".to_string()))?;
                    program.use_attribute_vec3("normal", normal_buffer)?;
                    program.use_uniform_mat4(
                        "normalMatrix",
                        &self.transformation.invert().unwrap().transpose(),
                    )?;
                    if program.requires_attribute("tangent") {
                        let tangent_buffer = self
                            .mesh
                            .tangent_buffer
                            .as_ref()
                            .ok_or(CoreError::MissingMeshBuffer("tangent".to_string()))?;
                        program.use_attribute_vec4("tangent", tangent_buffer)?;
                    }
                }
                if program.requires_attribute("color") {
                    let color_buffer = self
                        .mesh
                        .color_buffer
                        .as_ref()
                        .ok_or(CoreError::MissingMeshBuffer("color".to_string()))?;
                    program.use_attribute_vec4("color", color_buffer)?;
                }
                if let Some(ref index_buffer) = self.mesh.index_buffer {
                    program.draw_elements(
                        material.render_states(),
                        camera.viewport(),
                        index_buffer,
                    );
                } else {
                    program.draw_arrays(
                        material.render_states(),
                        camera.viewport(),
                        self.mesh.position_buffer.count() as u32 / 3,
                    );
                }
                Ok(())
            },
        )
    }
}

impl<M: Material> GeometryMut for Model<M> {
    fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
        let mut aabb = self.aabb_local.clone();
        aabb.transform(&self.transformation);
        self.aabb = aabb;
    }
}

impl<M: Material> Object for Model<M> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) -> ThreeDResult<()> {
        self.render_with_material(&self.material, camera, lights)
    }

    fn is_transparent(&self) -> bool {
        self.material.is_transparent()
    }
}

impl<M: Material + Clone> Clone for Model<M> {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            mesh: self.mesh.clone(),
            aabb: self.aabb.clone(),
            aabb_local: self.aabb_local.clone(),
            transformation: self.transformation,
            texture_transform: self.texture_transform,
            material: self.material.clone(),
        }
    }
}
