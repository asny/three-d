use crate::renderer::*;

///
/// Three arrows indicating the three main axes; the x-axis (red), the y-axis (green) and the z-axis (blue).
/// Used for easily debugging where objects are placed in the 3D world.
///
#[derive(Clone)]
pub struct Axes {
    model: Model<ColorMaterial>,
    aabb_local: AxisAlignedBoundingBox,
    aabb: AxisAlignedBoundingBox,
    transformation: Mat4,
}

impl Axes {
    ///
    /// Creates a new axes object consisting of three arrows with the given radius and length.
    ///
    pub fn new(context: &Context, radius: f32, length: f32) -> ThreeDResult<Self> {
        let mut mesh = CPUMesh::arrow(0.9, 0.6, 16);
        mesh.transform(&Mat4::from_nonuniform_scale(length, radius, radius));
        let model = Model::new(context, &mesh)?;
        let mut aabb = model.aabb();
        let mut aabb2 = aabb.clone();
        aabb2.transform(&Mat4::from_angle_z(degrees(90.0)));
        aabb.expand_with_aabb(&aabb2);
        let mut aabb3 = aabb.clone();
        aabb3.transform(&Mat4::from_angle_y(degrees(-90.0)));
        aabb.expand_with_aabb(&aabb3);
        Ok(Self {
            model,
            aabb: aabb.clone(),
            aabb_local: aabb,
            transformation: Mat4::identity(),
        })
    }
}

impl Geometry for Axes {
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
        let mut model = self.model.clone();
        model.render_with_material(&material, camera, lights)?;
        model.set_transformation(self.transformation * Mat4::from_angle_z(degrees(90.0)));
        model.render_with_material(&material, camera, lights)?;
        model.set_transformation(self.transformation * Mat4::from_angle_y(degrees(-90.0)));
        model.render_with_material(material, camera, lights)
    }
}
impl GeometryMut for Axes {
    fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
        let mut aabb = self.aabb_local.clone();
        aabb.transform(&self.transformation);
        self.aabb = aabb;
    }
}

impl Object for Axes {
    fn render(&self, camera: &Camera, _lights: &[&dyn Light]) -> ThreeDResult<()> {
        let mut model = self.model.clone();
        model.render_with_material(
            &ColorMaterial {
                color: Color::RED,
                ..Default::default()
            },
            camera,
            &[],
        )?;
        model.set_transformation(self.transformation * Mat4::from_angle_z(degrees(90.0)));
        model.render_with_material(
            &ColorMaterial {
                color: Color::GREEN,
                ..Default::default()
            },
            camera,
            &[],
        )?;
        model.set_transformation(self.transformation * Mat4::from_angle_y(degrees(-90.0)));
        model.render_with_material(
            &ColorMaterial {
                color: Color::BLUE,
                ..Default::default()
            },
            camera,
            &[],
        )?;
        Ok(())
    }

    fn is_transparent(&self) -> bool {
        false
    }
}
