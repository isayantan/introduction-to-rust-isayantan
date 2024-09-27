pub mod sampler;

use wasm_bindgen::prelude::*;

[wasm_bindgen]
pub fn run(){
    const N_STEP: usize = 100:
    const N_DIM: usize = 2;
    let mut state = sample::State::<N_DIM>::new(0xdeadbeef);
    for _ in 0..N_STEP {
        state.take_step();
        log(&format!("{:?}, {:?}", state.arr[0], state.arr[[1]))
}