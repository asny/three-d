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
