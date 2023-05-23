#!/env/bin/env bash
echo "running: cargo run --example animation"
cargo run --example animation

echo "running: cargo run --example environment --features egui-gui"
cargo run --example environment --features egui-gui

echo "running: cargo run --example fireworks"
cargo run --example fireworks

echo "running: cargo run --example fog"
cargo run --example fog

echo "running: cargo run --example forest # blackscreen"
cargo run --example forest

echo "running: cargo run --example headless --features headless"
cargo run --example headless --features headless

echo "running: cargo run --example image --features egui-gui"
cargo run --example image --features egui-gui

echo "running: cargo run --example instanced_draw_order"
cargo run --example instanced_draw_order

echo "running: cargo run --example instanced_shapes --features egui-gui"
cargo run --example instanced_shapes --features egui-gui

echo "running: cargo run --example lighting --features egui-gui"
cargo run --example lighting --features egui-gui

echo "running: cargo run --example lights --features egui-gui"
cargo run --example lights --features egui-gui

echo "running: cargo run --example logo"
cargo run --example logo

echo "running: cargo run --example mandelbrot"
cargo run --example mandelbrot

echo "running: cargo run --example multisample --features egui-gui"
cargo run --example multisample --features egui-gui

echo "running: cargo run --example multiwindow"
cargo run --example multiwindow

echo "running: cargo run --example normals"
cargo run --example normals

echo "running: cargo run --example pbr --features egui-gui"
cargo run --example pbr --features egui-gui

echo "running: cargo run --example picking"
cargo run --example picking

echo "running: cargo run --example point_cloud"
cargo run --example point_cloud

echo "running: cargo run --example screen --features egui-gui"
cargo run --example screen --features egui-gui

echo "running: cargo run --example shapes"
cargo run --example shapes

echo "running: cargo run --example shapes2d"
cargo run --example shapes2d

echo "running: cargo run --example sprites"
cargo run --example sprites

echo "running: cargo run --example statues --features egui-gui"
cargo run --example statues --features egui-gui

echo "running: cargo run --example terrain --features egui-gui"
cargo run --example terrain --features egui-gui

echo "running: cargo run --example texture"
cargo run --example texture 

echo "running: cargo run --example triangle"
cargo run --example triangle

echo "running: cargo run --example triangle_core"
cargo run --example triangle_core

echo "running: cargo run --example volume --features egui-gui"
cargo run --example volume --features egui-gui

echo "running: cargo run --example winit_window"
cargo run --example winit_window

echo "running: cargo run --example wireframe"
cargo run --example wireframe
