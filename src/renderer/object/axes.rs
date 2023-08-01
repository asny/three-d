use crate::renderer::*;

///
/// Three arrows indicating the three main axes; the x-axis (red), the y-axis (green) and the z-axis (blue).
/// Used for easily debugging where objects are placed in the 3D world.
///
pub struct Axes {
    model: Gm<InstancedMesh, ColorMaterial>,
}

impl Axes {
    ///
    /// Creates a new axes object consisting of three arrows with the given radius and length.
    ///
    pub fn new(context: &Context, radius: f32, length: f32) -> Self {
        let mut cpu_mesh = CpuMesh::arrow(0.9, 0.6, 16);
        cpu_mesh
            .transform(&Mat4::from_nonuniform_scale(length, radius, radius))
            .unwrap();
        let model = Gm::new(
            InstancedMesh::new(
                context,
                &Instances {
                    transformations: vec![
                        Mat4::identity(),
                        Mat4::from_angle_z(degrees(90.0)),
                        Mat4::from_angle_y(degrees(-90.0)),
                    ],
                    texture_transformations: None,
                    colors: Some(vec![Srgba::RED, Srgba::GREEN, Srgba::BLUE]),
                },
                &cpu_mesh,
            ),
            ColorMaterial::default(),
        );
        Self { model }
    }
}

impl<'a> IntoIterator for &'a Axes {
    type Item = &'a dyn Object;
    type IntoIter = std::iter::Once<&'a dyn Object>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

use std::ops::Deref;
impl Deref for Axes {
    type Target = Gm<InstancedMesh, ColorMaterial>;
    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

impl std::ops::DerefMut for Axes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.model
    }
}

impl Geometry for Axes {
    impl_geometry_body!(deref);

    fn animate(&mut self, time: f32) {
        self.model.animate(time)
    }
}

impl Object for Axes {
    impl_object_body!(deref);
}
