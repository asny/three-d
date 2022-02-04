![Logo](https://asny.github.io/three-d/0.10/logo.png)

# `three-d`

[![crates.io](https://img.shields.io/crates/v/three-d.svg)](https://crates.io/crates/three-d)
[![Docs.rs](https://docs.rs/three-d/badge.svg)](https://docs.rs/three-d)
[![Continuous integration](https://github.com/asny/three-d/actions/workflows/rust.yml/badge.svg)](https://github.com/asny/three-d/actions/workflows/rust.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/asny/three-d/blob/master/LICENSE)



### What is it?

A renderer which seeks to make graphics simple but still have the power to draw exactly what you want.

`three-d` 
- makes it possible to combine high-level features with custom shaders so you can focus on the important stuff.
- can be used by those without any graphics experience and who just want to draw something.
- tries to do stuff in a few simple lines of code.
- aims to be as explicit as possible so there is no surprices for you - no hidden magic.
- targets both desktop (using OpenGL graphics API) and web (compiled to WebAssembly and using WebGL2 graphics API).

`three-d` can for example be used for
- data visualization
- image processing
- UI rendering
- tools (2D or 3D)
- games (2D or 3D)

The crate consist of three main modules for drawing, `context`, `core` and `renderer`, and two optional utility modules, `io` and `window`:

| Module           | Description                   
| :---------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | 
| [`context`](https://docs.rs/three-d/0/three_d/context/) | Low-level rendering module - requires a solid understanding of graphics concepts. Gives you complete control over both setup and rendering.                             
| [`core`](https://docs.rs/three-d/0/three_d/core/) | Mid-level rendering module - requires at least some knowledge about graphics concepts. Use this if you want to write your own shaders and but don't want to spend time on setup and error handling. Can be combined with low-level calls in the `context` module as long as any graphics state changes are reset.                                                                                                                           
| [`renderer`](https://docs.rs/three-d/0/three_d/renderer/)  | High-level rendering module - requires no knowledge about graphics concepts. Use this if you just want to draw something on the screen. Features include methods for rendering different types of standard objects with different types of shading. Can be combined seamlessly with the mid-level features in the `core` module and also with calls in the `context` module as long as the graphics state is reset.             |
| [`io`](https://docs.rs/three-d/0/three_d/io/) | Contains functionality to load any type of asset runtime on both desktop and web as well as parsers for different image and 3D model formats. Also includes functionality to save data which is limited to desktop.
| [`window`](https://docs.rs/three-d/0/three_d/window/)  | Contains functionality for creating a window on both cross-platform desktop (requires the `"glutin-window"` feature) and web (requires the `"canvas"` feature). Also contain render loop, event handling and camera control functionality. Can be replaced by anything that provides an OpenGL or WebGL2 graphics context. 

### Examples

![PBR example](https://asny.github.io/three-d/0.10/pbr.png)
Several examples covering most features can be found in the [examples folder](https://github.com/asny/three-d/tree/master/examples) where you will also find an overview and links where you can try out each example on web. 
Check out the [triangle example](https://github.com/asny/three-d/blob/master/examples/triangle/main.rs) for a gentle introduction. 
The examples that fit with a specific release can be found in the branch for that release, ie. the examples for the [0.9 release](https://crates.io/crates/three-d/0.9.0) can be found in the [0.9 branch](https://github.com/asny/three-d/tree/0.9/examples).

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

### Supported browsers

Chrome, Firefox, Edge and Safari.

### Other

Feature requests and bug reports are more than welcome, just open an issue or start a discussion. Contributions are highly appreciated, please feel free to reach out or simply create a pull request against the [master branch](https://github.com/asny/three-d/tree/master).
