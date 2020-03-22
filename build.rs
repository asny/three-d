
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    setup_opengl();
}

#[cfg(not(target_arch = "wasm32"))]
fn setup_opengl()
{
    use std::env;
    use std::fs::File;
    use std::path::Path;
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut file_gl = File::create(&Path::new(&out_dir).join("bindings.rs")).unwrap();

    use gl_generator::{StructGenerator, DebugStructGenerator, Registry, Fallbacks, Api, Profile};
    let registry = Registry::new(Api::Gl, (4, 3), Profile::Core, Fallbacks::All, []);

    if env::var("CARGO_FEATURE_DEBUG").is_ok() {
        registry.write_bindings(
            DebugStructGenerator,
            &mut file_gl
        ).unwrap();
    } else {
        registry.write_bindings(
            StructGenerator,
            &mut file_gl
        ).unwrap();
    }
}
