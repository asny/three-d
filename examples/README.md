
# Examples

### Build

#### Native:

Build and run an example, in this case 'triangle':

```console
$ cargo run --example triangle --release
```

#### WebAssembly:

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

### A note on async

All of the examples builds to both native (desktop, mobile or whatever target specified) and WebAssembly (wasm) that can be run in a browser. 
Because they should run in a browser and to keep the same code for native and wasm, all loading happens async. 
If your application is native only, you can avoid the async runtime (`tokio` or `async-std`) and use `Loader::load_blocking` instead of `Loader::load_async`.

## Triangle [[code](https://github.com/asny/three-d/tree/master/examples/triangle/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/triangle.html)]

This is the recomended starting point for a gentle introduction to `three-d`. 

![Triangle example](https://asny.github.io/three-d/0.11/triangle.png)

## Mandelbrot [[code](https://github.com/asny/three-d/tree/master/examples/mandelbrot/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/mandelbrot.html)]

![Mandelbrot example](https://asny.github.io/three-d/0.11/mandelbrot.png)

## Shapes2D [[code](https://github.com/asny/three-d/tree/master/examples/shapes2d/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/shapes2d.html)]

![Shapes2d example](https://asny.github.io/three-d/0.11/shapes2d.png)

## Shapes [[code](https://github.com/asny/three-d/tree/master/examples/shapes/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/shapes.html)]

![Shapes example](https://asny.github.io/three-d/0.11/shapes.png)

## Screen [[code](https://github.com/asny/three-d/tree/master/examples/screen/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/screen.html)]

![Screen example](https://asny.github.io/three-d/0.11/screen.png)

## Sprites [[code](https://github.com/asny/three-d/tree/master/examples/sprites/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/sprites.html)]

![Sprites example](https://asny.github.io/three-d/0.11/sprites.png)

## Texture [[code](https://github.com/asny/three-d/tree/master/examples/texture/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/texture.html)]

![Texture example](https://asny.github.io/three-d/0.11/texture.png)

## Picking [[code](https://github.com/asny/three-d/tree/master/examples/picking/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/picking.html)]

![Picking example](https://asny.github.io/three-d/0.11/picking.png)

## Environment [[code](https://github.com/asny/three-d/tree/master/examples/environment/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/environment.html)]

![Environment example](https://asny.github.io/three-d/0.11/environment.png)

## PBR [[code](https://github.com/asny/three-d/tree/master/examples/pbr/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/pbr.html)]

![PBR example](https://asny.github.io/three-d/0.11/pbr.png)

## Lighting [[code](https://github.com/asny/three-d/tree/master/examples/lighting/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/lighting.html)]

![Lighting example](https://asny.github.io/three-d/0.11/lighting.png)

## Lights [[code](https://github.com/asny/three-d/tree/master/examples/lights/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/lights.html)]

![Lights example](https://asny.github.io/three-d/0.11/lights.png)

## Image [[code](https://github.com/asny/three-d/tree/master/examples/image/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/image.html)]

![Image example](https://asny.github.io/three-d/0.11/image.png)

## Fog [[code](https://github.com/asny/three-d/tree/master/examples/fog/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/fog.html)]

![Fog example](https://asny.github.io/three-d/0.11/fog.png)

## Fireworks [[code](https://github.com/asny/three-d/tree/master/examples/fireworks/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/fireworks.html)]

![Fireworks example](https://asny.github.io/three-d/0.11/fireworks.png)

## Statues [[code](https://github.com/asny/three-d/tree/master/examples/statues/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/statues.html)]

![Statues example](https://asny.github.io/three-d/0.11/statues.png)

## Wireframe [[code](https://github.com/asny/three-d/tree/master/examples/wireframe/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/wireframe.html)]

![Wireframe example](https://asny.github.io/three-d/0.11/wireframe.png)

## Forest [[code](https://github.com/asny/three-d/tree/master/examples/forest/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/forest.html)]

![Forest example](https://asny.github.io/three-d/0.11/forest.png)

## Volume [[code](https://github.com/asny/three-d/tree/master/examples/volume/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/volume.html)]

![Volume example](https://asny.github.io/three-d/0.11/volume.png)

## Normals [[code](https://github.com/asny/three-d/tree/master/examples/normals/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/normals.html)]

![Normals example](https://asny.github.io/three-d/0.11/normals.png)

## Logo [[code](https://github.com/asny/three-d/tree/master/examples/logo/src/main.rs)] [[demo](https://asny.github.io/three-d/0.11/logo.html)]

![Logo example](https://asny.github.io/three-d/0.11/logo.png)

## Headless [[code](https://github.com/asny/three-d/tree/master/examples/headless/src/main.rs)]

This example does not create a window but render directly to a render target and saves the result to disk. Therefore, this example does not work on web.
