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

### Run the examples

- Desktop: 
```console
# Build and run an example, in this case 'hello_world':
$ cargo run --example hello_world --release
``` 
- Web: 
```console
# Build and generate web output (webassembly, javascript and html files) into the pkg folder:
wasm-pack build examples/hello_world --target web --out-name web --out-dir ../../pkg

# Install a server that properly defines the `application/wasm` mime type for example
npm install -g http-server

# Start the server and go to http://localhost:8080 in a browser
cd pkg/
http-serve
# 
``` 

- Desktop + Web: 
```console
# Build and run an example on desktop and also generate web output 
# (webassembly, javascript and html files) into the pkg folder:
./examples/hello_world/run 
``` 