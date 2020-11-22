# Meili crates browser 2000

This example is a clone of [crates.meilisearch.com](https://crates.meilisearch.com), but I rewrote the frontend in Rust.\
It's using the same MeiliSearch server.

## Checking

If you only want to check if this example compiles, you can run
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
wasm-pack build examples/web_app/ --target=web  --no-typescript
```
Compiled files will be stored in the `pkg` folder.

## Using

Theoretically, you could just open the `pkg/index.html` file but due to browsers' security restrictions, you need a web server.

```console
python3 -m http.server 8080
```
