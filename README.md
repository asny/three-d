# `three-d`

[![crates.io](https://img.shields.io/crates/v/three-d.svg)](https://crates.io/crates/three-d)
[![Docs.rs](https://docs.rs/three-d/badge.svg)](https://docs.rs/three-d)
[![Continuous integration](https://github.com/asny/three-d/actions/workflows/rust.yml/badge.svg)](https://github.com/asny/three-d/actions/workflows/rust.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/asny/three-d/blob/master/LICENSE)

### What is it?

A 3D renderer which enables out-of-the-box build to both desktop (Rust + OpenGL) and web
(Rust to WebAssembly + WebGL2).
This makes it possible to develop a 3D application on desktop and easily deploy it on web!

### Supported browsers

Chrome, Firefox, Edge and Safari (Safari requires enabling the "WebGL 2.0" experimental feature).

### Examples

Several examples covering most features can be found [here](https://github.com/asny/three-d/tree/master/examples) and the examples from the latest release are also live at [asny.github.io/three-d/](https://asny.github.io/three-d/).
Take a look at the [triangle example](https://github.com/asny/three-d/blob/master/examples/triangle/main.rs) for a gentle introduction.

![Statues example](https://asny.github.io/three-d/statues.png)
![Lighting example](https://asny.github.io/three-d/lighting.png)
![Spider example](https://asny.github.io/three-d/spider.png)

### Features

| Feature           | Description                                                                                                                                                                                                                       |               Examples               |       `[features]`       |
| :---------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------: | :----------------------: |
| Context           | Thin and low-level graphics abstraction layer which maps one-to-one with the OpenGL/WebGL2 graphics APIs.                                                                                                                         |                                      |
| Graphics concepts | Modular abstractions of common graphics concepts such as buffer, texture, program and render target.                                                                                                                              |
| Camera            | Orthographic and perspective camera which has functionality for navigation and frustum culling queries.                                                                                                                           | [Mandelbrot], [Statues], [Fireworks] |
| Light             | Light definitions which is put in a uniform buffer. Currently implemented light types are ambient light, directional light, spot light and point light. Directional and spot lights has functionality for shadow mapping.         |  [Statues], [Lighting], [Wireframe]  |
| Mesh              | A triangle mesh object with fixed vertex shader and customizable fragment shader for customizable lighting. Supports rendering the depth and also with a fixed color and with a texture (ie. no lighting).                        |       [Triangle], [Mandelbrot]       |
| Instanced mesh    | Similar to Mesh, except it is possible to draw many instances of the same triangle mesh efficiently.                                                                                                                              |  [Wireframe], [Fireworks], [Forest]  |
| Skybox            | An illusion of a sky.                                                                                                                                                                                                             |           [Texture], [Fog]           |
| Particles         | Particle effect with fixed vertex shader and customizable fragment shader.                                                                                                                                                        |             [Fireworks]              |
| Imposters         | A level-of-detail technique to replace rendering high-poly meshes at a distance. A mesh is rendered from different angles into a set of textures and the textures are then rendered continuously instead of the high-poly meshes. |               [Forest]               |
| Image effect      | A customizable effect applied to each pixel of a render target, for example fog or anti-aliasing.                                                                                                                                 |                [Fog]                 |
| Phong renderer    | Rendering functionality based on the phong reflection model supporting a performance-limited amount of directional, point and spot lights with shadows.                                                                           |  [Statues], [Lighting], [Wireframe]  |     `phong-renderer`     |
| Runtime loading   | Loading any type of asset runtime on both desktop and web.                                                                                                                                                                        |    [Statues], [Forest], [Texture]    |
| 3D model parsers  | Built-in parsers for .obj (using the [wavefront-obj](https://crates.io/crates/wavefront_obj/main.rs) crate) and .3d files (a custom format).                                                                                      |    [Statues], [Forest], [Texture]    |     `3d-io` `obj-io`     |
| Image parsers     | Most image formats are supported (using the [image](https://crates.io/crates/image/main.rs) crate).                                                                                                                               |         [Texture], [Statues]         |        `image-io`        |
| GUI               | Immidiate mode GUI support using the [egui](https://crates.io/crates/egui) crate.                                                                                                                                                 |              [Lighting]              |        `egui-gui`        |
| Window            | Default windows for easy setup and event handling. Currently [glutin](https://crates.io/crates/glutin/main.rs) for cross-platform desktop and canvas for web.                                                                     |                [All]                 | `glutin-window` `canvas` |

It is always possible to combine features, for example rendering particles followed by direct calls to the graphics context.

### Build

#### Desktop:

Build and run an example, in this case 'triangle':

```console
$ cargo run --example triangle --release
```

#### Web:

Prerequisites: 
- A server that properly defines the `application/wasm` mime type (for example [http-server](https://www.npmjs.com/package/http-server))
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)

Build and generate web output (webassembly, javascript and html files) into the pkg folder:

```console
$ wasm-pack build examples/triangle --target web --out-name web --out-dir ../../pkg
```

Start the server and go to http://localhost:8080 in a browser:

```console
$ http-server
```

### Other

Feature requests and bug reports are more than welcome, just open an issue or start a discussion. Contributions are highly appreciated, please feel free to reach out or simply create a pull request.

[all]: https://github.com/asny/three-d/tree/master/examples/
[lighting]: https://github.com/asny/three-d/tree/master/examples/lighting/main.rs
[texture]: https://github.com/asny/three-d/tree/master/examples/texture/main.rs
[fog]: https://github.com/asny/three-d/tree/master/examples/fog/main.rs
[fireworks]: https://github.com/asny/three-d/tree/master/examples/fireworks/main.rs
[statues]: https://github.com/asny/three-d/tree/master/examples/statues/main.rs
[forest]: https://github.com/asny/three-d/tree/master/examples/forest/main.rs
[triangle]: https://github.com/asny/three-d/tree/master/examples/triangle/main.rs
[mandelbrot]: https://github.com/asny/three-d/tree/master/examples/mandelbrot/main.rs
[wireframe]: https://github.com/asny/three-d/tree/master/examples/wireframe/main.rs
