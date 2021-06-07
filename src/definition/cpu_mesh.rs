use crate::math::*;

#[derive(Debug)]
pub enum Indices {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
}

impl Indices {
    pub fn into_u32(&self) -> Vec<u32> {
        match self {
            Self::U8(ind) => ind.iter().map(|i| *i as u32).collect::<Vec<u32>>(),
            Self::U16(ind) => ind.iter().map(|i| *i as u32).collect::<Vec<u32>>(),
            Self::U32(ind) => ind.clone(),
        }
    }
}

///
/// A CPU-side version of a triangle mesh (for example [Mesh](crate::Mesh)).
/// Can be constructed manually or loaded via [io](crate::io)
/// or via the utility functions for generating simple triangle meshes.
///
#[derive(Default, Debug)]
pub struct CPUMesh {
    pub name: String,
    pub material_name: Option<String>,
    pub positions: Vec<Vec3>,
    pub indices: Option<Indices>,
    pub normals: Option<Vec<f32>>,
    pub uvs: Option<Vec<f32>>,
    pub colors: Option<Vec<u8>>,
}

impl CPUMesh {
    pub fn square(size: f32) -> Self {
        let indices = vec![0u8, 1, 2, 2, 3, 0];
        let halfsize = 0.5 * size;
        let positions = vec![
            vec3(-halfsize, -halfsize, 0.0),
            vec3(halfsize, -halfsize, 0.0),
            vec3(halfsize, halfsize, 0.0),
            vec3(-halfsize, halfsize, 0.0),
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

    pub fn circle(radius: f32, angle_subdivisions: u32) -> Self {
        let mut positions = Vec::new();
        let mut indices = Vec::new();
        let mut normals = Vec::new();
        for j in 0..angle_subdivisions {
            let angle = 2.0 * std::f32::consts::PI * j as f32 / angle_subdivisions as f32;

            positions.push(vec3(radius * angle.cos(), radius * angle.sin(), 0.0));

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

    pub fn sphere(radius: f32) -> Self {
        let x = radius * 0.525731112119133606f32;
        let z = radius * 0.850650808352039932f32;
        let positions = vec![
            vec3(-x, 0.0, z),
            vec3(x, 0.0, z),
            vec3(-x, 0.0, -z),
            vec3(x, 0.0, -z),
            vec3(0.0, z, x),
            vec3(0.0, z, -x),
            vec3(0.0, -z, x),
            vec3(0.0, -z, -x),
            vec3(z, x, 0.0),
            vec3(-z, x, 0.0),
            vec3(z, -x, 0.0),
            vec3(-z, -x, 0.0),
        ];
        let indices = vec![
            0u8, 1, 4, 0, 4, 9, 9, 4, 5, 4, 8, 5, 4, 1, 8, 8, 1, 10, 8, 10, 3, 5, 8, 3, 5, 3, 2, 2,
            3, 7, 7, 3, 10, 7, 10, 6, 7, 6, 11, 11, 6, 0, 0, 6, 1, 6, 10, 1, 9, 11, 0, 9, 2, 11, 9,
            5, 2, 7, 11, 2,
        ];
        let mut mesh = CPUMesh {
            name: "sphere".to_string(),
            indices: Some(Indices::U8(indices)),
            positions,
            ..Default::default()
        };
        mesh.compute_normals();
        mesh
    }

    pub fn cylinder(radius: f32, length: f32, angle_subdivisions: u32) -> Self {
        let length_subdivisions = 1;
        let mut positions = Vec::new();
        let mut indices = Vec::new();
        for i in 0..length_subdivisions + 1 {
            let x = i as f32 / length_subdivisions as f32;
            for j in 0..angle_subdivisions {
                let angle = 2.0 * std::f32::consts::PI * j as f32 / angle_subdivisions as f32;

                positions.push(vec3(length * x, radius * angle.cos(), radius * angle.sin()));
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

    pub fn cone(radius: f32, length: f32, angle_subdivisions: u32) -> Self {
        let length_subdivisions = 1;
        let mut positions = Vec::new();
        let mut indices = Vec::new();
        for i in 0..length_subdivisions + 1 {
            let x = i as f32 / length_subdivisions as f32;
            for j in 0..angle_subdivisions {
                let angle = 2.0 * std::f32::consts::PI * j as f32 / angle_subdivisions as f32;

                positions.push(vec3(
                    length * x,
                    radius * angle.cos() * (1.0 - x),
                    radius * angle.sin() * (1.0 - x),
                ));
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

    pub fn arrow(radius: f32, length: f32, angle_subdivisions: u32) -> Self {
        let cylinder_length = length * 0.7;
        let mut arrow = Self::cylinder(radius * 0.5, cylinder_length, angle_subdivisions);
        arrow.name = "arrow".to_string();
        let mut cone = Self::cone(radius, length - cylinder_length, angle_subdivisions);
        for i in 0..cone.positions.len() {
            cone.positions[i].x += cylinder_length;
        }
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
                .unwrap_or(compute_normals(&self.positions)),
        );
    }

    ///
    /// Computes the axis aligned bounding box of the mesh.
    ///
    pub fn compute_aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::new_with_positions(&self.positions)
    }
}

fn compute_normals_with_indices(indices: &[u32], positions: &[Vec3]) -> Vec<f32> {
    let mut normals = vec![0.0f32; positions.len() * 9];
    for face in 0..indices.len() / 3 {
        let index0 = indices[face * 3] as usize;
        let p0 = positions[index0];
        let index1 = indices[face * 3 + 1] as usize;
        let p1 = positions[index1];
        let index2 = indices[face * 3 + 2] as usize;
        let p2 = positions[index2];

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

fn compute_normals(positions: &[Vec3]) -> Vec<f32> {
    let mut normals = vec![0.0f32; positions.len() * 3];
    for face in 0..positions.len() / 3 {
        let index0 = face as usize;
        let p0 = positions[index0];
        let index1 = face + 1 as usize;
        let p1 = positions[index1];
        let index2 = face + 2 as usize;
        let p2 = positions[index2];

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
