use crate::definition::*;
use crate::io::*;
use ::gltf::Gltf;
use std::path::Path;

impl<'a> Loaded<'a> {
    pub fn gltf<P: AsRef<Path>>(
        &'a self,
        path: P,
    ) -> Result<(Vec<CPUMesh>, Vec<CPUMaterial>), IOError> {
        let mut cpu_meshes = Vec::new();
        let mut cpu_materials = Vec::new();

        let bytes = self.bytes(path.as_ref())?;
        let gltf = Gltf::from_slice(bytes)?;
        let (_, buffers, images) = ::gltf::import(path)?;
        for scene in gltf.scenes() {
            print!("Scene {}", scene.index());
            print!(" ({})", scene.name().unwrap_or("<Unnamed>"));
            println!();
            for node in scene.nodes() {
                print_tree(&node, &buffers, &mut cpu_meshes, &mut cpu_materials);
            }
        }

        Ok((cpu_meshes, cpu_materials))
    }
}

fn print_tree(
    node: &::gltf::Node,
    buffers: &[::gltf::buffer::Data],
    cpu_meshes: &mut Vec<CPUMesh>,
    cpu_materials: &mut Vec<CPUMaterial>,
) {
    print!(" -");
    print!(" Node {}", node.index());
    print!(" ({})", node.name().unwrap_or("<Unnamed>"));
    println!();

    if let Some(mesh) = node.mesh() {
        for primitive in mesh.primitives() {
            println!("- Primitive #{}", primitive.index());
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(read_positions) = reader.read_positions() {
                let mut positions = Vec::new();
                for value in read_positions {
                    positions.push(value[0]);
                    positions.push(value[1]);
                    positions.push(value[2]);
                }

                let normals = if let Some(values) = reader.read_normals() {
                    let mut nors = Vec::new();
                    for value in values {
                        nors.push(value[0]);
                        nors.push(value[1]);
                        nors.push(value[2]);
                    }
                    Some(nors)
                } else {
                    None
                };

                let colors = if let Some(values) = reader.read_colors(0) {
                    let mut cols = Vec::new();
                    for value in values.into_rgb_u8() {
                        cols.push(value[0]);
                        cols.push(value[1]);
                        cols.push(value[2]);
                    }
                    Some(cols)
                } else {
                    None
                };

                let uvs = if let Some(values) = reader.read_tex_coords(0) {
                    let mut uvs = Vec::new();
                    for value in values.into_f32() {
                        uvs.push(value[0]);
                        uvs.push(value[1]);
                    }
                    Some(uvs)
                } else {
                    None
                };

                let indices = if let Some(values) = reader.read_indices() {
                    let mut inds = Vec::new();
                    for value in values.into_u32() {
                        inds.push(value);
                    }
                    Some(inds)
                } else {
                    None
                };

                cpu_meshes.push(CPUMesh {
                    positions,
                    normals,
                    indices,
                    colors,
                    uvs,
                    ..Default::default()
                });
            }
        }
    }

    for child in node.children() {
        print_tree(&child, buffers, cpu_meshes, cpu_materials);
    }
}
