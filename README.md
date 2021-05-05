# grpc-web-client

A Rust implementation of the [gRPC-Web protocol](https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-WEB.md) that allows using [tonic](https://github.com/hyperium/tonic) in browsers via WASM.

## Testing

Running the tests requires [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) `wasm-pack`.

To run the tests, first start the server:

```bash
RUST_LOG=info cargo run -p test-server
```

Then, after the server is built and running, the tests can be run.

To test in Firefox:

```bash
wasm-pack test --firefox --headless test/test-client
```

To test in Chrome:

```bash
wasm-pack test --chrome --headless test/test-client
```

To test in Safari:

```bash
wasm-pack test --safari --headless test/test-client
```

## Acknowledgments

This package is heavily based on [webtonic](https://github.com/Sawchord/webtonic).