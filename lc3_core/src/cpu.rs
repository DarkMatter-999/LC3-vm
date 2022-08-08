use crate::memory::{Mem, self};
use crate::instructions::*;


const PC_START: usize = 0x3000;

pub struct CPU {
    rr0 : u16,
    rr1 : u16,
    rr2 : u16,
    rr3 : u16,
    rr4 : u16,
    rr5 : u16,
    rr6 : u16,
    rr7 : u16,
    pc : usize,
    rcond : u16,
    rcount : u16,
    memory: Mem
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            rr0 : 0,
            rr1 : 0,
            rr2 : 0,
            rr3 : 0,
            rr4 : 0,
            rr5 : 0,
            rr6 : 0,
            rr7 : 0,
            pc : PC_START,
            rcond : 0,
            rcount : 0,
            memory : Mem::new()
        }
    }

    pub fn run(&mut self) {
        self.rcond = Flags::value(&Flags::FlZro);

        loop {
            let inst = self.fetch();

            let op = inst >> 12;

            println!("{:x?}", op);
        }
    }

    fn fetch(&mut self) -> u16 {
        let inst = self.memory.read(self.pc);
        self.pc += 1;
        return inst;
    }

    pub fn load_image(&mut self, image: &Vec<u8>) {
        let origin: u16 = ((image[0] as u16) << 8) | (image[1] as u16);

        println!("Program Address : {:#01x}", origin);

        self.pc = origin as usize;

        let mut i = 2;
        let mut count = 0;
        while(i<image.len()) {
            // lil endian
            //self.memory.memory[origin as usize + count] = (image[i] as u16) | ((image[i+1] as u16) << 8);

            // big endian
            self.memory.memory[origin as usize + count] = ((image[i] as u16) << 8) | (image[i+1] as u16);
            i += 2;
            count += 1;
        }

        // println!("{:x?}", &self.memory.memory);
    }
}