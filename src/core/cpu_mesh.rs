use crate::core::*;

///
/// An array of indices. Supports different data types.
///
#[derive(Debug)]
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

///
/// A CPU-side version of a triangle mesh.
/// Can be constructed manually or loaded via [io](crate::io)
/// or via the utility functions for generating simple triangle meshes.
///
#[derive(Default, Debug)]
pub struct CPUMesh {
    /// Name.
    pub name: String,
    /// Name of the associated material, use this to match with [CPUMaterial::name].
    pub material_name: Option<String>,
    /// The positions of the vertices. Three contiguous floats defines a 3D position `(x, y, z)`, therefore the length must be divisable by 3.
    /// If there is no indices associated with this mesh, three contiguous positions defines a triangle, in that case, the length must also be divisable by 9.
    pub positions: Vec<f32>,
    /// The indices into the positions, normals, uvs and colors arrays which defines the three vertices of a triangle. Three contiguous indices defines a triangle, therefore the length must be divisable by 3.
    pub indices: Option<Indices>,
    /// The normals of the vertices. Three contiguous floats defines a normal `(x, y, z)`, therefore the length must be divisable by 3.
    pub normals: Option<Vec<f32>>,
    pub tangents: Option<Vec<f32>>,
    /// The uv coordinates of the vertices. Two contiguous floats defines a coordinate `(u, v)`, therefore the length must be divisable by 2.
    pub uvs: Option<Vec<f32>>,
    /// The colors of the vertices. Four contiguous bytes defines a color `(r, g, b, a)`, therefore the length must be divisable by 4.
    /// The colors are assumed to be in linear space.
    pub colors: Option<Vec<u8>>,
}

