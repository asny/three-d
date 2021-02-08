# `three-d`

[![](http://meritbadge.herokuapp.com/three-d)](https://crates.io/crates/three-d)

### What is it?

A 3D renderer which enables out-of-the-box build to both desktop (Rust + OpenGL) and web 
(Rust to WebAssembly using [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) + WebGL2 bindings provided by the [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) crate).
This means you can develop a 3D application on desktop and easily deploy it on web!

### Supported browsers

Chrome, Firefox, Edge and Safari (Safari requires enabling the "WebGL 2.0" experimental feature).

### Examples

See [https://asny.github.io/three-d/](https://asny.github.io/three-d/) for live examples.
The source code can be found [here](https://github.com/asny/three-d/tree/master/examples).

![Statues example](https://asny.github.io/three-d/statues.png)
![Lighting example](https://asny.github.io/three-d/lighting.png)
![Spider example](https://asny.github.io/three-d/spider.png)

### Features

Feature | Description | Examples | `[features]`
:--- |:---| :---: | :---:
Context | Thin and low-level graphics abstraction layer which maps one-to-one with the OpenGL/WebGL2 graphics APIs. |  |
Graphics concepts | Modular abstractions of common graphics concepts such as buffer, texture, program and render target. | [Triangle](https://github.com/asny/three-d/tree/master/examples/triangle), [Mandelbrot](https://github.com/asny/three-d/tree/master/examples/mandelbrot)
Camera | Orthographic and perspective camera which has functionality for navigation and frustum culling queries.  | [Mandelbrot](https://github.com/asny/three-d/tree/master/examples/mandelbrot), [Statues](https://github.com/asny/three-d/tree/master/examples/statues), [Fireworks](https://github.com/asny/three-d/tree/master/examples/fireworks)
Light | Light definitions which is put in a uniform buffer. Currently implemented light types are ambient light, directional light, spot light and point light. Directional and spot lights has functionality for shadow mapping. | [Statues](https://github.com/asny/three-d/tree/master/examples/statues), [Lighting](https://github.com/asny/three-d/tree/master/examples/lighting), [Wireframe](https://github.com/asny/three-d/tree/master/examples/wireframe)
Mesh | A triangle mesh object with fixed vertex shader and customizable fragment shader for customizable lighting. | [Texture](https://github.com/asny/three-d/tree/master/examples/texture), [Statues](https://github.com/asny/three-d/tree/master/examples/statues)
Instanced mesh | Similar to Mesh, except it is possible to draw many instances of the same triangle mesh efficiently. | [Wireframe](https://github.com/asny/three-d/tree/master/examples/wireframe), [Fireworks](https://github.com/asny/three-d/tree/master/examples/fireworks), [Forest](https://github.com/asny/three-d/tree/master/examples/forest)
Skybox | An illusion of a sky. | [Texture](https://github.com/asny/three-d/tree/master/examples/texture), [Fog](https://github.com/asny/three-d/tree/master/examples/fog)
Particles | Particle effect with fixed vertex shader and customizable fragment shader. | [Fireworks](https://github.com/asny/three-d/tree/master/examples/fireworks)
Imposters | A level-of-detail technique to replace rendering high-poly meshes at a distance. A mesh is rendered from different angles into a set of textures and the textures are then rendered continuously instead of the high-poly meshes. | [Forest](https://github.com/asny/three-d/tree/master/examples/forest)
Image effect | A customizable effect applied to each pixel of a render target, for example fog or anti-aliasing. | [Fog](https://github.com/asny/three-d/tree/master/examples/fog)
Phong forward pipeline | Forward pipeline based on the phong reflection model supporting a very limited amount of lights with shadows. Supports colored, transparent, textured and instanced meshes. | [Statues](https://github.com/asny/three-d/tree/master/examples/statues), [Fog](https://github.com/asny/three-d/tree/master/examples/fog), [Forest](https://github.com/asny/three-d/tree/master/examples/forest) | `phong-renderer`
Phong deferred pipeline | Deferred pipeline based on the phong reflection model supporting a performance-limited amount of directional, point and spot lights with shadows. Supports colored, textured and instanced meshes. | [Lighting](https://github.com/asny/three-d/tree/master/examples/lighting), [Wireframe](https://github.com/asny/three-d/tree/master/examples/wireframe), [Texture](https://github.com/asny/three-d/tree/master/examples/texture) | `phong-renderer`
Runtime loading | Loading any type of asset runtime on both desktop and web. | [Statues](https://github.com/asny/three-d/tree/master/examples/statues), [Forest](https://github.com/asny/three-d/tree/master/examples/forest), [Texture](https://github.com/asny/three-d/tree/master/examples/texture)
3D model parsers | Built-in parsers for .obj (using the [wavefront-obj](https://crates.io/crates/wavefront_obj) crate) and .3d files (a custom format). | [Statues](https://github.com/asny/three-d/tree/master/examples/statues), [Forest](https://github.com/asny/three-d/tree/master/examples/forest), [Texture](https://github.com/asny/three-d/tree/master/examples/texture) | `3d-io` `obj-io`
Image parsers | Most image formats are supported (using the [image](https://crates.io/crates/image) crate). | [Texture](https://github.com/asny/three-d/tree/master/examples/texture), [Statues](https://github.com/asny/three-d/tree/master/examples/statues) | `image-io`
Window | Default windows for easy setup and event handling. Currently [glutin](https://crates.io/crates/glutin) for cross-platform desktop and canvas for web. | [All](https://asny.github.io/three-d/) | `glutin-window` `canvas` 

It is always possible to combine features, for example rendering particles followed by direct calls to the graphics context.

### Build

#### Desktop: 
Build and run an example, in this case 'triangle':
```console
$ cargo run --example triangle --release
``` 
#### Web: 
Build and generate web output (webassembly, javascript and html files) into the pkg folder:
```console
$ wasm-pack build examples/triangle --target web --out-name web --out-dir ../../pkg
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
$ ./examples/triangle/run 
``` 

### Other
Feature requests and bug reports are more than welcome, just open an issue or start a discussion. Contributions are highly appreciated, please feel free to reach out or simply create a pull request.