
pub mod memory;
pub mod cpu;
pub mod instructions;

use std::fs;

use cpu::CPU;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test(s: String) -> String {
    s + "asdsad"
}

#[wasm_bindgen]
pub fn runimage(path: Vec<u8>) {
    let mut cpu = CPU::new();

    cpu.load_image(&path);

    cpu.run();
}