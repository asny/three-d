use crate::renderer::*;

pub struct Volume<T: TextureDataType> {
    model: Model<VolumeMaterial<T>>,
}

impl<T: TextureDataType> Volume<T> {
    pub fn new(context: &Context, cpu_texture: &CpuTexture3D<T>) -> ThreeDResult<Self> {
        Ok(Self {
            model: Model::new_with_material(
                context,
                &CpuMesh {
                    positions: Positions::F32(vec![
                        vec3(1.0, 1.0, -1.0),
                        vec3(-1.0, 1.0, -1.0),
                        vec3(1.0, 1.0, 1.0),
                        vec3(-1.0, 1.0, 1.0),
                        vec3(1.0, 1.0, 1.0),
                        vec3(-1.0, 1.0, -1.0),
                        vec3(-1.0, -1.0, -1.0),
                        vec3(1.0, -1.0, -1.0),
                        vec3(1.0, -1.0, 1.0),
                        vec3(1.0, -1.0, 1.0),
                        vec3(-1.0, -1.0, 1.0),
                        vec3(-1.0, -1.0, -1.0),
                        vec3(1.0, -1.0, -1.0),
                        vec3(-1.0, -1.0, -1.0),
                        vec3(1.0, 1.0, -1.0),
                        vec3(-1.0, 1.0, -1.0),
                        vec3(1.0, 1.0, -1.0),
                        vec3(-1.0, -1.0, -1.0),
                        vec3(-1.0, -1.0, 1.0),
                        vec3(1.0, -1.0, 1.0),
                        vec3(1.0, 1.0, 1.0),
                        vec3(1.0, 1.0, 1.0),
                        vec3(-1.0, 1.0, 1.0),
                        vec3(-1.0, -1.0, 1.0),
                        vec3(1.0, -1.0, -1.0),
                        vec3(1.0, 1.0, -1.0),
                        vec3(1.0, 1.0, 1.0),
                        vec3(1.0, 1.0, 1.0),
                        vec3(1.0, -1.0, 1.0),
                        vec3(1.0, -1.0, -1.0),
                        vec3(-1.0, 1.0, -1.0),
                        vec3(-1.0, -1.0, -1.0),
                        vec3(-1.0, 1.0, 1.0),
                        vec3(-1.0, -1.0, 1.0),
                        vec3(-1.0, 1.0, 1.0),
                        vec3(-1.0, -1.0, -1.0),
                    ]),
                    ..Default::default()
                },
                VolumeMaterial {
                    texture: Texture3D::new(context, cpu_texture)?,
                    lighting_model: LightingModel::Blinn,
                },
            )?,
        })
    }
}

impl<T: TextureDataType> Geometry for Volume<T> {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.model.aabb()
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        self.model.render_with_material(material, camera, lights)
    }
}

impl<T: TextureDataType> Object for Volume<T> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) -> ThreeDResult<()> {
        self.render_with_material(&self.model.material, camera, lights)
    }

    fn is_transparent(&self) -> bool {
        false
    }
}
