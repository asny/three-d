
use std::path::Path;
use crate::io::*;

pub struct Saver {

}

impl Saver {

    #[cfg(all(feature = "3d-io", feature = "image-io"))]
    pub fn save_3d_file<P: AsRef<Path>>(path: P, cpu_meshes: Vec<crate::CPUMesh>, cpu_materials: Vec<crate::CPUMaterial>) -> Result<(), Error>
    {
        let dir = path.as_ref().parent().unwrap();
        let filename = path.as_ref().file_stem().unwrap().to_str().unwrap();
        for cpu_material in cpu_materials.iter() {
            if let Some(ref cpu_texture) = cpu_material.texture_image {
                if let Some(ref bytes) = cpu_texture.bytes {
                    let number_of_channels = bytes.len() / (cpu_texture.width * cpu_texture.height);
                    let format = match number_of_channels {
                        1 => Ok(image::ColorType::L8),
                        3 => Ok(image::ColorType::Rgb8),
                        4 => Ok(image::ColorType::Rgba8),
                        _ => Err(crate::io::Error::FailedToSave {message: format!("Texture image could not be saved")})
                    }?;
                    let tex_path = dir.join(format!("{}_{}.png", filename, cpu_material.name));
                    image::save_buffer(tex_path, bytes, cpu_texture.width as u32, cpu_texture.height as u32, format)?;
                }
            }
        }
        let bytes = ThreeD::serialize(filename, cpu_meshes, cpu_materials)?;
        Self::save_file(dir.join(format!("{}.3d", filename)), &bytes)?;
        Ok(())
    }

    #[cfg(feature = "image-io")]
    pub fn save_pixels<P: AsRef<Path>>(path: P, pixels: &[u8], width: usize, height: usize) -> Result<(), Error>
    {
        let mut pixels_out = vec![0u8; width * height * 3];
        for row in 0..height {
            for col in 0..width {
                for i in 0..3 {
                    pixels_out[3 * width * (height - row - 1) + 3 * col + i] =
                        pixels[3 * width * row + 3 * col + i];
                }
            }
        }

        image::save_buffer(path, &pixels_out, width as u32, height as u32, image::ColorType::Rgb8)?;
        Ok(())
    }

    pub fn save_file<P: AsRef<Path>>(path: P, bytes: &[u8]) -> Result<(), Error>
    {
        let mut file = std::fs::File::create(path)?;
        use std::io::prelude::*;
        file.write_all(bytes)?;
        Ok(())
    }
}
