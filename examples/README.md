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

## Triangle [[code](https://github.com/asny/three-d/tree/master/examples/triangle/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/triangle.html)]

This is the recommended starting point for a gentle introduction to `three-d`.

![Triangle example](https://asny.github.io/three-d/0.19/triangle.png)

## Triangle core [[code](https://github.com/asny/three-d/tree/master/examples/triangle_core/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/triangle_core.html)]

This is the same as the `Triangle` example, except it only uses the core module and not the renderer module.

![Triangle core example](https://asny.github.io/three-d/0.19/triangle_core.png)

## Shapes2D [[code](https://github.com/asny/three-d/tree/master/examples/shapes2d/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/shapes2d.html)]

![Shapes2d example](https://asny.github.io/three-d/0.19/shapes2d.png)

## Shapes [[code](https://github.com/asny/three-d/tree/master/examples/shapes/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/shapes.html)]

![Shapes example](https://asny.github.io/three-d/0.19/shapes.png)

## Mandelbrot [[code](https://github.com/asny/three-d/tree/master/examples/mandelbrot/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/mandelbrot.html)]

![Mandelbrot example](https://asny.github.io/three-d/0.19/mandelbrot.png)

## Lights [[code](https://github.com/asny/three-d/tree/master/examples/lights/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/lights.html)]

![Lights example](https://asny.github.io/three-d/0.19/lights.png)

## Terrain [[code](https://github.com/asny/three-d/tree/master/examples/terrain/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/terrain.html)]

![Terrain example](https://asny.github.io/three-d/0.19/terrain.png)

## Environment [[code](https://github.com/asny/three-d/tree/master/examples/environment/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/environment.html)]

![Environment example](https://asny.github.io/three-d/0.19/environment.png)

## PBR [[code](https://github.com/asny/three-d/tree/master/examples/pbr/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/pbr.html)]

![PBR example](https://asny.github.io/three-d/0.19/pbr.png)

## Statues [[code](https://github.com/asny/three-d/tree/master/examples/statues/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/statues.html)]

![Statues example](https://asny.github.io/three-d/0.19/statues.png)

## Screen [[code](https://github.com/asny/three-d/tree/master/examples/screen/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/screen.html)]

![Screen example](https://asny.github.io/three-d/0.19/screen.png)

## Text [[code](https://github.com/asny/three-d/tree/master/examples/text/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/text.html)]

![Text example](https://asny.github.io/three-d/0.19/text.png)

## Image [[code](https://github.com/asny/three-d/tree/master/examples/image/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/image.html)]

![Image example](https://asny.github.io/three-d/0.19/image.png)

## Lighting [[code](https://github.com/asny/three-d/tree/master/examples/lighting/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/lighting.html)]

![Lighting example](https://asny.github.io/three-d/0.19/lighting.png)

## Sprites [[code](https://github.com/asny/three-d/tree/master/examples/sprites/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/sprites.html)]

![Sprites example](https://asny.github.io/three-d/0.19/sprites.png)

## Texture [[code](https://github.com/asny/three-d/tree/master/examples/texture/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/texture.html)]

![Texture example](https://asny.github.io/three-d/0.19/texture.png)

## Instanced Shapes [[code](https://github.com/asny/three-d/tree/master/examples/instanced_shapes/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/instanced_shapes.html)]

![Instanced Shapes example](https://asny.github.io/three-d/0.19/instanced_shapes.png)

## Multisample [[code](https://github.com/asny/three-d/tree/master/examples/multisample/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/multisample.html)]

![Screen example](https://asny.github.io/three-d/0.19/multisample.png)

## Picking [[code](https://github.com/asny/three-d/tree/master/examples/picking/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/picking.html)]

![Picking example](https://asny.github.io/three-d/0.19/picking.png)

## Animation [[code](https://github.com/asny/three-d/tree/master/examples/animation/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/animation.html)]

![Animation example](https://asny.github.io/three-d/0.19/animation.png)

## Volume [[code](https://github.com/asny/three-d/tree/master/examples/volume/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/volume.html)]

![Volume example](https://asny.github.io/three-d/0.19/volume.png)

## Point cloud [[code](https://github.com/asny/three-d/tree/master/examples/point_cloud/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/point_cloud.html)]

![Point cloud example](https://asny.github.io/three-d/0.19/point_cloud.png)

## Effect [[code](https://github.com/asny/three-d/tree/master/examples/effect/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/effect.html)]

![Effect example](https://asny.github.io/three-d/0.19/effect.png)

## Particles [[code](https://github.com/asny/three-d/tree/master/examples/particles/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/particles.html)]

![Particles example](https://asny.github.io/three-d/0.19/particles.png)

## Wireframe [[code](https://github.com/asny/three-d/tree/master/examples/wireframe/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/wireframe.html)]

![Wireframe example](https://asny.github.io/three-d/0.19/wireframe.png)

## Imposters [[code](https://github.com/asny/three-d/tree/master/examples/imposters/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/imposters.html)]

![Imposters example](https://asny.github.io/three-d/0.19/imposters.png)

## Instanced Draw Order [[code](https://github.com/asny/three-d/tree/master/examples/instanced_draw_order/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/instanced_draw_order.html)]

This example shows how depth ordering is currently working for `InstancedMesh` objects with transparency.

![Instanced Draw Order](https://asny.github.io/three-d/0.19/instanced_draw_order.png)

## Normals [[code](https://github.com/asny/three-d/tree/master/examples/normals/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/normals.html)]

![Normals example](https://asny.github.io/three-d/0.19/normals.png)

## Logo [[code](https://github.com/asny/three-d/tree/master/examples/logo/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/logo.html)]

![Logo example](https://asny.github.io/three-d/0.19/logo.png)

## Winit window [[code](https://github.com/asny/three-d/tree/master/examples/winit_window/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/winit_window.html)]

Shows how to easily combine a custom [winit](https://crates.io/crates/winit) window with `three-d` rendering.

![Winit window example](https://asny.github.io/three-d/0.19/winit_window.png)

## Multiwindow [[code](https://github.com/asny/three-d/tree/master/examples/multiwindow/src/main.rs)] [[demo](https://asny.github.io/three-d/0.19/multiwindow.html)]

Shows how to create multiple [winit](https://crates.io/crates/winit) windows and render with `three-d`.

## Headless [[code](https://github.com/asny/three-d/tree/master/examples/headless/src/main.rs)]

This example does not create a window but render directly to a render target and saves the result to disk. Therefore, this example does not work on web.
