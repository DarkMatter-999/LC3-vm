use std::io::{self, Read};
use std::io::Write;
use crate::io::{get_key, print};
use crate::memory::{Mem, self};
use crate::instructions::*;

use super::*;

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

            // println!("{:?}", OPCodes::from(op));
            // println!("PC 0x{:x?}", self.pc - 0x3000);

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

    fn get_reg(&mut self, r: u16) -> &u16 {
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

    fn set_reg(&mut self, r: u16, val: u16)  {
        match r {
            0 => self.rr0 = val,
            1 => self.rr1 = val,
            2 => self.rr2 = val,
            3 => self.rr3 = val,
            4 => self.rr4 = val,
            5 => self.rr5 = val,
            6 => self.rr6 = val,
            7 => self.rr7 = val,
            // 8 => self.pc = val,
            9 => self.rcond = val,
            10 => self.rcount = val,
            _ => println!("Cannot find register {}", r)
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
            // self.pc += offset as usize;
            let val = (self.pc as u16).wrapping_add(offset);
            self.pc = val as usize;
        }
    }

    fn add(&mut self, inst: u16) {
        /* destination register (DR) */
        let r0 = (inst >> 9) & 0x7;
        /* first operand (SR1) */
        let r1 = (inst >> 6) & 0x7;
        /* whether we are in immediate mode */
        let imm_flag = (inst >> 5) & 0x1;

        if (imm_flag != 0)
        {
            let imm5 = self.sign_extend(inst & 0x1F, 5);
            let r = *self.get_reg(r1);
            self.set_reg(r0, r.wrapping_add(imm5));
        }
        else
        {
            let r2 = inst & 0x7;
            let r = *self.get_reg(r1);
            let l = *self.get_reg(r2);
            self.set_reg(r0, r.wrapping_add(l));
        }

        self.update_flags(r0);
    }

    fn ld(&mut self, inst: u16) {
        let r0 = (inst >> 9) & 0x7;
        let pc_offset = self.sign_extend(inst & 0x1FF, 9);
        let val = self.memory.read((self.pc as u16).wrapping_add(pc_offset) as usize);
        
        self.set_reg(r0, val);

        self.update_flags(r0);
    }

    fn st(&mut self, inst: u16) {
        let r0 = (inst >> 9) & 0x7;
        let pc_offset = self.sign_extend(inst & 0x1FF, 9);

        let val = (self.pc as u16).wrapping_add(pc_offset);
        self.memory.write(val as usize, self.rr0);
    }

    fn jsr(&mut self, inst: u16) {
        let long_flag = (inst >> 11) & 1;
        self.rr7 = self.pc as u16;

        if (long_flag != 0) {
            let long_pc_offset = self.sign_extend(inst & 0x7FF, 11);
            self.pc = (self.pc as u16).wrapping_add(long_pc_offset) as usize;  /* JSR */
        } else {
            let r1 = (inst >> 6) & 0x7;
            self.pc = *self.get_reg(r1) as usize; /* JSRR */
        }
    }

    fn and_(&mut self, inst: u16) {
        let r0 = (inst >> 9) & 0x7;
        let r1 = (inst >> 6) & 0x7;
        let imm_flag = (inst >> 5) & 0x1;

        if (imm_flag != 0)
        {
            let imm5 = self.sign_extend(inst & 0x1F, 5);
            let val = *self.get_reg(r1) & imm5;

            self.set_reg(r0, val);
        }
        else
        {
            let r2 = inst & 0x7;
            let val = *self.get_reg(r1) & *self.get_reg(r2);

            self.set_reg(r0, val);
        }
        
        self.update_flags(r0);
    }

    fn ldr(&mut self, inst: u16) {
        let r0 = (inst >> 9) & 0x7;
        let r1 = (inst >> 6) & 0x7;
        let offset = self.sign_extend(inst & 0x3F, 6);

        let r1 = *self.get_reg(r1);
        let val = self.memory.read((r1 + offset) as usize);

        self.set_reg(r0, val);
        
        self.update_flags(r0);
    }

    fn str(&mut self, inst: u16) {
        let r0 = (inst >> 9) & 0x7;
        let r1 = (inst >> 6) & 0x7;
        let offset = self.sign_extend(inst & 0x3F, 6);

        let r0 = *self.get_reg(r0);
        let r1 = *self.get_reg(r1);
        self.memory.write((r1.wrapping_add(offset)) as usize, r0);
    }

    fn rti(&mut self, inst: u16) {
        panic!("Unused OPCode RTI");
    }

    fn not(&mut self, inst: u16) {
        let r0 = (inst >> 9) & 0x7;
        let r1 = (inst >> 6) & 0x7;

        let val = *self.get_reg(r1);
        self.set_reg(r0, !val);
        self.update_flags(r0);
    }

    fn ldi(&mut self, inst: u16) {
        /* destination register (DR) */
        let r0 = (inst >> 9) & 0x7;

        /* PCoffset 9*/
        let pc_offset = self.sign_extend(inst & 0x1FF, 9);

        /* add pc_offset to the current PC, look at that memory location to get the final address */
        let addr = self.memory.read((self.pc as u16).wrapping_add(pc_offset) as usize);
        
        let val = self.memory.read(addr as usize);
        self.set_reg(r0, val);
        self.update_flags(r0);
    }

    fn sti(&mut self, inst: u16) {
        let r0 = (inst >> 9) & 0x7;
        let pc_offset = self.sign_extend(inst & 0x1FF, 9);
        let addr = self.memory.read((self.pc as u16).wrapping_add(pc_offset) as usize);
        
        let val = *self.get_reg(r0);
        self.memory.write(addr as usize, val);
    }

    fn jmp(&mut self, inst: u16) {
        let r = (inst >> 6) & 0x7;
        let r = *self.get_reg(r);
        self.pc = r as usize;
    }

    fn res(&mut self, inst: u16) {
        panic!("Unused OPCode RES");
    }

    fn lea(&mut self, inst: u16) {
        let r0 = (inst >> 9) & 0x7;
        let pc_offset = self.sign_extend(inst & 0x1FF, 9);
        self.set_reg(r0, (self.pc as u16).wrapping_add(pc_offset));
        
        self.update_flags(r0);
    }

    fn trap(&mut self, inst: u16) {
        match TrapCodes::from(inst & 0xFF) {
            TrapCodes::TrapGetC => self.trap_getc(),
            TrapCodes::TrapOut => self.trap_out(),
            TrapCodes::TrapPuts => self.trap_puts(),
            TrapCodes::TrapIn => self.trap_in_(),
            TrapCodes::TrapPutsP => self.trap_putsp(),
            TrapCodes::TrapHalt => self.trap_halt(),
        }
    }

    fn trap_getc(&mut self) {
        let ch = get_key();

        self.rr0 = ch as u16;

        // self.update_flags(self.rr0);
    }

    fn trap_out(&mut self) {
        let c = self.rr0 as u8;
        print(c);
    }

    fn trap_puts(&mut self) {
        /* one char per word */
        let mut count = 0;
        loop {
            let chr = self.memory.read((self.rr0 + count) as usize);
            if (chr == 0) {
                break;
            }
            print(chr as u8);
            count += 1;
        }

        io::stdout().flush().unwrap(); // expect("Flushed.");
    }

    fn trap_in_(&mut self) {
        print!("Enter a character: ");

        let char = std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as u16)
                .unwrap();

        self.rr0 = char;

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

            let c1 = ((chr & 0xFF) as u8);
            print(c1);
            let c2 = ((chr >> 8) as u8);
            if c2 != '\0' as u8 {
                print(c2);
            }

            count += 1;
        }
    }

    fn trap_halt(&mut self) {
        println!("HALTING");
        self.running = false;
    }
}