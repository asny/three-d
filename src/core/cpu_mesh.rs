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

///
/// A CPU-side version of a triangle mesh.
/// Can be constructed manually or loaded via [io](crate::io)
/// or via the utility functions for generating simple triangle meshes.
///
#[derive(Default)]
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
    /// The tangents of the vertices, orthogonal direction to the normal.
    /// Three contiguous floats defines a tangent `(x, y, z)` and a value that specifies the handedness (either -1.0 or 1.0), therefore the length must be divisable by 4.
    pub tangents: Option<Vec<f32>>,
    /// The uv coordinates of the vertices. Two contiguous floats defines a coordinate `(u, v)`, therefore the length must be divisable by 2.
    pub uvs: Option<Vec<f32>>,
    /// The colors of the vertices. Four contiguous bytes defines a color `(r, g, b, a)`, therefore the length must be divisable by 4.
    /// The colors are assumed to be in linear space.
    pub colors: Option<Vec<u8>>,
}

impl std::fmt::Debug for CPUMesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("CPUMesh");
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

///
/// Specifies the functions used to generate a mesh.
///
/// Fields with `None` will generate a mesh without that field.
///
pub struct CPUMeshGenerator<'t, P> {
    /// Generates the position buffer.
    pub positions: &'t mut (dyn FnMut(P) -> [f32; 3] + 't),
    /// Generates the normals buffer.
    pub normals: Option<&'t mut (dyn FnMut(P) -> [f32; 3] + 't)>,
    /// Generates the tangents buffer.
    pub tangents: Option<&'t mut (dyn FnMut(P) -> [f32; 4] + 't)>,
    /// Generates the uv buffer.
    pub uvs: Option<&'t mut (dyn FnMut(P) -> [f32; 2] + 't)>,
    /// Generates the color buffer.
    pub colors: Option<&'t mut (dyn FnMut(P) -> [u8; 4] + 't)>,
}

