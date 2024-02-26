# Examples

### Build

#### Native:

Build and run an example, in this case the `triangle` example:

```console
$ cargo run --release --example triangle
```

#### WebAssembly:

See `web/README.md`.

### A note on async

All of the examples builds to both native (desktop, mobile or whatever target specified) and WebAssembly (wasm) that can be run in a browser.
Because they should run in a browser and to keep the same code for native and wasm, all loading happens async.
If your application is native only, you can avoid the async runtime (`tokio` or `async-std`) and use `three_d_asset::load` instead of `three_d_asset::load_async`.

## Triangle [[code](https://github.com/asny/three-d/tree/0.17/examples/triangle/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/triangle.html)]

This is the recommended starting point for a gentle introduction to `three-d`.

![Triangle example](https://asny.github.io/three-d/0.17/triangle.png)

## Triangle core [[code](https://github.com/asny/three-d/tree/0.17/examples/triangle_core/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/triangle_core.html)]

This is the same as the `Triangle` example, except it only uses the core module and not the renderer module.

![Triangle core example](https://asny.github.io/three-d/0.17/triangle_core.png)

## Mandelbrot [[code](https://github.com/asny/three-d/tree/0.17/examples/mandelbrot/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/mandelbrot.html)]

![Mandelbrot example](https://asny.github.io/three-d/0.17/mandelbrot.png)

## Shapes2D [[code](https://github.com/asny/three-d/tree/0.17/examples/shapes2d/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/shapes2d.html)]

![Shapes2d example](https://asny.github.io/three-d/0.17/shapes2d.png)

## Shapes [[code](https://github.com/asny/three-d/tree/0.17/examples/shapes/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/shapes.html)]

![Shapes example](https://asny.github.io/three-d/0.17/shapes.png)

## Instanced Draw Order [[code](https://github.com/asny/three-d/tree/0.17/examples/instanced_draw_order/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/instanced_draw_order.html)]

This example shows how depth ordering is currently working for `InstancedMesh` objects with transparency.

![Instanced Draw Order](https://asny.github.io/three-d/0.17/instanced_draw_order.png)

## Instanced Shapes [[code](https://github.com/asny/three-d/tree/0.17/examples/instanced_shapes/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/instanced_shapes.html)]

![Instanced Shapes example](https://asny.github.io/three-d/0.17/instanced_shapes.png)

## Screen [[code](https://github.com/asny/three-d/tree/0.17/examples/screen/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/screen.html)]

![Screen example](https://asny.github.io/three-d/0.17/screen.png)

## Multisample [[code](https://github.com/asny/three-d/tree/0.17/examples/multisample/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/multisample.html)]

![Screen example](https://asny.github.io/three-d/0.17/multisample.png)

## Sprites [[code](https://github.com/asny/three-d/tree/0.17/examples/sprites/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/sprites.html)]

![Sprites example](https://asny.github.io/three-d/0.17/sprites.png)

## Texture [[code](https://github.com/asny/three-d/tree/0.17/examples/texture/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/texture.html)]

![Texture example](https://asny.github.io/three-d/0.17/texture.png)

## Animation [[code](https://github.com/asny/three-d/tree/0.17/examples/animation/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/animation.html)]

![Animation example](https://asny.github.io/three-d/0.17/animation.png)

## Picking [[code](https://github.com/asny/three-d/tree/0.17/examples/picking/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/picking.html)]

![Picking example](https://asny.github.io/three-d/0.17/picking.png)

## Environment [[code](https://github.com/asny/three-d/tree/0.17/examples/environment/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/environment.html)]

![Environment example](https://asny.github.io/three-d/0.17/environment.png)

## PBR [[code](https://github.com/asny/three-d/tree/0.17/examples/pbr/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/pbr.html)]

![PBR example](https://asny.github.io/three-d/0.17/pbr.png)

## Lighting [[code](https://github.com/asny/three-d/tree/0.17/examples/lighting/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/lighting.html)]

![Lighting example](https://asny.github.io/three-d/0.17/lighting.png)

## Lights [[code](https://github.com/asny/three-d/tree/0.17/examples/lights/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/lights.html)]

![Lights example](https://asny.github.io/three-d/0.17/lights.png)

## Image [[code](https://github.com/asny/three-d/tree/0.17/examples/image/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/image.html)]

![Image example](https://asny.github.io/three-d/0.17/image.png)

## Point cloud [[code](https://github.com/asny/three-d/tree/0.17/examples/point_cloud/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/point_cloud.html)]

![Point cloud example](https://asny.github.io/three-d/0.17/point_cloud.png)

## Fog [[code](https://github.com/asny/three-d/tree/0.17/examples/fog/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/fog.html)]

![Fog example](https://asny.github.io/three-d/0.17/fog.png)

## Terrain [[code](https://github.com/asny/three-d/tree/0.17/examples/terrain/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/terrain.html)]

![Terrain example](https://asny.github.io/three-d/0.17/terrain.png)

## Fireworks [[code](https://github.com/asny/three-d/tree/0.17/examples/fireworks/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/fireworks.html)]

![Fireworks example](https://asny.github.io/three-d/0.17/fireworks.png)

## Statues [[code](https://github.com/asny/three-d/tree/0.17/examples/statues/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/statues.html)]

![Statues example](https://asny.github.io/three-d/0.17/statues.png)

## Wireframe [[code](https://github.com/asny/three-d/tree/0.17/examples/wireframe/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/wireframe.html)]

![Wireframe example](https://asny.github.io/three-d/0.17/wireframe.png)

## Forest [[code](https://github.com/asny/three-d/tree/0.17/examples/forest/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/forest.html)]

![Forest example](https://asny.github.io/three-d/0.17/forest.png)

## Volume [[code](https://github.com/asny/three-d/tree/0.17/examples/volume/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/volume.html)]

![Volume example](https://asny.github.io/three-d/0.17/volume.png)

## Normals [[code](https://github.com/asny/three-d/tree/0.17/examples/normals/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/normals.html)]

![Normals example](https://asny.github.io/three-d/0.17/normals.png)

## Logo [[code](https://github.com/asny/three-d/tree/0.17/examples/logo/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/logo.html)]

![Logo example](https://asny.github.io/three-d/0.17/logo.png)

## Winit window [[code](https://github.com/asny/three-d/tree/0.17/examples/winit_window/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/winit_window.html)]

Shows how to easily combine a custom [winit](https://crates.io/crates/winit) window with `three-d` rendering.

![Winit window example](https://asny.github.io/three-d/0.17/winit_window.png)

## Multiwindow [[code](https://github.com/asny/three-d/tree/0.17/examples/multiwindow/src/main.rs)] [[demo](https://asny.github.io/three-d/0.17/multiwindow.html)]

Shows how to create multiple [winit](https://crates.io/crates/winit) windows and render with `three-d`.

## Headless [[code](https://github.com/asny/three-d/tree/0.17/examples/headless/src/main.rs)]

This example does not create a window but render directly to a render target and saves the result to disk. Therefore, this example does not work on web.
