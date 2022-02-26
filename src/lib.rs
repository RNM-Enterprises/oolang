use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn big_computation() {
    alert("Big computation in Rust");
}

#[wasm_bindgen]
pub fn welcome(name: &str) {
	alert(&format!("Hello {}, from Rust!", name));
}

#[wasm_bindgen]
pub fn fib(n: u32) -> u32 {
	if n == 0 || n == 1 {
		return n;
	}
	fib(n - 1) + fib(n - 2)
}