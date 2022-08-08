
const memory_max: usize = 1 << 16;

pub struct Mem {
    pub memory: [u16; memory_max]
}

impl Mem {
    pub fn new() -> Mem {
        Mem { 
            memory: [0; memory_max]
        }
    }

    pub fn read(&self, addr: usize) -> u16 {
        self.memory[addr]
    }

    pub fn write(&mut self, addr: usize, val: u16) {
        self.memory[addr] = val;
    }
}