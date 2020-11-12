
use crate::*;
use std::rc::Rc;
use std::cell::RefCell;

#[cfg(feature = "3d-io")]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CPUMesh {
    pub magic_number: u8,
    pub version: u8,
    pub indices: Vec<u32>,
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>
}

#[cfg(feature = "3d-io")]
impl CPUMesh {
    pub fn empty() -> Self
    {
        Self::new(&[], &[], &[], &[]).unwrap()
    }

    pub fn new(indices: &[u32], positions: &[f32], normals: &[f32], uvs: &[f32]) -> Result<Self, objects::Error>
    {
        Ok(CPUMesh {magic_number: 61, version: 2, indices: indices.to_owned(), positions: positions.to_owned(), normals: normals.to_owned(), uvs: uvs.to_owned()})
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<CPUMesh, bincode::Error>
    {
        let decoded = bincode::deserialize::<CPUMesh>(bytes)
            .or_else(|_| bincode::deserialize::<CPUMeshV1>(bytes).map(|m| CPUMesh {
                magic_number: m.magic_number,
                version: 2,
                indices: m.indices,
                positions: m.positions,
                normals: m.normals,
                uvs: vec![]
            }))?;
        if decoded.magic_number != 61 {
            Err(bincode::Error::new(bincode::ErrorKind::Custom("Corrupt file!".to_string())))?;
        }
        Ok(decoded)
    }

    pub fn from_file(path: &'static str) -> Rc<RefCell<CPUMesh>> {
        let mesh = Rc::new(RefCell::new(Self::empty()));
        let m = mesh.clone();
        Self::from_file_with_mapping(path, move |mesh| {*m.borrow_mut() = mesh;});
        mesh
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_file_with_mapping<F: 'static>(path: &'static str, mapping: F)
        where F: Fn(CPUMesh)
    {
        let mut file = std::fs::File::open(path).unwrap();
        let mut bytes = Vec::new();
        use std::io::prelude::*;
        file.read_to_end(&mut bytes).unwrap();
        mapping(Self::from_bytes(&bytes).unwrap());
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_file_with_mapping<F: 'static>(path: &'static str, mapping: F)
        where F: Fn(CPUMesh)
    {
        wasm_bindgen_futures::spawn_local(Self::load(path, mapping));
    }

    #[cfg(target_arch = "wasm32")]
    async fn load<F: 'static>(url: &'static str, mapping: F)
        where F: Fn(CPUMesh)
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, RequestInit, RequestMode, Response};

        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(&url, &opts).unwrap();
        request.headers().set("Accept", "application/octet-stream").unwrap();

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
        let resp: Response = resp_value.dyn_into().unwrap();

        // Convert this other `Promise` into a rust `Future`.
        let data: JsValue = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
        let bytes: Vec<u8> = js_sys::Uint8Array::new(&data).to_vec();
        mapping(CPUMesh::from_bytes(&bytes).unwrap());
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, objects::Error>
    {
        Ok(bincode::serialize(self)?)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_file(&self, path: &str) -> Result<(), objects::Error>
    {
        let mut file = std::fs::File::create(path)?;
        use std::io::prelude::*;
        file.write_all(&self.to_bytes()?)?;
        Ok(())
    }
}

#[cfg(feature = "3d-io")]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct CPUMeshV1 {
    pub magic_number: u8,
    pub version: u8,
    pub indices: Vec<u32>,
    pub positions: Vec<f32>,
    pub normals: Vec<f32>
}

pub fn compute_normals_with_indices(indices: &[u32], positions: &[f32]) -> Vec<f32> {
    let mut normals = vec![0.0f32; positions.len() * 3];
    for face in 0..indices.len()/3 {
        let index0 = indices[face*3] as usize;
        let p0 = vec3(positions[index0*3], positions[index0*3+1], positions[index0*3+2]);
        let index1 = indices[face*3 + 1] as usize;
        let p1 = vec3(positions[index1*3], positions[index1*3+1], positions[index1*3+2]);
        let index2 = indices[face*3 + 2] as usize;
        let p2 = vec3(positions[index2*3], positions[index2*3+1], positions[index2*3+2]);

        let normal = (p1 - p0).cross(p2 - p0);
        normals[index0*3] += normal.x;
        normals[index0*3+1] += normal.y;
        normals[index0*3+2] += normal.z;
        normals[index1*3] += normal.x;
        normals[index1*3+1] += normal.y;
        normals[index1*3+2] += normal.z;
        normals[index2*3] += normal.x;
        normals[index2*3+1] += normal.y;
        normals[index2*3+2] += normal.z;
    }

    for i in 0..normals.len()/3 {
        let normal = vec3(normals[3*i], normals[3*i+1], normals[3*i+2]).normalize();
        normals[3*i] = normal.x;
        normals[3*i+1] = normal.y;
        normals[3*i+2] = normal.z;
    }
    normals
}


pub fn compute_normals(positions: &[f32]) -> Vec<f32> {
    let mut normals = vec![0.0f32; positions.len()];
    for face in 0..positions.len()/9 {
        let index0 = face*3 as usize;
        let p0 = vec3(positions[index0*3], positions[index0*3+1], positions[index0*3+2]);
        let index1 = face*3 + 1 as usize;
        let p1 = vec3(positions[index1*3], positions[index1*3+1], positions[index1*3+2]);
        let index2 = face*3 + 2 as usize;
        let p2 = vec3(positions[index2*3], positions[index2*3+1], positions[index2*3+2]);

        let normal = (p1 - p0).cross(p2 - p0);
        normals[index0*3] += normal.x;
        normals[index0*3+1] += normal.y;
        normals[index0*3+2] += normal.z;
        normals[index1*3] += normal.x;
        normals[index1*3+1] += normal.y;
        normals[index1*3+2] += normal.z;
        normals[index2*3] += normal.x;
        normals[index2*3+1] += normal.y;
        normals[index2*3+2] += normal.z;
    }

    for i in 0..normals.len()/3 {
        let normal = vec3(normals[3*i], normals[3*i+1], normals[3*i+2]).normalize();
        normals[3*i] = normal.x;
        normals[3*i+1] = normal.y;
        normals[3*i+2] = normal.z;
    }
    normals
}