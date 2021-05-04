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
        let (_, buffers, _) = ::gltf::import(path)?;
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
            if let Some(iter) = reader.read_positions() {
                let mut positions = Vec::new();
                for vertex_position in iter {
                    positions.push(vertex_position[0]);
                    positions.push(vertex_position[1]);
                    positions.push(vertex_position[2]);
                }

                cpu_meshes.push(CPUMesh {
                    positions,
                    ..Default::default()
                });
            }
        }
    }

    for child in node.children() {
        print_tree(&child, buffers, cpu_meshes, cpu_materials);
    }
}
