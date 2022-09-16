use crate::core::*;
use crate::renderer::*;
const PATCH_SIZE: f32 = 16.0;
const PATCHES_PER_SIDE: u32 = 33;
const HALF_PATCHES_PER_SIDE: i32 = (PATCHES_PER_SIDE as i32 - 1) / 2;
const VERTICES_PER_UNIT: usize = 4;
const VERTICES_PER_SIDE: usize = (PATCH_SIZE + 1.0) as usize * VERTICES_PER_UNIT;
const VERTEX_DISTANCE: f32 = 1.0 / VERTICES_PER_UNIT as f32;

pub struct Terrain<M: Material> {
    context: Context,
    center: (i32, i32),
    patches: Vec<Gm<TerrainPatch, M>>,
    material: M,
}
impl<M: Material + Clone> Terrain<M> {
    pub fn new(
        context: &Context,
        material: M,
        height_map: &impl Fn(f32, f32) -> f32,
        position: Vec3,
    ) -> Self {
        let mut t = Self {
            context: context.clone(),
            center: Self::pos2patch(position),
            patches: Vec::new(),
            material: material.clone(),
        };
        for ix in t.center.0 - HALF_PATCHES_PER_SIDE..t.center.0 + HALF_PATCHES_PER_SIDE + 1 {
            for iy in t.center.1 - HALF_PATCHES_PER_SIDE..t.center.1 + HALF_PATCHES_PER_SIDE + 1 {
                let patch = TerrainPatch::new(context, height_map, ix, iy);
                t.patches.push(Gm::new(patch, material.clone()));
            }
        }
        t.update(height_map, position);
        t
    }

    pub fn update(&mut self, height_map: &impl Fn(f32, f32) -> f32, position: Vec3) {
        let (x0, y0) = Self::pos2patch(position);

        while x0 > self.center.0 {
            self.center.0 += 1;
            for iy in
                self.center.1 - HALF_PATCHES_PER_SIDE..self.center.1 + HALF_PATCHES_PER_SIDE + 1
            {
                self.patches.push(Gm::new(
                    TerrainPatch::new(
                        &self.context,
                        height_map,
                        self.center.0 + HALF_PATCHES_PER_SIDE,
                        iy,
                    ),
                    self.material.clone(),
                ));
            }
        }

        while x0 < self.center.0 {
            self.center.0 -= 1;
            for iy in
                self.center.1 - HALF_PATCHES_PER_SIDE..self.center.1 + HALF_PATCHES_PER_SIDE + 1
            {
                self.patches.push(Gm::new(
                    TerrainPatch::new(
                        &self.context,
                        height_map,
                        self.center.0 - HALF_PATCHES_PER_SIDE,
                        iy,
                    ),
                    self.material.clone(),
                ));
            }
        }
        while y0 > self.center.1 {
            self.center.1 += 1;
            for ix in
                self.center.0 - HALF_PATCHES_PER_SIDE..self.center.0 + HALF_PATCHES_PER_SIDE + 1
            {
                self.patches.push(Gm::new(
                    TerrainPatch::new(
                        &self.context,
                        height_map,
                        ix,
                        self.center.1 + HALF_PATCHES_PER_SIDE,
                    ),
                    self.material.clone(),
                ));
            }
        }

        while y0 < self.center.1 {
            self.center.1 -= 1;
            for ix in
                self.center.0 - HALF_PATCHES_PER_SIDE..self.center.0 + HALF_PATCHES_PER_SIDE + 1
            {
                self.patches.push(Gm::new(
                    TerrainPatch::new(
                        &self.context,
                        height_map,
                        ix,
                        self.center.1 - HALF_PATCHES_PER_SIDE,
                    ),
                    self.material.clone(),
                ));
            }
        }

        self.patches.retain(|p| {
            let (ix, iy) = p.index();
            (x0 - ix).abs() <= HALF_PATCHES_PER_SIDE && (y0 - iy).abs() <= HALF_PATCHES_PER_SIDE
        });
    }

    fn pos2patch(position: Vec3) -> (i32, i32) {
        (
            (position.x / PATCH_SIZE).floor() as i32,
            (position.z / PATCH_SIZE).floor() as i32,
        )
    }

    ///
    /// Returns an iterator over the reference to the objects in this model which can be used as input to a render function, for example [RenderTarget::render].
    ///
    pub fn obj_iter(&self) -> impl Iterator<Item = &dyn Object> + Clone {
        self.patches.iter().map(|m| m as &dyn Object)
    }

    ///
    /// Returns an iterator over the reference to the geometries in this model which can be used as input to for example [pick], [RenderTarget::render_with_material] or [DirectionalLight::generate_shadow_map].
    ///
    pub fn geo_iter(&self) -> impl Iterator<Item = &dyn Geometry> + Clone {
        self.patches.iter().map(|m| m as &dyn Geometry)
    }
}
