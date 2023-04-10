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
If your application is native only, you can avoid the async runtime (`tokio` or `async-std`) and use `three_d_asset::load` instead of `three_d_asset::load_async`.

## Triangle [[code](https://github.com/asny/three-d/tree/master/examples/triangle/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/triangle.html)]

This is the recomended starting point for a gentle introduction to `three-d`.

![Triangle example](https://asny.github.io/three-d/0.16/triangle.png)

## Mandelbrot [[code](https://github.com/asny/three-d/tree/master/examples/mandelbrot/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/mandelbrot.html)]

![Mandelbrot example](https://asny.github.io/three-d/0.16/mandelbrot.png)

## Shapes2D [[code](https://github.com/asny/three-d/tree/master/examples/shapes2d/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/shapes2d.html)]

![Shapes2d example](https://asny.github.io/three-d/0.16/shapes2d.png)

## Shapes [[code](https://github.com/asny/three-d/tree/master/examples/shapes/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/shapes.html)]

![Shapes example](https://asny.github.io/three-d/0.16/shapes.png)

## Instanced Draw Order [[code](https://github.com/asny/three-d/tree/master/examples/instanced_draw_order/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/instanced_draw_order.html)]

This example shows how depth ordering is currently working for `InstancedMesh` objects with transparency.

![Instanced Draw Order](https://asny.github.io/three-d/0.16/instanced_draw_order.png)

## Instanced Shapes [[code](https://github.com/asny/three-d/tree/master/examples/instanced_shapes/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/instanced_shapes.html)]

![Instanced Shapes example](https://asny.github.io/three-d/0.16/instanced_shapes.png)

## Screen [[code](https://github.com/asny/three-d/tree/master/examples/screen/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/screen.html)]

![Screen example](https://asny.github.io/three-d/0.16/screen.png)

## Multisample [[code](https://github.com/asny/three-d/tree/master/examples/multisample/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/multisample.html)]

![Screen example](https://asny.github.io/three-d/0.16/multisample.png)

## Sprites [[code](https://github.com/asny/three-d/tree/master/examples/sprites/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/sprites.html)]

![Sprites example](https://asny.github.io/three-d/0.16/sprites.png)

## Texture [[code](https://github.com/asny/three-d/tree/master/examples/texture/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/texture.html)]

![Texture example](https://asny.github.io/three-d/0.16/texture.png)

## Animation [[code](https://github.com/asny/three-d/tree/master/examples/animation/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/animation.html)]

![Animation example](https://asny.github.io/three-d/0.16/animation.png)

## Picking [[code](https://github.com/asny/three-d/tree/master/examples/picking/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/picking.html)]

![Picking example](https://asny.github.io/three-d/0.16/picking.png)

## Environment [[code](https://github.com/asny/three-d/tree/master/examples/environment/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/environment.html)]

![Environment example](https://asny.github.io/three-d/0.16/environment.png)

## PBR [[code](https://github.com/asny/three-d/tree/master/examples/pbr/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/pbr.html)]

![PBR example](https://asny.github.io/three-d/0.16/pbr.png)

## Lighting [[code](https://github.com/asny/three-d/tree/master/examples/lighting/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/lighting.html)]

![Lighting example](https://asny.github.io/three-d/0.16/lighting.png)

## Lights [[code](https://github.com/asny/three-d/tree/master/examples/lights/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/lights.html)]

![Lights example](https://asny.github.io/three-d/0.16/lights.png)

## Image [[code](https://github.com/asny/three-d/tree/master/examples/image/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/image.html)]

![Image example](https://asny.github.io/three-d/0.16/image.png)

## Point cloud [[code](https://github.com/asny/three-d/tree/master/examples/point_cloud/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/point_cloud.html)]

![Point cloud example](https://asny.github.io/three-d/0.16/point_cloud.png)

## Fog [[code](https://github.com/asny/three-d/tree/master/examples/fog/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/fog.html)]

![Fog example](https://asny.github.io/three-d/0.16/fog.png)

## Terrain [[code](https://github.com/asny/three-d/tree/master/examples/terrain/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/terrain.html)]

![Terrain example](https://asny.github.io/three-d/0.16/terrain.png)

## Fireworks [[code](https://github.com/asny/three-d/tree/master/examples/fireworks/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/fireworks.html)]

![Fireworks example](https://asny.github.io/three-d/0.16/fireworks.png)

## Statues [[code](https://github.com/asny/three-d/tree/master/examples/statues/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/statues.html)]

![Statues example](https://asny.github.io/three-d/0.16/statues.png)

## Wireframe [[code](https://github.com/asny/three-d/tree/master/examples/wireframe/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/wireframe.html)]

![Wireframe example](https://asny.github.io/three-d/0.16/wireframe.png)

## Forest [[code](https://github.com/asny/three-d/tree/master/examples/forest/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/forest.html)]

![Forest example](https://asny.github.io/three-d/0.16/forest.png)

## Volume [[code](https://github.com/asny/three-d/tree/master/examples/volume/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/volume.html)]

![Volume example](https://asny.github.io/three-d/0.16/volume.png)

## Normals [[code](https://github.com/asny/three-d/tree/master/examples/normals/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/normals.html)]

![Normals example](https://asny.github.io/three-d/0.16/normals.png)

## Logo [[code](https://github.com/asny/three-d/tree/master/examples/logo/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/logo.html)]

![Logo example](https://asny.github.io/three-d/0.16/logo.png)

## Winit window [[code](https://github.com/asny/three-d/tree/master/examples/winit_window/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/winit_window.html)]

Shows how to easily combine a custom [winit](https://crates.io/crates/winit) window with `three-d` rendering.

![Winit window example](https://asny.github.io/three-d/0.16/winit_window.png)

## Multiwindow [[code](https://github.com/asny/three-d/tree/master/examples/multiwindow/src/main.rs)] [[demo](https://asny.github.io/three-d/0.16/multiwindow.html)]

Shows how to create multiple [winit](https://crates.io/crates/winit) windows and render with `three-d`.

![Multiwindow example](https://asny.github.io/three-d/0.16/multiwindow.png)

## Headless [[code](https://github.com/asny/three-d/tree/master/examples/headless/src/main.rs)]

This example does not create a window but render directly to a render target and saves the result to disk. Therefore, this example does not work on web.
