# Building for the web

In order to build the three-d examples for the browser, you can follow these steps. **Make sure you have both Rust and npm (which should include `npx`) installed.**

You can find an overview of all examples in `examples/README.md`.

## Step by step

1. Run `npm install` in the `web/` directory
2. Run the following command to build the `shapes` example. Replace the path to the example to build other examples
    * `npx wasm-pack build "../examples/shapes" --target web --out-name web --out-dir ../../web/pkg`
3. Run `npm run serve` to show the application in the browser
4. Open `http://localhost:8080` in the browser
