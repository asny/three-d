use crate::renderer::*;

pub struct Volume<T: TextureDataType> {
    model: Model<VolumeMaterial<T>>,
}

impl<T: TextureDataType> Volume<T> {
    pub fn new(context: &Context, material: VolumeMaterial<T>) -> ThreeDResult<Self> {
        Ok(Self {
            model: Model::new_with_material(context, &cpu_mesh(material.size), material)?,
        })
    }
}

impl<T: TextureDataType> std::ops::Deref for Volume<T> {
    type Target = Model<VolumeMaterial<T>>;
    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

impl<T: TextureDataType> std::ops::DerefMut for Volume<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.model
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
        self.model.render(camera, lights)
    }

    fn is_transparent(&self) -> bool {
        self.model.is_transparent()
    }
}

fn cpu_mesh(size: Vec3) -> CpuMesh {
    let mut m = CpuMesh {
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
    };
    m.transform(&Mat4::from_nonuniform_scale(
        0.5 * size.x,
        0.5 * size.y,
        0.5 * size.z,
    ));
    m
}
