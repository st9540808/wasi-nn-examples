use wasi_nn;

const MOBILENETV2_PATH: &str = "testdata/models/mobilenetv2-7.onnx";

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

#[no_mangle]
fn load_model() {
    let model = std::fs::read(MOBILENETV2_PATH).unwrap();
    println!("load_model: loaded {} bytes", model.len());
    let _ = unsafe {
        wasi_nn::load(
            &[&model],
            wasi_nn::GRAPH_ENCODING_ONNX,
            wasi_nn::EXECUTION_TARGET_CPU,
        )
        .unwrap()
    };
}

fn main() {
    // println!("curdir: {:#?}", std::env::current_dir().unwrap());
    // println!("PATH: {:#?}", std::env::var("PATH").unwrap());
    load_empty();
    load_model();
}
