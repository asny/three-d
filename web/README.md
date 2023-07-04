# Build for the web

In order to build the `three-d` examples for the web, you can follow these steps. All commands should run in this `web/` directory.

1. Make sure you have both `Rust` and `npm` (which should include `npx`) installed.

2. Run

```console
$ npm install
```

3. Run (The following command builds the `triangle` example. Replace the path to the example to build other examples. You can find an overview of all examples in `examples/README.md`.)

```console
$ npx wasm-pack build "../examples/triangle" --target web --out-name web --out-dir ../../web/pkg
```

4. Run

```console
$ npm run serve
```

5. Open `http://localhost:8080` in a browser
