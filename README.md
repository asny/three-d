# Dust

### What is Dust?

An OpenGL/WebGL2 renderer written in Rust for the fun of it. 
Dust enables out-of-the-box build to both desktop (Rust + OpenGL) and web 
(Rust to WebAssembly using [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) + WebGL bindings provided by the [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) crate).
This means you can develop a 3D application on desktop and easily deploy it on web!

### Examples

https://asny.github.io/dust/

### Main features

- Thin and low-level graphics abstraction layer which maps one-to-one with the OpenGL/WebGL2 graphics APIs.
- Modular abstractions of common graphics concepts such as buffer, texture, camera, program, rendertarget etc. 
It is always possible to combine with direct call to the OpenGL/WebGL2 graphics abstraction layer.
- Deferred renderer which supports rendering several types of 3D objects (triangle mesh, skybox, imposter etc.), 
multiple light types including shadows, and effects applied after rendering the scene (for example fog). 
Again, it is always possible to combine with lower-level functionality and it can be avoided altogether by enabling the "no-renderer" feature.
- Default windows for easy setup (currently [glutin](https://crates.io/crates/glutin) for cross-platform desktop and canvas for web). 
Can be avoided by disabling the "glutin-window" feature and "canvas" feature respectively.

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
$ cd examples/
$ http-server
``` 

#### Desktop and Web: 
Build and run an example on desktop and also generate web output (webassembly, javascript and html files) into the pkg folder:
```console
$ ./examples/hello_world/run 
``` 