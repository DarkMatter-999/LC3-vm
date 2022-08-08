use std::{env, process::exit, fs};
use lc3_core::{cpu::CPU};

fn main() {

    let mut argv = env::args();

    match argv.len() {
        2 => {
            runimage(&argv.nth(1).expect("Argv parse failed")); 
        }
        _ => {
            eprintln!("Usage: lc3 [image-file]\n");
            exit(64);
        } 
    }
}

fn runimage(path: &str) {
    let mut cpu = CPU::new();
    let image = fs::read(path);
    
    match image {
        Ok(img) => cpu.load_image(&img),
        Err(_) => {
            eprintln!("Usage: lc3 [image-file]\n");
            exit(64);
        }
    }
}