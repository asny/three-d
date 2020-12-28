# `three-d`

[![](http://meritbadge.herokuapp.com/three-d)](https://crates.io/crates/three-d)

### What is it?

A 3D renderer which enables out-of-the-box build to both desktop (Rust + OpenGL) and web 
(Rust to WebAssembly using [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) + WebGL2 bindings provided by the [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) crate).
This means you can develop a 3D application on desktop and easily deploy it on web!

### Supported browsers

Chrome, Firefox, Edge and Safari (Safari requires enabling the "WebGL 2.0" experimental feature).

### Examples

https://asny.github.io/three-d/

![Lighting example](https://asny.github.io/three-d/lighting.png)
![Spider example](https://asny.github.io/three-d/spider.png)

### Main features

- Thin and low-level graphics abstraction layer which maps one-to-one with the OpenGL/WebGL2 graphics APIs.
- Modular abstractions of common graphics concepts such as buffer, texture, camera, program, rendertarget etc. 
It is always possible to combine with direct call to the OpenGL/WebGL2 graphics abstraction layer.
- Effects applied after rendering the scene (for example FXAA) and skybox.
- Renderer which is based on the phong reflection model and which enables both forward and deferred rendering.
It supports instancing, textures and multiple light types including shadows. 
Again, it is always possible to combine with lower-level functionality and it can be avoided altogether by disabling the "phong-renderer" feature.
- Default windows for easy setup (currently [glutin](https://crates.io/crates/glutin) for cross-platform desktop and canvas for web). 
Can be avoided by disabling the "glutin-window" feature and "canvas" feature respectively.
- A loader for loading any type of asset runtime on both desktop and web. 
Built-in parsers for images, .obj and .3d files (the latter is a custom format). All loading features can be disabled.

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
$ http-server
``` 

#### Desktop and Web: 
Build and run an example on desktop and also generate web output (webassembly, javascript and html files) into the pkg folder:
```console
$ ./examples/hello_world/run 
``` 

### Other:
Feature requests and bug reports are more than welcome, just open an issue. Contributions are highly appreciated, please feel free to reach out to me or simply create a pull request.