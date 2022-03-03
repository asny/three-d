use crate::core::*;

///
/// An array of indices. Supports different data types.
///
pub enum Indices {
    /// Uses unsigned 8 bit integer for each index.
    U8(Vec<u8>),
    /// Uses unsigned 16 bit integer for each index.
    U16(Vec<u16>),
    /// Uses unsigned 32 bit integer for each index.
    U32(Vec<u32>),
}

impl Indices {
    ///
    /// Returns all the indices as an `u32` data type. Clones all of the indices, so do not use it too often.
    ///
    pub fn into_u32(&self) -> Vec<u32> {
        match self {
            Self::U8(ind) => ind.iter().map(|i| *i as u32).collect::<Vec<u32>>(),
            Self::U16(ind) => ind.iter().map(|i| *i as u32).collect::<Vec<u32>>(),
            Self::U32(ind) => ind.clone(),
        }
    }
}

impl std::fmt::Debug for Indices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Indices");
        match self {
            Self::U8(ind) => d.field("u8", &ind.len()),
            Self::U16(ind) => d.field("u16", &ind.len()),
            Self::U32(ind) => d.field("u32", &ind.len()),
        };
        d.finish()
    }
}

/// See [CpuMesh]
#[deprecated = "Renamed to CpuMesh"]
pub type CPUMesh = CpuMesh;

///
/// A CPU-side version of a triangle mesh.
/// Can be constructed manually or loaded via [io](crate::io)
/// or via the utility functions for generating simple triangle meshes.
///
#[derive(Default)]
pub struct CpuMesh {
    /// Name.
    pub name: String,
    /// Name of the associated material, use this to match with [CpuMaterial::name].
    pub material_name: Option<String>,
    /// The positions of the vertices.
    /// If there is no indices associated with this mesh, three contiguous positions defines a triangle, in that case, the length must be divisable by 3.
    pub positions: Vec<Vec3>,
    /// The indices into the positions, normals, uvs and colors arrays which defines the three vertices of a triangle. Three contiguous indices defines a triangle, therefore the length must be divisable by 3.
    pub indices: Option<Indices>,
    /// The normals of the vertices.
    pub normals: Option<Vec<Vec3>>,
    /// The tangents of the vertices, orthogonal direction to the normal.
    /// The fourth value specifies the handedness (either -1.0 or 1.0).
    pub tangents: Option<Vec<Vec4>>,
    /// The uv coordinates of the vertices.
    pub uvs: Option<Vec<Vec2>>,
    /// The colors of the vertices.
    /// The colors are assumed to be in linear space.
    pub colors: Option<Vec<Color>>,
}

impl std::fmt::Debug for CpuMesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("CpuMesh");
        d.field("name", &self.name);
        d.field("material name", &self.material_name);
        d.field("positions", &self.positions.len());
        d.field("indices", &self.indices);
        d.field("normals", &self.normals.as_ref().map(|v| v.len()));
        d.field("tangents", &self.tangents.as_ref().map(|v| v.len()));
        d.field("uvs", &self.uvs.as_ref().map(|v| v.len()));
        d.field("colors", &self.colors.as_ref().map(|v| v.len()));
        d.finish()
    }
}

