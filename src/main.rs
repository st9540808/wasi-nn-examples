use wasi_nn;

#[no_mangle]
fn load_empty() {
    let _ = unsafe {
        wasi_nn::load(
            &[&[]],
            wasi_nn::GRAPH_ENCODING_ONNX,
            wasi_nn::EXECUTION_TARGET_CPU,
        )
        .unwrap()
    };
}

fn main() {
    println!("Hello, world!");
    load_empty();
}
