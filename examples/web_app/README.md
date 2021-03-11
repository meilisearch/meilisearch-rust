# Build your front-end page in Rust with WebAssembly

> **Note**
> It is not possible to run MeiliSearch in the browser without a server. This demo uses the Rust SDK in a browser using WASM, and communicate with a MeiliSearch instance that is running on a remote server.

This example is a clone of [crates.meilisearch.com](https://crates.meilisearch.com), but the front-end is written in Rust!
The Rust source files are compiled into WebAssembly and so can be readable by the browsers.

## Checking

If you only want to check if this example compiles, you can run:

```console
cargo build --example web_app
```

## Building

To build this example, you need [wasm-pack](https://github.com/rustwasm/wasm-pack).\
You can install `wasm-pack` with this command:
```console
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

```console
wasm-pack build examples/web_app/ --target=web --no-typescript
```

The compiled files will be stored in the `examples/web_app/pkg` folder.

## Using

Theoretically, you could just open the `examples/web_app/pkg/index.html` file but due to browsers' security restrictions, you need a web server. For example:

```console
python3 -m http.server 8080
```

And then go to the `http://localhost:8080/` URL in your browser.
