# Dust

### What is Dust?

An OpenGL/WebGL renderer written in Rust for the fun of it.

### Main features:

- Thin and safe abstraction layer on top of OpenGL/WebGL graphics API. Seeks to ensure always being in the correct state.
- Enabling out-of-the-box build to both desktop (Rust + OpenGL) and web (Rust to WebAssembly using [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) + WebGL bindings provided by the [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) crate)
- Additional default abstractions for simple setup of new projects:
    - Deferred renderer which supports multiple light types and shadows
    - Triangle mesh factory and loader
    - Shader programs which can render meshes (e.g. MeshShader, Skybox, Wireframe, ..)
    - Effects which can be applied to the rendered image (e.g. Fog, Debug, ..)
    - Camera input handler

### Run the examples

#### Desktop: 
Build and run an example, in this case 'hello_world':
```console
$ cargo run --example hello_world --release
``` 
#### Web: 
Build and generate web output (webassembly, javascript and html files) into the pkg folder:
```console
$ wasm-pack build examples/hello_world --target web --out-name web --out-dir ../../pkg
``` 
Install a server that properly defines the `application/wasm` mime type for example:
```console
$ npm install -g http-server
``` 
Start the server and go to http://localhost:8080 in a browser:
```console
$ cd pkg/
$ http-server
``` 

#### Desktop and Web: 
Build and run an example on desktop and also generate web output (webassembly, javascript and html files) into the pkg folder:
```console
$ ./examples/hello_world/run 
``` 