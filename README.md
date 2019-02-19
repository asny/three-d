# Dust

### What is Dust?

An OpenGL/WebGL renderer written in Rust for the fun of it.

### Main features:

- Thin and safe abstraction layer on top of OpenGL/WebGL graphics API. Seeks to ensure always being in the correct state.
- Enabling out-of-the-box build to both desktop (Rust + OpenGL) and web (Rust to WebAssembly using [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) + WebGL bindings provided by the [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) crate)
- Additional default abstractions for simple setup of new projects:
    - Pipelines which defines the rendering pipeline (Forward or Deferred)
    - Objects which can render geometry (e.g. Skybox, ShadedMesh, Wireframe, ..)
    - Effects which can be applied to a rendered image at some point in the rendering pipeline (e.g. Fog, Debug, ..)
    - And more..

### Run the 'hello world' example

- Desktop: 
```console
$ cargo run --example hello_world --release
``` 
- Web: 
```console
# Any server that properly defines the `application/wasm` mime type can be used
npm install -g http-server

# Install the wasm-bindgen tool
cargo install -f wasm-bindgen@0.2.37

# Build the 'hello world' example to WebAssembly
cd examples/web/
RELEASE=1 ./build

# Run the demo at http://localhost:8080
cd www/
http-server --open
``` 