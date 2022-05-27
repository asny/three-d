use crate::renderer::*;

///
/// Three arrows indicating the three main axes; the x-axis (red), the y-axis (green) and the z-axis (blue).
/// Used for easily debugging where objects are placed in the 3D world.
///
pub struct Axes {
    model: std::cell::RefCell<Gm<Mesh, ColorMaterial>>,
    aabb_local: AxisAlignedBoundingBox,
    aabb: AxisAlignedBoundingBox,
    transformation: Mat4,
}

impl Axes {
    ///
    /// Creates a new axes object consisting of three arrows with the given radius and length.
    ///
    pub fn new(context: &Context, radius: f32, length: f32) -> ThreeDResult<Self> {
        let mut mesh = CpuMesh::arrow(0.9, 0.6, 16);
        mesh.transform(&Mat4::from_nonuniform_scale(length, radius, radius))?;
        let model = Gm::new(Mesh::new(context, &mesh)?, ColorMaterial::default());
        let mut aabb = model.aabb();
        let mut aabb2 = aabb.clone();
        aabb2.transform(&Mat4::from_angle_z(degrees(90.0)));
        aabb.expand_with_aabb(&aabb2);
        let mut aabb3 = aabb.clone();
        aabb3.transform(&Mat4::from_angle_y(degrees(-90.0)));
        aabb.expand_with_aabb(&aabb3);
        Ok(Self {
            model: std::cell::RefCell::new(model),
            aabb: aabb.clone(),
            aabb_local: aabb,
            transformation: Mat4::identity(),
        })
    }

    ///
    /// Returns the local to world transformation applied to the axes.
    ///
    pub fn transformation(&self) -> Mat4 {
        self.transformation
    }

    ///
    /// Set the local to world transformation applied to the axes.
    /// Can be used to visualize a local coordinate system.
    ///
    pub fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
        let mut aabb = self.aabb_local.clone();
        aabb.transform(&self.transformation);
        self.aabb = aabb;
    }
}

impl Geometry for Axes {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.aabb
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        self.model
            .borrow_mut()
            .set_transformation(self.transformation);
        self.model
            .borrow()
            .render_with_material(&material, camera, lights)?;
        self.model
            .borrow_mut()
            .set_transformation(self.transformation * Mat4::from_angle_z(degrees(90.0)));
        self.model
            .borrow()
            .render_with_material(&material, camera, lights)?;
        self.model
            .borrow_mut()
            .set_transformation(self.transformation * Mat4::from_angle_y(degrees(-90.0)));
        self.model
            .borrow()
            .render_with_material(material, camera, lights)
    }
}

impl Object for Axes {
    fn render(&self, camera: &Camera, _lights: &[&dyn Light]) -> ThreeDResult<()> {
        self.model
            .borrow_mut()
            .set_transformation(self.transformation);
        self.model.borrow().render_with_material(
            &ColorMaterial {
                color: Color::RED,
                ..Default::default()
            },
            camera,
            &[],
        )?;
        self.model
            .borrow_mut()
            .set_transformation(self.transformation * Mat4::from_angle_z(degrees(90.0)));
        self.model.borrow().render_with_material(
            &ColorMaterial {
                color: Color::GREEN,
                ..Default::default()
            },
            camera,
            &[],
        )?;
        self.model
            .borrow_mut()
            .set_transformation(self.transformation * Mat4::from_angle_y(degrees(-90.0)));
        self.model.borrow().render_with_material(
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
