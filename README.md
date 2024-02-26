<div align="center">
  <img width="50%" src="https://asny.github.io/three-d/0.17/logo.png" alt="three-d logo">
</div>

# `three-d`

[![crates.io](https://img.shields.io/crates/v/three-d.svg)](https://crates.io/crates/three-d)
[![Docs.rs](https://docs.rs/three-d/badge.svg)](https://docs.rs/three-d)
[![Continuous integration](https://github.com/asny/three-d/actions/workflows/rust.yml/badge.svg)](https://github.com/asny/three-d/actions/workflows/rust.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/asny/three-d/blob/master/LICENSE)

### What is it?

A OpenGL/WebGL/OpenGL ES renderer which seeks to make graphics simple but still have the power to efficiently draw exactly what you want.

`three-d`

- targets those who just want to draw something and those who want to avoid the tedious setup but still wants low-level access.
- makes it possible to combine high-level features with custom low-level implementations for example custom shaders.
- tries to do stuff in a few simple lines of code.
- aims to be as explicit as possible so there is no surprises for you - no hidden magic.
- targets desktop, web and mobile.

`three-d` can for example be used for

- data visualization
- image processing
- UI rendering
- tools (2D or 3D)
- games (2D or 3D)

### Quotes

Quotes from issues, discussions and pull requests:

<div align="center">
<em>

..., thanks for writing such a nice 3d crate. I'm able to get what I want done without thousands of lines of boilerplate. It's truly a pleasure.

I must say: Man, you did a hell of a job with this library! It's really beyond performant, easy and understanable to use and I am grateful, you made it open source! Thank you so much.

Thank you for making such a great 3d tool, It has been such a great use to me as most other rust based 3d tooling is either too low level, or a whole fully fledged game engine which does not fit my needs.

First off, three-d is a joy to work with, even for someone with little experience in Rust (though admittedly, I do have some experience working with OpenGL).

Thanks for offering a really cool and simple to get started 3D Library in rust :)

Hi, thanks for such a cool and easy-to-use project!

Thanks for this library, it's been a joy to work with so far!

First of, Great job for the library and therefore thank you!

Thanks so much for this library, it's looking excellent!

Thanks for the great project.

This project has been wonderful to use so far, thank you so much for you time and dedication.

</em>

</div>

### Structure

The crate consist of three main modules for drawing, `context`, `core` and `renderer`, and an optional `window` module for easy setup:

| Module                                                                                  | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :-------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`context`](https://docs.rs/three-d/0/three_d/context/)                                 | Low-level rendering module - requires a solid understanding of graphics concepts. Gives you complete control over both setup and rendering.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| [`core`](https://docs.rs/three-d/0/three_d/core/)                                       | Mid-level rendering module - requires at least some knowledge about graphics concepts. Use this if you want to write your own shaders, but don't want to spend time on setup and error handling. Can be combined with low-level functionality in the `context` module.                                                                                                                                                                                                                                                                                                                                                                                                                        |
| [`renderer`](https://docs.rs/three-d/0/three_d/renderer/)                               | High-level rendering module - requires no knowledge about graphics concepts. Use this if you just want to draw something. Features include functionality to rendering different types of standard objects with different types of shading. Can be combined seamlessly with the mid-level features in the `core` module as well as functionality in the `context` module.                                                                                                                                                                                                                                                                                                                      |
| [`window`](https://docs.rs/three-d/0/three_d/window/) (requires the `"window"` feature) | Window functionality on cross-platform native and web, which primarily is provided to make it easy to get started. In some cases, it is desirable to replace the default `Window` with a custom [winit](https://github.com/rust-windowing/winit) window as exemplified in the `winit_window` example. However, this module can also be replaced entirely by anything that provides an OpenGL or WebGL2 graphics context, for example [winit](https://github.com/rust-windowing/winit) and [glutin](https://github.com/rust-windowing/glutin), [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/introduction.html) or [eframe](https://github.com/emilk/egui/tree/master/crates/eframe). |

In addition, the [three-d-asset](https://github.com/asny/three-d-asset) crate enables loading, deserializing, serializing and saving 3D assets, for example 3D models, textures etc. Please make sure to use the same version of [three-d-asset](https://github.com/asny/three-d-asset) as defined in the `Cargo.toml`.

### [Examples](https://github.com/asny/three-d/tree/0.17/examples)

![PBR example](https://asny.github.io/three-d/0.17/pbr.png)

Several examples covering most features can be found in the [examples folder](https://github.com/asny/three-d/tree/0.17/examples).

Here you will also find an overview of the examples, build instructions and links to the web version of the examples.

The examples that fit with a specific release can be found in the branch for that release, ie. the examples for the [0.9 release](https://crates.io/crates/three-d/0.9.0) can be found in the [0.9 branch](https://github.com/asny/three-d/tree/0.9/examples).

### Support

`three-d` is supported

- on all major desktop OS (Windows, Mac, Linux) by using OpenGL 3.3 graphics API. _Note: MacOS has deprecated OpenGL so it still works, but it might not perform optimally._
- in all major browsers (Chrome, Firefox, Edge and Safari) by compiling to WebAssembly and using the WebGL 2.0 graphics API (see `web/README.md`)
- on embedded/mobile systems with OpenGL ES 3.0 support. _Note: this is not tested regularly, please report any issues._

### State of the project

Most parts are relatively stable, but do expect regular breaking changes until a 1.0.0 release.
The master branch is work in progress, so if you want to avoid too many breaking changes and features not working, use a released version.
If possible and feasible, functionality will be deprecated in one release, before it is removed in the next release.

### Contributing

Feature requests and bug reports are more than welcome; just open an issue or start a discussion. Contributions are highly appreciated; please feel free to reach out or simply create a pull request against the [master branch](https://github.com/asny/three-d/tree/master). To avoid waste of time, please reach out before making major changes.
