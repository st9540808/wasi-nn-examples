# wasi-nn examples

wasi-nn examples is a standalone development environment for WASI-NN which uses ONNX Runtime as backend implementation.

## Usage

After cloing this repository, use the command below to build WebAssembly executable.

```bash
$ git submodule update --init
$ cargo build --target wasm32-wasi
```

Then use [WasmEdge](https://github.com/WasmEdge/WasmEdge) to run the compiled WebAssembly executable.

```bash
$ LD_LIBRARY_PATH=<PATH-TO-ONNX>onnxruntime-linux-x64-1.9.0/lib \
  wasmedge --dir .:. ./target/wasm32-wasi/debug/wasi-nn-examples.wasm
```