impl CPUMesh {
    ///
    /// Returns the material for this mesh in the given list of materials. Returns `None` if no suitable material can be found.
    ///
    pub fn material<'a>(&self, materials: &'a [CPUMaterial]) -> Option<&'a CPUMaterial> {
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
        for i in 0..self.positions.len() / 3 {
            let p = (transform
                * vec4(
                    self.positions[i * 3],
                    self.positions[i * 3 + 1],
                    self.positions[i * 3 + 2],
                    1.0,
                ))
            .truncate();
            self.positions[i * 3] = p.x;
            self.positions[i * 3 + 1] = p.y;
            self.positions[i * 3 + 2] = p.z;
        }
        let normal_transform = transform.invert().unwrap().transpose();

        if let Some(ref mut normals) = self.normals {
            for i in 0..normals.len() / 3 {
                let p = (normal_transform
                    * vec4(normals[i * 3], normals[i * 3 + 1], normals[i * 3 + 2], 1.0))
                .truncate();
                normals[i * 3] = p.x;
                normals[i * 3 + 1] = p.y;
                normals[i * 3 + 2] = p.z;
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
            -halfsize, -halfsize, 0.0, halfsize, -halfsize, 0.0, halfsize, halfsize, 0.0,
            -halfsize, halfsize, 0.0,
        ];
        let normals = vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
        let uvs = vec![0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
        CPUMesh {
            name: "square".to_string(),
            indices: Some(Indices::U8(indices)),
            positions,
            normals: Some(normals),
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

            positions.push(angle.cos());
            positions.push(angle.sin());
            positions.push(0.0);

            normals.push(0.0);
            normals.push(0.0);
            normals.push(1.0);
        }

        for j in 0..angle_subdivisions {
            indices.push(0);
            indices.push(j as u16);
            indices.push(((j + 1) % angle_subdivisions) as u16);
        }
        CPUMesh {
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

        positions.push(0.0);
        positions.push(0.0);
        positions.push(1.0);

        normals.push(0.0);
        normals.push(0.0);
        normals.push(1.0);

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
                positions.push(x);
                positions.push(y);
                positions.push(z);

                normals.push(x);
                normals.push(y);
                normals.push(z);

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
        positions.push(0.0);
        positions.push(0.0);
        positions.push(-1.0);

        normals.push(0.0);
        normals.push(0.0);
        normals.push(-1.0);

        let i = 1 + (angle_subdivisions - 2) * angle_subdivisions * 2;
        for j in 0..angle_subdivisions * 2 {
            let j1 = (j + 1) % (angle_subdivisions * 2);
            indices.push((i + j) as u16);
            indices.push(((angle_subdivisions - 1) * angle_subdivisions * 2 + 1) as u16);
            indices.push((i + j1) as u16);
        }

        CPUMesh {
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
            1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0,
            1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0,
            -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0,
            1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0,
            -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0,
        ];
        let uvs = vec![
            1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0,
            0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 0.0, 0.0,
        ];
        let mut mesh = CPUMesh {
            positions,
            uvs: Some(uvs),
            ..Default::default()
        };
        mesh.compute_normals();
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

                positions.push(x);
                positions.push(angle.cos());
                positions.push(angle.sin());
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

                positions.push(x);
                positions.push(angle.cos() * (1.0 - x));
                positions.push(angle.sin() * (1.0 - x));
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
        self.normals = Some(
            self.indices
                .as_ref()
                .map(|indices| compute_normals_with_indices(&indices.into_u32(), &self.positions))
                .unwrap_or_else(|| compute_normals(&self.positions)),
        );
    }

    pub fn compute_tangents(&mut self) {
        self.tangents = Some(
            self.indices
                .as_ref()
                .map(|indices| compute_tangents_with_indices(&indices.into_u32(), &self.positions))
                .unwrap_or_else(|| compute_tangents(&self.positions)),
        );
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
        if let Some(ref indices) = self.indices {
            let index_count = match indices {
                Indices::U8(ind) => ind.len(),
                Indices::U16(ind) => ind.len(),
                Indices::U32(ind) => ind.len(),
            };
            if index_count % 3 != 0 {
                Err(CoreError::InvalidBufferLength(
                    "index".to_string(),
                    index_count,
                ))?;
            }
            if self.positions.len() % 3 != 0 {
                Err(CoreError::InvalidBufferLength(
                    "position".to_string(),
                    index_count,
                ))?;
            }
            if let Some(ref data) = self.normals {
                if data.len() % 3 != 0 {
                    Err(CoreError::InvalidBufferLength(
                        "normal".to_string(),
                        index_count,
                    ))?;
                }
            }
            if let Some(ref data) = self.colors {
                if data.len() % 4 != 0 {
                    Err(CoreError::InvalidBufferLength(
                        "color".to_string(),
                        index_count,
                    ))?;
                }
            }
            if let Some(ref data) = self.uvs {
                if data.len() % 2 != 0 {
                    Err(CoreError::InvalidBufferLength(
                        "uv coordinate".to_string(),
                        index_count,
                    ))?;
                }
            }
            if cfg!(debug) {
                let indices_valid = match indices {
                    Indices::U8(ind) => {
                        let len = self.positions.len();
                        ind.iter().all(|&i| (i as usize) < len)
                    }
                    Indices::U16(ind) => {
                        let len = self.positions.len();
                        ind.iter().all(|&i| (i as usize) < len)
                    }
                    Indices::U32(ind) => {
                        let len = self.positions.len();
                        ind.iter().all(|&i| (i as usize) < len)
                    }
                };
                if !indices_valid {
                    Err(CoreError::InvalidIndexBuffer(self.positions.len()))?;
                }
            }
        } else {
            if self.positions.len() % 9 != 0 {
                Err(CoreError::InvalidPositionBuffer(self.positions.len()))?;
            }
        };
        Ok(())
    }
}

fn compute_normals_with_indices(indices: &[u32], positions: &[f32]) -> Vec<f32> {
    let mut normals = vec![0.0f32; positions.len() * 3];
    for face in 0..indices.len() / 3 {
        let index0 = indices[face * 3] as usize;
        let p0 = vec3(
            positions[index0 * 3],
            positions[index0 * 3 + 1],
            positions[index0 * 3 + 2],
        );
        let index1 = indices[face * 3 + 1] as usize;
        let p1 = vec3(
            positions[index1 * 3],
            positions[index1 * 3 + 1],
            positions[index1 * 3 + 2],
        );
        let index2 = indices[face * 3 + 2] as usize;
        let p2 = vec3(
            positions[index2 * 3],
            positions[index2 * 3 + 1],
            positions[index2 * 3 + 2],
        );

        let normal = (p1 - p0).cross(p2 - p0);
        normals[index0 * 3] += normal.x;
        normals[index0 * 3 + 1] += normal.y;
        normals[index0 * 3 + 2] += normal.z;
        normals[index1 * 3] += normal.x;
        normals[index1 * 3 + 1] += normal.y;
        normals[index1 * 3 + 2] += normal.z;
        normals[index2 * 3] += normal.x;
        normals[index2 * 3 + 1] += normal.y;
        normals[index2 * 3 + 2] += normal.z;
    }

    for i in 0..normals.len() / 3 {
        let normal = vec3(normals[3 * i], normals[3 * i + 1], normals[3 * i + 2]).normalize();
        normals[3 * i] = normal.x;
        normals[3 * i + 1] = normal.y;
        normals[3 * i + 2] = normal.z;
    }
    normals
}

fn compute_normals(positions: &[f32]) -> Vec<f32> {
    let mut normals = vec![0.0f32; positions.len()];
    for face in 0..positions.len() / 9 {
        let index0 = face * 3 as usize;
        let p0 = vec3(
            positions[index0 * 3],
            positions[index0 * 3 + 1],
            positions[index0 * 3 + 2],
        );
        let index1 = face * 3 + 1 as usize;
        let p1 = vec3(
            positions[index1 * 3],
            positions[index1 * 3 + 1],
            positions[index1 * 3 + 2],
        );
        let index2 = face * 3 + 2 as usize;
        let p2 = vec3(
            positions[index2 * 3],
            positions[index2 * 3 + 1],
            positions[index2 * 3 + 2],
        );

        let normal = (p1 - p0).cross(p2 - p0);
        normals[index0 * 3] += normal.x;
        normals[index0 * 3 + 1] += normal.y;
        normals[index0 * 3 + 2] += normal.z;
        normals[index1 * 3] += normal.x;
        normals[index1 * 3 + 1] += normal.y;
        normals[index1 * 3 + 2] += normal.z;
        normals[index2 * 3] += normal.x;
        normals[index2 * 3 + 1] += normal.y;
        normals[index2 * 3 + 2] += normal.z;
    }

    for i in 0..normals.len() / 3 {
        let normal = vec3(normals[3 * i], normals[3 * i + 1], normals[3 * i + 2]).normalize();
        normals[3 * i] = normal.x;
        normals[3 * i + 1] = normal.y;
        normals[3 * i + 2] = normal.z;
    }
    normals
}

fn compute_tangents_with_indices(indices: &[u32], positions: &[f32]) -> Vec<f32> {
    unimplemented!()
}

fn compute_tangents(positions: &[f32]) -> Vec<f32> {
    unimplemented!()
}