impl CpuMesh {
    ///
    /// Returns the material for this mesh in the given list of materials. Returns `None` if no suitable material can be found.
    ///
    pub fn material<'a>(&self, materials: &'a [CpuMaterial]) -> Option<&'a CpuMaterial> {
        self.material_name.as_ref().and_then(|name| {
            materials
                .iter()
                .position(|mat| &mat.name == name)
                .map(|index| &materials[index])
        })
    }

    ///
    /// Transforms the mesh by the given transformation.
    ///
    pub fn transform(&mut self, transform: &Mat4) {
        for pos in self.positions.iter_mut() {
            *pos = (transform * pos.extend(1.0)).truncate();
        }
        let normal_transform = transform.invert().unwrap().transpose();

        if let Some(ref mut normals) = self.normals {
            for n in normals.iter_mut() {
                *n = (normal_transform * n.extend(1.0)).truncate();
            }
        }

        if let Some(ref mut tangents) = self.tangents {
            for t in tangents.iter_mut() {
                *t = (normal_transform * t.truncate().extend(1.0))
                    .truncate()
                    .extend(t.w);
            }
        }
    }

    ///
    /// Returns a square mesh spanning the xy-plane with positions in the range `[-1..1]` in the x and y axes.
    ///
    pub fn square() -> Self {
        let indices = vec![0u8, 1, 2, 2, 3, 0];
        let halfsize = 1.0;
        let positions = vec![
            vec3(-halfsize, -halfsize, 0.0),
            vec3(halfsize, -halfsize, 0.0),
            vec3(halfsize, halfsize, 0.0),
            vec3(-halfsize, halfsize, 0.0),
        ];
        let normals = vec![
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 0.0, 1.0),
        ];
        let tangents = vec![
            vec4(1.0, 0.0, 0.0, 1.0),
            vec4(1.0, 0.0, 0.0, 1.0),
            vec4(1.0, 0.0, 0.0, 1.0),
            vec4(1.0, 0.0, 0.0, 1.0),
        ];
        let uvs = vec![
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(1.0, 1.0),
            vec2(0.0, 1.0),
        ];
        CpuMesh {
            name: "square".to_string(),
            indices: Some(Indices::U8(indices)),
            positions,
            normals: Some(normals),
            tangents: Some(tangents),
            uvs: Some(uvs),
            ..Default::default()
        }
    }

    ///
    /// Returns a circle mesh spanning the xy-plane with radius 1 and center in `(0, 0, 0)`.
    ///
    pub fn circle(angle_subdivisions: u32) -> Self {
        let mut positions = Vec::new();
        let mut indices = Vec::new();
        let mut normals = Vec::new();
        for j in 0..angle_subdivisions {
            let angle = 2.0 * std::f32::consts::PI * j as f32 / angle_subdivisions as f32;

            positions.push(vec3(angle.cos(), angle.sin(), 0.0));
            normals.push(vec3(0.0, 0.0, 1.0));
        }

        for j in 0..angle_subdivisions {
            indices.push(0);
            indices.push(j as u16);
            indices.push(((j + 1) % angle_subdivisions) as u16);
        }
        CpuMesh {
            name: "circle".to_string(),
            indices: Some(Indices::U16(indices)),
            positions,
            normals: Some(normals),
            ..Default::default()
        }
    }

    ///
    /// Returns a sphere mesh with radius 1 and center in `(0, 0, 0)`.
    ///
    pub fn sphere(angle_subdivisions: u32) -> Self {
        let mut positions = Vec::new();
        let mut indices = Vec::new();
        let mut normals = Vec::new();

        positions.push(vec3(0.0, 0.0, 1.0));
        normals.push(vec3(0.0, 0.0, 1.0));

        for j in 0..angle_subdivisions * 2 {
            let j1 = (j + 1) % (angle_subdivisions * 2);
            indices.push(0);
            indices.push((1 + j) as u16);
            indices.push((1 + j1) as u16);
        }

        for i in 0..angle_subdivisions - 1 {
            let theta = std::f32::consts::PI * (i + 1) as f32 / angle_subdivisions as f32;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();
            let i0 = 1 + i * angle_subdivisions * 2;
            let i1 = 1 + (i + 1) * angle_subdivisions * 2;

            for j in 0..angle_subdivisions * 2 {
                let phi = std::f32::consts::PI * j as f32 / angle_subdivisions as f32;
                let x = sin_theta * phi.cos();
                let y = sin_theta * phi.sin();
                let z = cos_theta;
                positions.push(vec3(x, y, z));
                normals.push(vec3(x, y, z));

                if i != angle_subdivisions - 2 {
                    let j1 = (j + 1) % (angle_subdivisions * 2);
                    indices.push((i0 + j) as u16);
                    indices.push((i1 + j1) as u16);
                    indices.push((i0 + j1) as u16);
                    indices.push((i1 + j1) as u16);
                    indices.push((i0 + j) as u16);
                    indices.push((i1 + j) as u16);
                }
            }
        }
        positions.push(vec3(0.0, 0.0, -1.0));
        normals.push(vec3(0.0, 0.0, -1.0));

        let i = 1 + (angle_subdivisions - 2) * angle_subdivisions * 2;
        for j in 0..angle_subdivisions * 2 {
            let j1 = (j + 1) % (angle_subdivisions * 2);
            indices.push((i + j) as u16);
            indices.push(((angle_subdivisions - 1) * angle_subdivisions * 2 + 1) as u16);
            indices.push((i + j1) as u16);
        }

        CpuMesh {
            name: "sphere".to_string(),
            indices: Some(Indices::U16(indices)),
            positions,
            normals: Some(normals),
            ..Default::default()
        }
    }

    ///
    /// Returns an axis aligned unconnected cube mesh with positions in the range `[-1..1]` in all axes.
    ///
    pub fn cube() -> Self {
        let positions = vec![
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
        ];
        let uvs = vec![
            vec2(1.0, 0.0),
            vec2(0.0, 0.0),
            vec2(1.0, 1.0),
            vec2(0.0, 1.0),
            vec2(1.0, 1.0),
            vec2(0.0, 0.0),
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(1.0, 1.0),
            vec2(1.0, 1.0),
            vec2(0.0, 1.0),
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(0.0, 0.0),
            vec2(1.0, 1.0),
            vec2(0.0, 1.0),
            vec2(1.0, 1.0),
            vec2(0.0, 0.0),
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(1.0, 1.0),
            vec2(1.0, 1.0),
            vec2(0.0, 1.0),
            vec2(0.0, 0.0),
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(1.0, 1.0),
            vec2(1.0, 1.0),
            vec2(0.0, 1.0),
            vec2(0.0, 0.0),
            vec2(1.0, 0.0),
            vec2(0.0, 0.0),
            vec2(1.0, 1.0),
            vec2(0.0, 1.0),
            vec2(1.0, 1.0),
            vec2(0.0, 0.0),
        ];
        let mut mesh = CpuMesh {
            positions,
            uvs: Some(uvs),
            ..Default::default()
        };
        mesh.compute_normals();
        mesh.compute_tangents().unwrap();
        mesh
    }

    ///
    /// Returns a cylinder mesh around the x-axis in the range `[0..1]` and with radius 1.
    ///
    pub fn cylinder(angle_subdivisions: u32) -> Self {
        let length_subdivisions = 1;
        let mut positions = Vec::new();
        let mut indices = Vec::new();
        for i in 0..length_subdivisions + 1 {
            let x = i as f32 / length_subdivisions as f32;
            for j in 0..angle_subdivisions {
                let angle = 2.0 * std::f32::consts::PI * j as f32 / angle_subdivisions as f32;

                positions.push(vec3(x, angle.cos(), angle.sin()));
            }
        }
        for i in 0..length_subdivisions {
            for j in 0..angle_subdivisions {
                indices.push((i * angle_subdivisions + j) as u16);
                indices.push((i * angle_subdivisions + (j + 1) % angle_subdivisions) as u16);
                indices.push(((i + 1) * angle_subdivisions + (j + 1) % angle_subdivisions) as u16);

                indices.push((i * angle_subdivisions + j) as u16);
                indices.push(((i + 1) * angle_subdivisions + (j + 1) % angle_subdivisions) as u16);
                indices.push(((i + 1) * angle_subdivisions + j) as u16);
            }
        }
        let mut mesh = Self {
            name: "cylinder".to_string(),
            positions,
            indices: Some(Indices::U16(indices)),
            ..Default::default()
        };
        mesh.compute_normals();
        mesh
    }

    ///
    /// Returns a cone mesh around the x-axis in the range `[0..1]` and with radius 1 at -1.0.
    ///
    pub fn cone(angle_subdivisions: u32) -> Self {
        let length_subdivisions = 1;
        let mut positions = Vec::new();
        let mut indices = Vec::new();
        for i in 0..length_subdivisions + 1 {
            let x = i as f32 / length_subdivisions as f32;
            for j in 0..angle_subdivisions {
                let angle = 2.0 * std::f32::consts::PI * j as f32 / angle_subdivisions as f32;

                positions.push(vec3(x, angle.cos() * (1.0 - x), angle.sin() * (1.0 - x)));
            }
        }
        for i in 0..length_subdivisions {
            for j in 0..angle_subdivisions {
                indices.push((i * angle_subdivisions + j) as u16);
                indices.push((i * angle_subdivisions + (j + 1) % angle_subdivisions) as u16);
                indices.push(((i + 1) * angle_subdivisions + (j + 1) % angle_subdivisions) as u16);

                indices.push((i * angle_subdivisions + j) as u16);
                indices.push(((i + 1) * angle_subdivisions + (j + 1) % angle_subdivisions) as u16);
                indices.push(((i + 1) * angle_subdivisions + j) as u16);
            }
        }
        let mut mesh = Self {
            name: "cone".to_string(),
            positions,
            indices: Some(Indices::U16(indices)),
            ..Default::default()
        };
        mesh.compute_normals();
        mesh
    }

    ///
    /// Returns an arrow mesh around the x-axis in the range `[0..1]` and with radius 1.
    /// The tail length and radius should be in the range `]0..1[`.
    ///
    pub fn arrow(tail_length: f32, tail_radius: f32, angle_subdivisions: u32) -> Self {
        let mut arrow = Self::cylinder(angle_subdivisions);
        arrow.transform(&Mat4::from_nonuniform_scale(
            tail_length,
            tail_radius,
            tail_radius,
        ));
        arrow.name = "arrow".to_string();
        let mut cone = Self::cone(angle_subdivisions);
        cone.transform(
            &(Mat4::from_translation(vec3(tail_length, 0.0, 0.0))
                * Mat4::from_nonuniform_scale(1.0 - tail_length, 1.0, 1.0)),
        );
        let mut indices = arrow.indices.unwrap().into_u32();
        let cone_indices = cone.indices.unwrap().into_u32();
        let offset = indices.iter().max().unwrap() + 1;
        indices.extend(cone_indices.iter().map(|i| i + offset));
        arrow.indices = Some(Indices::U16(indices.iter().map(|i| *i as u16).collect()));

        arrow.positions.extend(cone.positions);
        arrow
            .normals
            .as_mut()
            .unwrap()
            .extend(cone.normals.as_ref().unwrap());
        arrow
    }

    ///
    /// Computes the per vertex normals and updates the normals of the mesh.
    /// It will override the current normals if they already exist.
    ///
    pub fn compute_normals(&mut self) {
        let mut normals = vec![vec3(0.0, 0.0, 0.0); self.positions.len()];
        self.for_each_triangle(|i0, i1, i2| {
            let p0 = self.positions[i0];
            let p1 = self.positions[i1];
            let p2 = self.positions[i2];
            let normal = (p1 - p0).cross(p2 - p0);
            normals[i0] += normal;
            normals[i1] += normal;
            normals[i2] += normal;
        });

        for n in normals.iter_mut() {
            *n = n.normalize();
        }
        self.normals = Some(normals);
    }

    ///
    /// Computes the per vertex tangents and updates the tangents of the mesh.
    /// It will override the current tangents if they already exist.
    ///
    pub fn compute_tangents(&mut self) -> ThreeDResult<()> {
        if self.normals.is_none() || self.uvs.is_none() {
            Err(CoreError::FailedComputingTangents)?;
        }
        let mut tan1 = vec![vec3(0.0, 0.0, 0.0); self.positions.len()];
        let mut tan2 = vec![vec3(0.0, 0.0, 0.0); self.positions.len()];

        self.for_each_triangle(|i0, i1, i2| {
            let a = self.positions[i0];
            let b = self.positions[i1];
            let c = self.positions[i2];
            let uva = self.uvs.as_ref().unwrap()[i0];
            let uvb = self.uvs.as_ref().unwrap()[i1];
            let uvc = self.uvs.as_ref().unwrap()[i2];

            let ba = b - a;
            let ca = c - a;

            let uvba = uvb - uva;
            let uvca = uvc - uva;

            let d = uvba.x * uvca.y - uvca.x * uvba.y;
            if d.abs() > 0.00001 {
                let r = 1.0 / d;
                let sdir = (ba * uvca.y - ca * uvba.y) * r;
                let tdir = (ca * uvba.x - ba * uvca.x) * r;
                tan1[i0] += sdir;
                tan1[i1] += sdir;
                tan1[i2] += sdir;
                tan2[i0] += tdir;
                tan2[i1] += tdir;
                tan2[i2] += tdir;
            }
        });

        let mut tangents = vec![vec4(0.0, 0.0, 0.0, 0.0); self.positions.len()];
        self.for_each_vertex(|index| {
            let normal = self.normals.as_ref().unwrap()[index];
            let t = tan1[index];
            let tangent = (t - normal * normal.dot(t)).normalize();
            let handedness = if normal.cross(tangent).dot(tan2[index]) < 0.0 {
                1.0
            } else {
                -1.0
            };
            tangents[index] = tangent.extend(handedness);
        });

        self.tangents = Some(tangents);
        Ok(())
    }

    ///
    ///  Iterates over all vertices in this mesh and calls the callback function with the index for each vertex.
    ///
    pub fn for_each_vertex(&self, mut callback: impl FnMut(usize)) {
        for i in 0..self.positions.len() {
            callback(i);
        }
    }

    ///
    /// Iterates over all triangles in this mesh and calls the callback function with the three indices, one for each vertex in the triangle.
    ///
    pub fn for_each_triangle(&self, mut callback: impl FnMut(usize, usize, usize)) {
        match self.indices {
            Some(Indices::U8(ref indices)) => {
                for face in 0..indices.len() / 3 {
                    let index0 = indices[face * 3] as usize;
                    let index1 = indices[face * 3 + 1] as usize;
                    let index2 = indices[face * 3 + 2] as usize;
                    callback(index0, index1, index2);
                }
            }
            Some(Indices::U16(ref indices)) => {
                for face in 0..indices.len() / 3 {
                    let index0 = indices[face * 3] as usize;
                    let index1 = indices[face * 3 + 1] as usize;
                    let index2 = indices[face * 3 + 2] as usize;
                    callback(index0, index1, index2);
                }
            }
            Some(Indices::U32(ref indices)) => {
                for face in 0..indices.len() / 3 {
                    let index0 = indices[face * 3] as usize;
                    let index1 = indices[face * 3 + 1] as usize;
                    let index2 = indices[face * 3 + 2] as usize;
                    callback(index0, index1, index2);
                }
            }
            None => {
                for face in 0..self.positions.len() / 3 {
                    callback(face * 3, face * 3 + 1, face * 3 + 2);
                }
            }
        }
    }

    ///
    /// Computes the axis aligned bounding box of the mesh.
    ///
    pub fn compute_aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::new_with_positions(&self.positions)
    }

    ///
    /// Returns an error if the mesh is not valid.
    ///
    pub fn validate(&self) -> ThreeDResult<()> {
        let vertex_count = if let Some(ref indices) = self.indices {
            let index_count = match indices {
                Indices::U8(ind) => ind.len(),
                Indices::U16(ind) => ind.len(),
                Indices::U32(ind) => ind.len(),
            };
            if index_count % 3 != 0 {
                Err(CoreError::InvalidNumberOfVertices(index_count))?;
            }
            match indices {
                Indices::U8(ind) => ind.iter().max().map(|m| m + 1).unwrap_or(0) as usize,
                Indices::U16(ind) => ind.iter().max().map(|m| m + 1).unwrap_or(0) as usize,
                Indices::U32(ind) => ind.iter().max().map(|m| m + 1).unwrap_or(0) as usize,
            }
        } else {
            if self.positions.len() % 3 != 0 {
                Err(CoreError::InvalidBufferLength(
                    "position".to_string(),
                    (self.positions.len() as f32 / 3.0).ceil() as usize * 3,
                    self.positions.len(),
                ))?;
            }
            let vertex_count = self.positions.len() / 3;
            if vertex_count % 3 != 0 {
                Err(CoreError::InvalidNumberOfVertices(vertex_count))?;
            }
            vertex_count
        };
        let buffer_check = |length: Option<usize>, name: &str, size| -> ThreeDResult<()> {
            if let Some(length) = length {
                if length != size * vertex_count {
                    Err(CoreError::InvalidBufferLength(
                        name.to_string(),
                        size * vertex_count,
                        length,
                    ))?;
                }
            }
            Ok(())
        };

        buffer_check(Some(self.positions.len()), "position", 3)?;
        buffer_check(self.normals.as_ref().map(|b| b.len()), "normal", 3)?;
        buffer_check(self.tangents.as_ref().map(|b| b.len()), "tangent", 4)?;
        buffer_check(self.colors.as_ref().map(|b| b.len()), "color", 4)?;
        buffer_check(self.uvs.as_ref().map(|b| b.len()), "uv coordinate", 2)?;

        Ok(())
    }
}
