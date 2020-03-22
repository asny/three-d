# `three-d`

### What is it?

A renderer written in Rust which enables out-of-the-box build to both desktop (Rust + OpenGL) and web 
(Rust to WebAssembly using [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) + WebGL bindings provided by the [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) crate).
This means you can develop a 3D application on desktop and easily deploy it on web!

### Examples

https://asny.github.io/three-d/

### Main features

- Thin and low-level graphics abstraction layer which maps one-to-one with the OpenGL/WebGL2 graphics APIs.
- Modular abstractions of common graphics concepts such as buffer, texture, camera, program, rendertarget etc. 
It is always possible to combine with direct call to the OpenGL/WebGL2 graphics abstraction layer.
- Deferred renderer which supports rendering several types of 3D objects (triangle mesh, skybox, imposter etc.), 
multiple light types including shadows, and effects applied after rendering the scene (for example fog). 
Again, it is always possible to combine with lower-level functionality and it can be avoided altogether by enabling the "no-renderer" feature.
- Default windows for easy setup (currently [glutin](https://crates.io/crates/glutin) for cross-platform desktop and canvas for web). 
Can be avoided by disabling the "glutin-window" feature and "canvas" feature respectively.

### Build

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

### The 3d format

`three-d` supports a custom format with the extension ".3d". 
The advantages of the .3d format is that it is smaller in size than other open formats like obj and stl 
and easier to read/write when you are using Rust or specifically the [serde](https://github.com/serde-rs/serde) and [bincode](https://github.com/servo/bincode) crates. 
To create a .3d file, the easiest option is to create a [CPUMesh](https://github.com/asny/three-d/blob/master/src/objects/cpu_mesh.rs):

```rust
use three_d::*;

fn main() {
    // Create the cpu mesh (a single triangle)
    let indices = [0, 1, 2];
    let positions = [-3.0, -1.0, 0.0,  3.0, -1.0, 0.0,  0.0, 2.0, 0.0];
    let normals = [0.0, 0.0, 1.0,  0.0, 0.0, 1.0,  0.0, 0.0, 1.0];
    let cpu_mesh = CPUMesh::new(&indices, &positions, &normals).unwrap();
    
    // Save to a ".3d" file
    cpu_mesh.to_file("foo.3d").unwrap();

    // Load the ".3d" file into a cpu mesh and transform it into a mesh on the gpu that can be rendered
    let mesh = CPUMesh::from_file("foo.3d").unwrap().to_mesh(&gl).unwrap();
}
```

If you want to load, create, combine or deform meshes and then save to ".3d" format, the [tri-mesh](https://github.com/asny/tri-mesh) crate is an option.

If you want to make your own reader/writer of the .3d format yourself, then take a look at the [CPUMesh](https://github.com/asny/three-d/blob/master/src/objects/cpu_mesh.rs) implementation.