impl<'t, P: Copy> CPUMeshGenerator<'t, P> {
    fn generate(&mut self, indices: Option<Indices>, name: impl ToString, params: &[P]) -> CPUMesh {
        fn vec_gen<P: Copy, T, const N: usize>(
            params: &[P],
            gen: &mut (dyn FnMut(P) -> [T; N] + '_),
        ) -> Vec<T> {
            let mut vec = Vec::with_capacity(params.len() * N);

            for &p in params {
                vec.extend(gen(p));
            }

            vec
        }

        CPUMesh {
            name: name.to_string(),
            material_name: None,
            indices,
            positions: vec_gen(params, self.positions),
            normals: self.normals.as_mut().map(|f| vec_gen(params, f)),
            tangents: self.tangents.as_mut().map(|f| vec_gen(params, f)),
            uvs: self.uvs.as_mut().map(|f| vec_gen(params, f)),
            colors: self.colors.as_mut().map(|f| vec_gen(params, f)),
        }
    }
}

///
/// A circle vertex.
///
#[derive(Clone, Copy)]
pub struct CircleVertex {
    /// The angle bearing of the vertex from +X towards +Y.
    pub angle: f32,
    /// The X coordinate of the normalized direction vector.
    pub x: f32,
    /// The Y coordinate of the normalized direction vector.
    pub y: f32,
}

///
/// A sphere vertex.
///
#[derive(Clone, Copy)]
pub struct SphereVertex {
    /// The X coordinate of the normalized direction vector.
    pub x: f32,
    /// The Y coordinate of the normalized direction vector.
    pub y: f32,
    /// The Z coordinate of the normalized direction vector.
    pub z: f32,
    /// The angle of the vertex from +Z towards the XY plane.
    pub theta: f32,
    /// The angle of the vertex projection on the XY plane from +X towards +Y.
    pub phi: f32,
}

///
/// A cylinder vertex.
///
#[derive(Clone, Copy)]
pub struct CylinderVertex {
    /// The Y coordinate of the normalized direction vector on the YZ plane.
    pub y: f32,
    /// The Z coordinate of the normalized direction vector on the YZ plane.
    pub z: f32,
    /// The angle of the vertex from +Y towards +Z on the YZ plane.
    pub angle: f32,
    /// The X coordinate (height) of the vertex.
    pub x: f32,
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
                let n = normal_transform
                    * vec4(normals[i * 3], normals[i * 3 + 1], normals[i * 3 + 2], 1.0);
                normals[i * 3] = n.x;
                normals[i * 3 + 1] = n.y;
                normals[i * 3 + 2] = n.z;
            }
        }

        if let Some(ref mut tangents) = self.tangents {
            for i in 0..tangents.len() / 4 {
                let t = normal_transform
                    * vec4(
                        tangents[i * 4],
                        tangents[i * 4 + 1],
                        tangents[i * 4 + 2],
                        1.0,
                    );
                tangents[i * 4] = t.x;
                tangents[i * 4 + 1] = t.y;
                tangents[i * 4 + 2] = t.z;
            }
        }
    }

    ///
    /// Returns a square mesh spanning the xy-plane with positions in the range `[-1..1]` in the x and y axes.
    ///
    pub fn square() -> Self {
        let halfsize = 1.0;

        Self::square_gen(
            "square",
            CPUMeshGenerator {
                positions: &mut |[u, v]| {
                    [(u * 2.0 - 1.0) * halfsize, (v * 2.0 - 1.0) * halfsize, 0.0]
                },
                normals: Some(&mut |_| [0.0, 0.0, 1.0]),
                tangents: Some(&mut |_| [1.0, 0.0, 0.0, 1.0]),
                uvs: Some(&mut |[u, v]| [u, v]),
                colors: None,
            },
        )
    }

    ///
    /// Generates a square mesh using a generator with parameters `[0.0|1.0, 0.0|1.0]`.
    ///
    pub fn square_gen(name: impl ToString, mut gen: CPUMeshGenerator<'_, [f32; 2]>) -> Self {
        let indices = vec![0u8, 1, 2, 2, 3, 0];

        gen.generate(
            Some(Indices::U8(indices)),
            name,
            &[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
        )
    }

    ///
    /// Returns a circle mesh spanning the xy-plane with radius 1 and center in `(0, 0, 0)`.
    ///
    pub fn circle(angle_subdivisions: u32) -> Self {
        Self::circle_gen(
            "circle",
            angle_subdivisions,
            CPUMeshGenerator {
                positions: &mut |vertex| [vertex.x, vertex.y, 0.0],
                normals: Some(&mut |_| [0.0, 0.0, 1.0]),
                tangents: None,
                uvs: None,
                colors: None,
            },
        )
    }

    ///
    /// Generates a circle mesh using a generator.
    ///
    pub fn circle_gen(
        name: impl ToString,
        angle_subdivisions: u32,
        mut gen: CPUMeshGenerator<'_, CircleVertex>,
    ) -> Self {
        let mut params = Vec::new();
        let mut indices = Vec::new();

        for j in 0..angle_subdivisions {
            let angle = 2.0 * std::f32::consts::PI * j as f32 / angle_subdivisions as f32;
            let (y, x) = angle.sin_cos();
            params.push(CircleVertex { angle, x, y });

            indices.push(0);
            indices.push(j as u16);
            indices.push(((j + 1) % angle_subdivisions) as u16);
        }

        gen.generate(Some(Indices::U16(indices)), name, &params)
    }

    ///
    /// Returns a sphere mesh with radius 1 and center in `(0, 0, 0)`.
    ///
    pub fn sphere(angle_subdivisions: u32) -> Self {
        Self::sphere_gen(
            "sphere",
            angle_subdivisions,
            CPUMeshGenerator {
                positions: &mut |v| [v.x, v.y, v.z],
                normals: Some(&mut |v| [v.x, v.y, v.z]),
                tangents: None,
                uvs: None,
                colors: None,
            },
        )
    }

    ///
    /// Generates a circle mesh using a generator.
    ///
    pub fn sphere_gen(
        name: impl ToString,
        angle_subdivisions: u32,
        mut gen: CPUMeshGenerator<'_, SphereVertex>,
    ) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        vertices.push(SphereVertex {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            theta: 0.0,
            phi: 0.0,
        });

        for j in 0..angle_subdivisions * 2 {
            let j1 = (j + 1) % (angle_subdivisions * 2);
            indices.push(0);
            indices.push((1 + j) as u16);
            indices.push((1 + j1) as u16);
        }

        for i in 0..angle_subdivisions - 1 {
            let theta = std::f32::consts::PI * (i + 1) as f32 / angle_subdivisions as f32;
            let (sin_theta, cos_theta) = theta.sin_cos();
            let i0 = 1 + i * angle_subdivisions * 2;
            let i1 = 1 + (i + 1) * angle_subdivisions * 2;

            for j in 0..angle_subdivisions * 2 {
                let phi = std::f32::consts::PI * j as f32 / angle_subdivisions as f32;
                let (sin_phi, cos_phi) = phi.sin_cos();
                let x = sin_theta * cos_phi;
                let y = sin_theta * sin_phi;
                let z = cos_theta;
                vertices.push(SphereVertex {
                    x,
                    y,
                    z,
                    theta,
                    phi,
                });

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

        vertices.push(SphereVertex {
            x: 0.0,
            y: 0.0,
            z: -1.0,
            theta: std::f32::consts::PI,
            phi: 0.0,
        });

        let i = 1 + (angle_subdivisions - 2) * angle_subdivisions * 2;
        for j in 0..angle_subdivisions * 2 {
            let j1 = (j + 1) % (angle_subdivisions * 2);
            indices.push((i + j) as u16);
            indices.push(((angle_subdivisions - 1) * angle_subdivisions * 2 + 1) as u16);
            indices.push((i + j1) as u16);
        }

        gen.generate(Some(Indices::U16(indices)), name, &vertices)
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
        mesh.compute_tangents().unwrap();
        mesh
    }

    ///
    /// Returns a cylinder mesh around the x-axis in the range `[0..1]` and with radius 1.
    ///
    pub fn cylinder(angle_subdivisions: u32) -> Self {
        let mut mesh = Self::cylinder_gen(
            "cylinder",
            angle_subdivisions,
            CPUMeshGenerator {
                positions: &mut |v| [v.x, v.y, v.z],
                normals: None,
                tangents: None,
                uvs: None,
                colors: None,
            },
        );
        mesh.compute_normals();
        mesh
    }

    ///
    /// Generates a cylinder mesh using a generator.
    ///
    pub fn cylinder_gen(
        name: impl ToString,
        angle_subdivisions: u32,
        mut gen: CPUMeshGenerator<'_, CylinderVertex>,
    ) -> Self {
        let mut vertices = Vec::new();
        let length_subdivisions = 1;
        let mut indices = Vec::new();

        for i in 0..length_subdivisions + 1 {
            let x = i as f32 / length_subdivisions as f32;
            for j in 0..angle_subdivisions {
                let angle = 2.0 * std::f32::consts::PI * j as f32 / angle_subdivisions as f32;

                let (z, y) = angle.sin_cos();
                vertices.push(CylinderVertex { y, z, angle, x })
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

        gen.generate(Some(Indices::U16(indices)), name, &vertices)
    }

    ///
    /// Returns a cone mesh around the x-axis in the range `[0..1]` and with radius 1 at -1.0.
    ///
    pub fn cone(angle_subdivisions: u32) -> Self {
        let mut mesh = Self::cylinder_gen(
            "cone",
            angle_subdivisions,
            CPUMeshGenerator {
                positions: &mut |v| [v.x, v.y * (1.0 - v.x), v.z * (1.0 - v.x)],
                normals: None,
                tangents: None,
                uvs: None,
                colors: None,
            },
        );
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
        let mut normals = vec![0.0f32; self.positions.len()];
        self.for_each_triangle(|i0, i1, i2| {
            let p0 = self.position(i0);
            let p1 = self.position(i1);
            let p2 = self.position(i2);
            let normal = (p1 - p0).cross(p2 - p0);
            normals[i0 * 3] += normal.x;
            normals[i0 * 3 + 1] += normal.y;
            normals[i0 * 3 + 2] += normal.z;
            normals[i1 * 3] += normal.x;
            normals[i1 * 3 + 1] += normal.y;
            normals[i1 * 3 + 2] += normal.z;
            normals[i2 * 3] += normal.x;
            normals[i2 * 3 + 1] += normal.y;
            normals[i2 * 3 + 2] += normal.z;
        });

        self.for_each_vertex(|i| {
            let normal = vec3(normals[3 * i], normals[3 * i + 1], normals[3 * i + 2]).normalize();
            normals[3 * i] = normal.x;
            normals[3 * i + 1] = normal.y;
            normals[3 * i + 2] = normal.z;
        });
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
        let mut tan1 = vec![vec3(0.0, 0.0, 0.0); self.positions.len() / 3];
        let mut tan2 = vec![vec3(0.0, 0.0, 0.0); self.positions.len() / 3];

        self.for_each_triangle(|i0, i1, i2| {
            let a = self.position(i0);
            let b = self.position(i1);
            let c = self.position(i2);
            let uva = self.uv(i0).unwrap();
            let uvb = self.uv(i1).unwrap();
            let uvc = self.uv(i2).unwrap();

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

        let mut tangents = vec![0.0f32; 4 * self.positions.len() / 3];
        self.for_each_vertex(|index| {
            let normal = self.normal(index).unwrap();
            let t = tan1[index];
            let tangent = (t - normal * normal.dot(t)).normalize();
            let handedness = if normal.cross(tangent).dot(tan2[index]) < 0.0 {
                1.0
            } else {
                -1.0
            };
            tangents[index * 4] = tangent.x;
            tangents[index * 4 + 1] = tangent.y;
            tangents[index * 4 + 2] = tangent.z;
            tangents[index * 4 + 3] = handedness;
        });

        self.tangents = Some(tangents);
        Ok(())
    }

    ///
    ///  Iterates over all vertices in this mesh and calls the callback function with the index for each vertex.
    ///
    pub fn for_each_vertex(&self, mut callback: impl FnMut(usize)) {
        for i in 0..self.positions.len() / 3 {
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
                for face in 0..self.positions.len() / 9 {
                    callback(face * 3, face * 3 + 1, face * 3 + 2);
                }
            }
        }
    }

    ///
    /// Returns the position of the vertex with the given index.
    ///
    pub fn position(&self, vertex_index: usize) -> Vec3 {
        vec3(
            self.positions[3 * vertex_index],
            self.positions[3 * vertex_index + 1],
            self.positions[3 * vertex_index + 2],
        )
    }

    ///
    /// Returns the normal of the vertex with the given index.
    ///
    pub fn normal(&self, vertex_index: usize) -> Option<Vec3> {
        self.normals.as_ref().map(|normals| {
            vec3(
                normals[3 * vertex_index],
                normals[3 * vertex_index + 1],
                normals[3 * vertex_index + 2],
            )
        })
    }

    ///
    /// Returns the uv coordinates of the vertex with the given index.
    ///
    pub fn uv(&self, vertex_index: usize) -> Option<Vec2> {
        self.uvs
            .as_ref()
            .map(|uvs| vec2(uvs[2 * vertex_index], uvs[2 * vertex_index + 1]))
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
