use crate::renderer::*;

///
/// Three arrows indicating the three main axes; the x-axis (red), the y-axis (green) and the z-axis (blue).
/// Used for easily debugging where objects are placed in the 3D world.
///
pub struct Axes {
    model: std::sync::RwLock<Gm<Mesh, ColorMaterial>>,
    aabb_local: AxisAlignedBoundingBox,
    aabb: AxisAlignedBoundingBox,
    transformation: Mat4,
}

impl Axes {
    ///
    /// Creates a new axes object consisting of three arrows with the given radius and length.
    ///
    pub fn new(context: &Context, radius: f32, length: f32) -> Self {
        let mut mesh = CpuMesh::arrow(0.9, 0.6, 16);
        mesh.transform(&Mat4::from_nonuniform_scale(length, radius, radius))
            .unwrap();
        let model = Gm::new(Mesh::new(context, &mesh), ColorMaterial::default());
        let mut aabb = model.aabb();
        let mut aabb2 = aabb.clone();
        aabb2.transform(&Mat4::from_angle_z(degrees(90.0)));
        aabb.expand_with_aabb(&aabb2);
        let mut aabb3 = aabb.clone();
        aabb3.transform(&Mat4::from_angle_y(degrees(-90.0)));
        aabb.expand_with_aabb(&aabb3);
        Self {
            model: std::sync::RwLock::new(model),
            aabb: aabb.clone(),
            aabb_local: aabb,
            transformation: Mat4::identity(),
        }
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

impl<'a> IntoIterator for &'a Axes {
    type Item = &'a dyn Object;
    type IntoIter = std::iter::Once<&'a dyn Object>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
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
    ) {
        self.model
            .write()
            .unwrap()
            .set_transformation(self.transformation);
        self.model
            .read()
            .unwrap()
            .render_with_material(material, camera, lights);
        self.model
            .write()
            .unwrap()
            .set_transformation(self.transformation * Mat4::from_angle_z(degrees(90.0)));
        self.model
            .read()
            .unwrap()
            .render_with_material(material, camera, lights);
        self.model
            .write()
            .unwrap()
            .set_transformation(self.transformation * Mat4::from_angle_y(degrees(-90.0)));
        self.model
            .read()
            .unwrap()
            .render_with_material(material, camera, lights);
    }

    fn render_with_post_material(
        &self,
        material: &dyn PostMaterial,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        self.model
            .write()
            .unwrap()
            .set_transformation(self.transformation);
        self.model.read().unwrap().render_with_post_material(
            material,
            camera,
            lights,
            color_texture,
            depth_texture,
        );
        self.model
            .write()
            .unwrap()
            .set_transformation(self.transformation * Mat4::from_angle_z(degrees(90.0)));
        self.model.read().unwrap().render_with_post_material(
            material,
            camera,
            lights,
            color_texture,
            depth_texture,
        );
        self.model
            .write()
            .unwrap()
            .set_transformation(self.transformation * Mat4::from_angle_y(degrees(-90.0)));
        self.model.read().unwrap().render_with_post_material(
            material,
            camera,
            lights,
            color_texture,
            depth_texture,
        );
    }
}

impl Object for Axes {
    fn render(&self, camera: &Camera, _lights: &[&dyn Light]) {
        self.model
            .write()
            .unwrap()
            .set_transformation(self.transformation);
        self.model.read().unwrap().render_with_material(
            &ColorMaterial {
                color: Color::RED,
                ..Default::default()
            },
            camera,
            &[],
        );
        self.model
            .write()
            .unwrap()
            .set_transformation(self.transformation * Mat4::from_angle_z(degrees(90.0)));
        self.model.read().unwrap().render_with_material(
            &ColorMaterial {
                color: Color::GREEN,
                ..Default::default()
            },
            camera,
            &[],
        );
        self.model
            .write()
            .unwrap()
            .set_transformation(self.transformation * Mat4::from_angle_y(degrees(-90.0)));
        self.model.read().unwrap().render_with_material(
            &ColorMaterial {
                color: Color::BLUE,
                ..Default::default()
            },
            camera,
            &[],
        );
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}
