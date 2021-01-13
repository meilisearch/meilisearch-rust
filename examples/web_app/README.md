# Build your front-end page in Rust with WebAssembly

> **Note**
> If you're looking to run MeiliSearch in a web browser using WASM, that is not possible yet as the Rust libraries used by MeiliSearch are not supporting WASM. Instead, this tutorial explains how to embed the whole frontend code in a WASM.

This example is a clone of [crates.meilisearch.com](https://crates.meilisearch.com), but the front-end is written in Rust!
The Rust source files are compiled into WebAssembly and so can be readable by the browsers.

## Checking

If you only want to check if this example compiles, you can run:

```console
$ cargo build --example web_app
```

## Building

To build this example, you need [wasm-pack](https://github.com/rustwasm/wasm-pack).\
You can install `wasm-pack` with this command:
```console
$ curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

```console
$ wasm-pack build examples/web_app/ --target=web --no-typescript
```

The compiled files will be stored in the `examples/web_app/pkg` folder.

## Using

Theoretically, you could just open the `examples/web_app/pkg/index.html` file but due to browsers' security restrictions, you need a web server. For example:

```console
$ python3 -m http.server 8080
```

And then go to the `http://localhost:8080/` URL in your browser.
