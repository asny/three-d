use gl_generator::{Registry, Fallbacks, Api, Profile};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let file_gl = File::create(&Path::new(&out_dir).join("bindings.rs")).unwrap();

    let key = "TARGET";
    match env::var(key) {
        Ok(val) => {
            println!("{}: {:?}", key, val);
            setup_opengl(file_gl);
        },
        Err(e) => println!("couldn't interpret {}: {}", key, e),
    }
}

/*fn setup_webgl(mut file_gl: File)
{
    use gl_generator::{StaticStructGenerator};
    Registry::new(Api::Gles2, (3, 0), Profile::Core, Fallbacks::All, []).write_bindings(
        StaticStructGenerator,
        &mut file_gl
    ).unwrap();
}*/

fn setup_opengl(mut file_gl: File)
{
    use gl_generator::{StructGenerator, DebugStructGenerator};
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
