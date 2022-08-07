
const memory_max: usize = 1 << 16;

pub struct Mem {
    memory: [i32; memory_max]
}

impl Mem {
    pub fn new() -> Mem {
        Mem { 
            memory: [0; memory_max]
        }
    }
}