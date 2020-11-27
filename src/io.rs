use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use log::info;
use std::path::{Path, PathBuf};

#[cfg(feature = "3d-io")]
pub mod threed;

#[cfg(feature = "3d-io")]
pub use threed::*;

#[cfg(feature = "obj-io")]
pub mod obj;

#[cfg(feature = "obj-io")]
pub use obj::*;


#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "image-io")]
    Image(image::ImageError),
    #[cfg(feature = "3d-io")]
    Bincode(bincode::Error),
    #[cfg(feature = "obj-io")]
    Obj(wavefront_obj::ParseError),
    #[cfg(not(target_arch = "wasm32"))]
    IO(std::io::Error),
    FailedToLoad {message: String}
}

#[cfg(feature = "image-io")]
impl From<image::ImageError> for Error {
    fn from(other: image::ImageError) -> Self {
        Error::Image(other)
    }
}

#[cfg(feature = "3d-io")]
impl From<bincode::Error> for Error {
    fn from(other: bincode::Error) -> Self {
        Error::Bincode(other)
    }
}

#[cfg(feature = "obj-io")]
impl From<wavefront_obj::ParseError> for Error {
    fn from(other: wavefront_obj::ParseError) -> Self {
        Error::Obj(other)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Error::IO(other)
    }
}

pub type Loaded = HashMap<PathBuf, Result<Vec<u8>, std::io::Error>>;
type RefLoaded = Rc<RefCell<Loaded>>;

pub struct Loader {
}

impl Loader {

    pub fn load<F, P: AsRef<Path>>(paths: &[P], on_done: F)
        where F: 'static + FnOnce(&mut Loaded)
    {
        Self::load_with_progress(paths, |progress| {
                    info!("Progress: {}%", 100.0f32 * progress);
        }, on_done);
    }

    pub fn load_with_progress<F, G, P>(paths: &[P], progress_callback: G, on_done: F)
        where
            G: 'static + Fn(f32),
            F: 'static + FnOnce(&mut Loaded),
            P: AsRef<Path>
    {
        let loads = Rc::new(RefCell::new(HashMap::new()));
        for path in paths {
            loads.borrow_mut().insert(path.as_ref().to_path_buf(), Ok(Vec::new()));
            Self::load_file(path,loads.clone());
        }
        info!("Loading started...");
        Self::wait_local(loads.clone(), progress_callback, on_done);
    }

    pub fn get<P: AsRef<Path>>(loaded: &Loaded, path: P) -> Result<&[u8], Error> {
        let bytes = loaded.get(path.as_ref()).ok_or(
            Error::FailedToLoad {message:format!("Tried to use a resource which was not loaded: {}", path.as_ref().to_str().unwrap())})?.as_ref()
            .map_err(|_| Error::FailedToLoad {message:format!("Could not load resource: {}", path.as_ref().to_str().unwrap())})?;
        Ok(bytes)
    }

    #[cfg(feature = "image-io")]
    pub fn get_image<P: AsRef<Path>>(loaded: &Loaded, path: P) -> Result<image::DynamicImage, Error> {
        let img = image::load_from_memory(Self::get(loaded, path)?)?;
        Ok(img)
    }

    fn wait_local<F, G>(loads: RefLoaded, progress_callback: G, on_done: F)
        where
            G: 'static + Fn(f32),
            F: 'static + FnOnce(&mut Loaded)
    {
        Self::sleep(100, move || {

            let is_loading = match loads.try_borrow() {
                Ok(map) => {
                    let total_count = map.len();
                    let mut count = 0;
                    for bytes in map.values() {
                        if bytes.is_err() || bytes.as_ref().unwrap().len() > 0 {
                            count = count + 1;
                        }
                    }
                    progress_callback(count as f32 / total_count as f32);
                    count < total_count
                },
                Err(_) => true
            };

            if is_loading {
                Self::wait_local(loads, progress_callback, on_done);
            } else {
                info!("Loading done.");
                on_done(&mut loads.borrow_mut());
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn sleep<F>(millis: u64, fun: F)
    where
        F: 'static + FnOnce()
    {
        std::thread::sleep(std::time::Duration::from_millis(millis));
        fun();
    }

    #[cfg(target_arch = "wasm32")]
    fn sleep<F>(millis: u64, fun: F)
    where
        F: 'static + FnOnce()
    {
        use gloo_timers::callback::Timeout;
        let timeout = Timeout::new(millis as u32, fun);
        timeout.forget();
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_file<P: AsRef<Path>>(path: P, loads: RefLoaded)
    {
        let file = std::fs::File::open(path.as_ref());
        match file {
            Ok(mut f) => {
                use std::io::prelude::*;
                let mut bytes = Vec::new();
                let result = f.read_to_end(&mut bytes).and(Ok(bytes));
                loads.borrow_mut().insert(path.as_ref().to_path_buf(), result);
            },
            Err(e) => {loads.borrow_mut().insert(path.as_ref().to_path_buf(), Err(e));}
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn load_file<P: AsRef<Path>>(path: P, loads: RefLoaded)
    {
        wasm_bindgen_futures::spawn_local(Self::load_file_async(path, loads));
    }

    #[cfg(target_arch = "wasm32")]
    async fn load_file_async<P: AsRef<Path>>(path: P, loads: RefLoaded)
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, RequestInit, RequestMode, Response};

        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(path.as_ref().to_str(), &opts).unwrap();
        request.headers().set("Accept", "application/octet-stream").unwrap();

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
        let resp: Response = resp_value.dyn_into().unwrap();

        // Convert this other `Promise` into a rust `Future`.
        let data: JsValue = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
        loads.borrow_mut().insert(path.to_owned(), Ok(js_sys::Uint8Array::new(&data).to_vec()));
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct Saver {

}

#[cfg(not(target_arch = "wasm32"))]
impl Saver {

    #[cfg(all(feature = "3d-io", feature = "image-io"))]
    pub fn save_3d_file<P: AsRef<Path>>(path: P, cpu_meshes: &Vec<&crate::CPUMesh>) -> Result<(), Error>
    {
        let dir = path.as_ref().parent().unwrap();
        let filename = path.as_ref().file_stem().unwrap().to_str().unwrap();
        let mut input = Vec::new();
        let mut i = 0;
        for cpu_mesh in cpu_meshes {
            let texture_path = if let Some(ref img) = cpu_mesh.texture {
                let tex_path = dir.join(format!("{}_tex{}.png", filename, i));
                i += 1;
                img.save_with_format(&tex_path, image::ImageFormat::Png)?;
                Some(tex_path)
            } else {None};
            input.push((*cpu_mesh, texture_path));
        }

        let bytes = ThreeD::serialize(&input)?;
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

        image::save_buffer(path, &pixels_out, width as u32, height as u32, image::ColorType::Rgb8).unwrap();
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
