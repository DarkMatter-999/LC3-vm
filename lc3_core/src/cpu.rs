use crate::memory::{Mem, self};


const PC_START: i16 = 0x3000;

enum Flags {
    FlPos = 1 << 0, /* P */
    FlZro = 1 << 1, /* Z */
    FlNeg = 1 << 2, /* N */
}

enum OPCodes {
    OpBr,     /* branch */
    OpAdd,    /* add  */
    OpLd,     /* load */
    OpSt,     /* store */
    OpJsr,    /* jump register */
    OpAnd,    /* bitwise and */
    OpLdr,    /* load register */
    OpStr,    /* store register */
    OpRti,    /* unused */
    OpNot,    /* bitwise not */
    OpLdi,    /* load indirect */
    OpSti,    /* store indirect */
    OpJmp,    /* jump */
    OpRes,    /* reserved (unused) */
    OpLea,    /* load effective address */
    OpTrap    /* execute trap */
}

pub struct CPU {
    rr0 : i16,
    rr1 : i16,
    rr2 : i16,
    rr3 : i16,
    rr4 : i16,
    rr5 : i16,
    rr6 : i16,
    rr7 : i16,
    pc : i16,
    rcond : i16,
    rcount : i16,
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

    }

    fn fetch() {}

    pub fn load_image(&mut self, image: &Vec<u8>) {
        let origin: u16 = ((image[0] as u16) << 8) | (image[1] as u16);

        println!("{:#01x}", origin);

        for i in 2..image.len() {
            self.memory.memory[(origin as usize +i)] = image[i] as u16;
        }

        let mut count = image.len();
        let mut temp: u16;
        while(count > 0) {
            temp = self.memory.memory[origin as usize + count];
            self.memory.memory[origin as usize + count] = self.memory.memory[origin as usize + count + 1];
            self.memory.memory[origin as usize + count + 1] = temp;

            count -= 2;
        }

        println!("{:x?}", &self.memory.memory);
    }
}