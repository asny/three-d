![Logo](https://asny.github.io/three-d/0.14/logo.png)

# `three-d`

[![crates.io](https://img.shields.io/crates/v/three-d.svg)](https://crates.io/crates/three-d)
[![Docs.rs](https://docs.rs/three-d/badge.svg)](https://docs.rs/three-d)
[![Continuous integration](https://github.com/asny/three-d/actions/workflows/rust.yml/badge.svg)](https://github.com/asny/three-d/actions/workflows/rust.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/asny/three-d/blob/master/LICENSE)



### What is it?

A OpenGL/WebGL/OpenGL ES renderer which seeks to make graphics simple but still have the power to draw exactly what you want.

`three-d` 
- makes it possible to combine high-level features with custom shaders so you can focus on the important stuff.
- can be used by those without any graphics experience and who just want to draw something.
- tries to do stuff in a few simple lines of code.
- aims to be as explicit as possible so there is no surprices for you - no hidden magic.
- targets both desktop, web and mobile.

`three-d` can for example be used for
- data visualization
- image processing
- UI rendering
- tools (2D or 3D)
- games (2D or 3D)

The crate consist of three main modules for drawing, `context`, `core` and `renderer`, and an optional `window` module for easy setup:

| Module           | Description                   
| :---------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | 
| [`context`](https://docs.rs/three-d/0/three_d/context/) | Low-level rendering module - requires a solid understanding of graphics concepts. Gives you complete control over both setup and rendering.                             
| [`core`](https://docs.rs/three-d/0/three_d/core/) | Mid-level rendering module - requires at least some knowledge about graphics concepts. Use this if you want to write your own shaders and but don't want to spend time on setup and error handling. Can be combined with low-level functionality in the `context` module.                                                                                                                           
| [`renderer`](https://docs.rs/three-d/0/three_d/renderer/)  | High-level rendering module - requires no knowledge about graphics concepts. Use this if you just want to draw something on the screen. Features include methods for rendering different types of standard objects with different types of shading. Can be combined seamlessly with the mid-level features in the `core` module as well as functionality in the `context` module.             |
| [`window`](https://docs.rs/three-d/0/three_d/window/) (requires the `"window"` feature) | Window functionality on cross-platform native and web. This is primarily provided to make it easy to get started, however, in some cases it is desireable to replace this with a custom setup. It can be replaced by anything that provides an OpenGL or WebGL2 graphics context, for example [winit](https://github.com/rust-windowing/winit) and [glutin](https://github.com/rust-windowing/glutin) (the code in this `window` module can be used as a starting point), [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/introduction.html) and related crates (only on web) or [eframe](https://github.com/emilk/egui/tree/master/eframe) as shown in [this](https://github.com/emilk/egui/blob/master/eframe/examples/custom_3d_three-d.rs) example.

In addition, the [three-d-asset](https://github.com/asny/three-d-asset) crate enables loading, deserializing, serializing and saving 3D assets, for example 3D models, textures etc. 

### [Examples](https://github.com/asny/three-d/tree/0.14/examples)

![PBR example](https://asny.github.io/three-d/0.14/pbr.png)

Several examples covering most features can be found in the [examples folder](https://github.com/asny/three-d/tree/0.14/examples). 

Here you will also find an overview of the examples, build instructions and links where you can try out each example on web. 

The examples that fit with a specific release can be found in the branch for that release, ie. the examples for the [0.9 release](https://crates.io/crates/three-d/0.9.0) can be found in the [0.9 branch](https://github.com/asny/three-d/tree/0.9/examples).

### Support


`three-d` is supported
- on all major desktop OS (Windows, Mac, Linux) by using OpenGL 3.3 graphics API. *Note: MacOS has deprecated OpenGL so it still works, but it might not perform optimally.*
- in all major browsers (Chrome, Firefox, Edge and Safari) by compiling to WebAssembly and using the WebGL 2.0 graphics API. 
- on embedded/mobile systems with OpenGL ES 3.0 support. *Note: this is not tested regularly, please report any issues.*

### State of the project

Most parts are relatively stable, but do expect regular breaking changes until a 1.0.0 release.
The master branch is work in progress, so if you want to avoid too many breaking changes and features not working, use a released version.
If possible and feasible, functionality will be deprecated in one release before it is removed in the next release.

### Contributing

Feature requests and bug reports are more than welcome, just open an issue or start a discussion. Contributions are highly appreciated, please feel free to reach out or simply create a pull request against the [master branch](https://github.com/asny/three-d/tree/0.14). To avoid waste of time, please reach out before making major changes.

### Sponsors

[![rerun.io](https://asny.github.io/three-d/rerun.png)](https://www.rerun.io/)