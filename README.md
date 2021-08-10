# `three-d`

[![crates.io](https://img.shields.io/crates/v/three-d.svg)](https://crates.io/crates/three-d)
[![Docs.rs](https://docs.rs/three-d/badge.svg)](https://docs.rs/three-d)
[![Continuous integration](https://github.com/asny/three-d/actions/workflows/rust.yml/badge.svg)](https://github.com/asny/three-d/actions/workflows/rust.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/asny/three-d/blob/master/LICENSE)

### What is it?

A 3D renderer which enables out-of-the-box build to both desktop (Rust + OpenGL) and web
(Rust to WebAssembly + WebGL2).
This makes it possible to develop a 3D application on desktop and easily deploy it on both desktop and web!

The crate consist of four main modules:
| Module           | Description                   
| :---------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | 
| [`context`](https://docs.rs/three-d/0.7.3/three_d/context/) | Low-level graphics abstraction layer which maps one-to-one with the OpenGL graphics API on desktop and WebGL2 bindings provided by the [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) crate on web. Use this if you want to have complete control of a feature but be aware that there are no safety checks.                              
| [`core`](https://docs.rs/three-d/0.7.3/three_d/core/) | Mid-level modular abstractions of common graphics concepts such as buffer, texture, program, render target and so on. Can be combined with low-level calls in the `context` module as long as any graphics state changes are reset.                                                                                                                           
| [`renderer`](https://docs.rs/three-d/0.7.3/three_d/renderer/)  | High-level features for easy loading and rendering of different types of objects with different types of shading. Can be combined seamlessly with the mid-level features in the `core` module and also with calls in the `context` module as long as the graphics state is reset.             |
| [`window`](https://docs.rs/three-d/0.7.3/three_d/window/)  | Default windows for easy setup and event handling. Currently [glutin](https://crates.io/crates/glutin/main.rs) for cross-platform desktop (requires the `"glutin-window"` feature) and canvas for web (requires the `"canvas"` feature). Can be replaced by anything that provides an OpenGL or WebGL2 graphics context. Also contains camera control utilities.


### Supported browsers

Chrome, Firefox, Edge and Safari (Safari might requires enabling the "WebGL 2.0" experimental feature).

### Examples

Several examples covering most features can be found [here](https://github.com/asny/three-d/tree/master/examples) and the examples from the latest release are also live at [asny.github.io/three-d/](https://asny.github.io/three-d/).
Take a look at the [triangle example](https://github.com/asny/three-d/blob/master/examples/triangle/main.rs) for a gentle introduction.

![Lighting example](https://asny.github.io/three-d/lighting.png)
![Statues example](https://asny.github.io/three-d/statues.png)
![PBR example](https://asny.github.io/three-d/pbr.png)
![Spider example](https://asny.github.io/three-d/spider.png)

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
