use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn add(a:i32,b:i32) {
    let c = a + b;
    println!("{} + {} = {}", a, b, c);
}