use std::io::{self, Read};
use std::io::Write;
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
    memory: Mem,
    running: bool
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
            memory : Mem::new(),
            running: true
        }
    }

    pub fn run(&mut self) {
        self.rcond = Flags::value(&Flags::FlZro);

        while(self.running) {
            let inst = self.fetch();

            let op = inst >> 12;

            // println!("{:x?}", op);

            match(OPCodes::from(op)) {
                OPCodes::OpBr => self.br(inst),
                OPCodes::OpAdd => self.add(inst),
                OPCodes::OpLd => self.ld(inst),
                OPCodes::OpSt => self.st(inst),
                OPCodes::OpJsr => self.jsr(inst),
                OPCodes::OpAnd => self.and_(inst),
                OPCodes::OpLdr => self.ldr(inst),
                OPCodes::OpStr => self.str(inst),
                OPCodes::OpRti => self.rti(inst),
                OPCodes::OpNot => self.not(inst),
                OPCodes::OpLdi => self.ldi(inst),
                OPCodes::OpSti => self.sti(inst),
                OPCodes::OpJmp => self.jmp(inst),
                OPCodes::OpRes => self.res(inst),
                OPCodes::OpLea => self.lea(inst),
                OPCodes::OpTrap => self.trap(inst),
            }
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

    fn sign_extend(&mut self, x: u16, bit_count: u16) -> u16 {
        let mut x = x;
        if ((x >> (bit_count - 1)) & 1) != 0 {
            x |= (0xFFFF << bit_count);
        }

        return x;
    }

    fn get_reg(&self, r: u16) -> &u16 {
        match r {
            0 => &self.rr0,
            1 => &self.rr1,
            2 => &self.rr2,
            3 => &self.rr3,
            4 => &self.rr4,
            5 => &self.rr5,
            6 => &self.rr6,
            7 => &self.rr7,
            // 8 => &self.pc,
            9 => &self.rcond,
            10 => &self.rcount,
            _ => &0
        }
    }

    fn update_flags(&mut self, r: u16) {
        if (*self.get_reg(r) == 0) {
            self.rcond = Flags::value(&Flags::FlZro);
        } else if ((*self.get_reg(r) >> 15) != 0) {
            /* a 1 in the left-most bit indicates negative */
            self.rcond = Flags::value(&Flags::FlNeg);
        } else {
            self.rcond = Flags::value(&Flags::FlPos);
        }
    }

    fn br(&mut self, inst: u16) {
        let offset = self.sign_extend(inst & 0x1ff, 9);
        let cond_flag = (inst >> 9) & 0x7;

        if ((cond_flag & self.rcond) != 0) {
            self.pc += offset as usize;
        }
    }

    fn add(&mut self, inst: u16) {
        /* destination register (DR) */
        let r0 = (inst >> 9) & 0x7;

        /* first operand (SR1) */
        let r1 = (inst >> 6) & 0x7;

        /* whether we are in immediate mode */
        let imm_flag = (inst >> 5) & 0x1;

        if (imm_flag != 0) {
            let imm5 = self.sign_extend(inst & 0x1F, 5);
            self.rr0 = self.rr1 + imm5;
        } else {
            let r2 = inst & 0x7;
            self.rr0 = self.rr1 + self.rr2;
        }

        self.update_flags(r0);
    }

    fn ld(&mut self, inst: u16) {
        let r0 = (inst >> 9) & 0x7;
        let pc_offset = self.sign_extend(inst & 0x1FF, 9) as usize;
        self.rr0 = self.memory.read(self.pc + pc_offset);
        
        self.update_flags(r0);
    }

    fn st(&mut self, inst: u16) {
        let r0 = (inst >> 9) & 0x7;
        let pc_offset = self.sign_extend(inst & 0x1FF, 9) as usize;
        self.memory.write(self.pc + pc_offset, self.rr0);
    }

    fn jsr(&mut self, inst: u16) {
        let long_flag = (inst >> 11) & 1;
        self.rr7 = self.pc as u16;

        if (long_flag != 0) {
            let long_pc_offset = self.sign_extend(inst & 0x7FF, 11);
            self.pc += long_pc_offset as usize;  /* JSR */
        } else {
            let r1 = (inst >> 6) & 0x7;
            self.pc = self.rr1 as usize; /* JSRR */
        }
    }

    fn and_(&mut self, inst: u16) {
        let r0 = (inst >> 9) & 0x7;
        let r1 = (inst >> 6) & 0x7;
        let imm_flag = (inst >> 5) & 0x1;

        if (imm_flag != 0)
        {
            let imm5 = self.sign_extend(inst & 0x1F, 5);
            self.rr0 = self.rr1 & imm5;
        }
        else
        {
            let r2 = inst & 0x7;
            self.rr0 = self.rr1 & self.rr2;
        }
        
        self.update_flags(r0);
    }

    fn ldr(&mut self, inst: u16) {
        let r0 = (inst >> 9) & 0x7;
        let r1 = (inst >> 6) & 0x7;
        let offset = self.sign_extend(inst & 0x3F, 6);
        self.rr0 = self.memory.read((self.rr1 + offset) as usize);
        
        self.update_flags(r0);
    }

    fn str(&mut self, inst: u16) {
        let r0 = (inst >> 9) & 0x7;
        let r1 = (inst >> 6) & 0x7;
        let offset = self.sign_extend(inst & 0x3F, 6);
        self.memory.write((self.rr1 + offset) as usize, self.rr0);
    }

    fn rti(&mut self, inst: u16) {
        panic!("Unused OPCode RTI");
    }

    fn not(&mut self, inst: u16) {
        let r0 = (inst >> 9) & 0x7;
        let r1 = (inst >> 6) & 0x7;

        self.rr0 = !self.rr1;
        self.update_flags(r0);
    }

    fn ldi(&mut self, inst: u16) {
        /* destination register (DR) */
        let r0 = (inst >> 9) & 0x7;

        /* PCoffset 9*/
        let pc_offset = self.sign_extend(inst & 0x1FF, 9) as usize;

        /* add pc_offset to the current PC, look at that memory location to get the final address */
        self.rr0 = self.memory.read(self.memory.read(self.pc + pc_offset) as usize);
        self.update_flags(r0);
    }

    fn sti(&mut self, inst: u16) {
        let r0 = (inst >> 9) & 0x7;
        let pc_offset = self.sign_extend(inst & 0x1FF, 9) as usize;
        self.memory.write(self.memory.read(self.pc + pc_offset) as usize, self.rr0);
    }

    fn jmp(&mut self, inst: u16) {
        let r1 = (inst >> 6) & 0x7;
        self.pc = self.rr1 as usize;
    }

    fn res(&mut self, inst: u16) {
        panic!("Unused OPCode RES");
    }

    fn lea(&mut self, inst: u16) {
        let r0 = (inst >> 9) & 0x7;
        let pc_offset = self.sign_extend(inst & 0x1FF, 9);
        self.rr0 = self.pc as u16 + pc_offset;
        
        self.update_flags(r0);
    }

    fn trap(&mut self, inst: u16) {
        match TrapCodes::from(inst) {
            TrapCodes::TrapGetC => self.trap_getc(),
            TrapCodes::TrapOut => self.out(),
            TrapCodes::TrapPuts => self.trap_puts(),
            TrapCodes::TrapIn => self.in_(),
            TrapCodes::TrapPutsP => self.trap_putsp(),
            TrapCodes::TrapHalt => self.halt(),
        }
    }

    fn trap_getc(&mut self) {
        io::stdout().flush().expect("Flushed.");
        let mut buffer = [0; 1];
        std::io::stdin().read_exact(&mut buffer).unwrap();
        
        self.update_flags(self.rr0);
    }

    fn out(&mut self) {
        print!("{}", std::char::from_u32(self.rr0 as u32).unwrap_or(' '));
        io::stdout().flush().expect("Flushed.");
    }

    fn trap_puts(&mut self) {
        /* one char per word */
        let mut count = 0;
        loop {
            let chr = self.memory.read(self.rr0 as usize + count);
            if (chr == 0) {
                break;
            }

            let chr = std::char::from_u32(chr as u32);
            print!("{}", chr.unwrap_or(' '));
            count += 1;
        }

        io::stdout().flush().expect("Flushed.");
    }

    fn in_(&mut self) {
        print!("Enter a character: ");

        let mut buffer = [0; 1];
        std::io::stdin().read_exact(&mut buffer).unwrap();
        let chr = std::char::from_u32(buffer[0] as u32);

        print!("{}", std::char::from_u32(self.rr0 as u32).unwrap_or(' '));
        io::stdout().flush().expect("Flushed.");

        self.rr0 = buffer[0] as u16;

        self.update_flags(self.rr0);
    }

    fn trap_putsp(&mut self) {
        /* one char per byte (two bytes per word)
        here we need to swap back to
        big endian format */
        let mut count = 0;
        loop {
            let chr = self.memory.read(self.rr0 as usize + count);
            if (chr == 0) {
                break;
            }

            let chr1 = std::char::from_u32((chr & 0xFF) as u32);
            print!("{}", chr1.unwrap_or(' '));
            let chr2 = std::char::from_u32((chr >> 8) as u32);
            print!("{}", chr2.unwrap_or(' '));

            count += 1;
        }

        io::stdout().flush().expect("Flushed.");
    }

    fn halt(&mut self) {
        println!("HALTING");
        io::stdout().flush().expect("Flushed.");
        self.running = false;
    }
}