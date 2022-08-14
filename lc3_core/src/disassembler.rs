use crate::instructions::{OPCodes, TrapCodes};

fn get_reg(r: u16) -> &'static str {
    match r {
        0 => "R0",
        1 => "R1",
        2 => "R2",
        3 => "R3",
        4 => "R4",
        5 => "R5",
        6 => "R6",
        7 => "R7",
        8 => "PC",
        9 => "RCond",
        10 => "RCount",
        _ => "0"
    }
}

fn sign_extend(x: u16, bit_count: u16) -> u16 {
    let mut x = x;
    if ((x >> (bit_count - 1)) & 1) != 0 {
        x |= (0xFFFF << bit_count);
    }

    return x;
}

pub fn disassemble(inst: u16) {
    let op = inst >> 12;

    match OPCodes::from(op) {
        OPCodes::OpTrap => {
            match TrapCodes::from(inst & 0xFF) {
                TrapCodes::TrapGetC => println!("{:?}", TrapCodes::TrapGetC),
                TrapCodes::TrapOut => println!("{:?}", TrapCodes::TrapOut),
                TrapCodes::TrapPuts => println!("{:?}", TrapCodes::TrapPuts),
                TrapCodes::TrapIn => println!("{:?}", TrapCodes::TrapIn),
                TrapCodes::TrapPutsP => println!("{:?}", TrapCodes::TrapPutsP),
                TrapCodes::TrapHalt => println!("{:?}", TrapCodes::TrapHalt),
            }
        },
        OPCodes::OpBr => {
            let offset = sign_extend(inst & 0x1ff, 9);
            
            println!("{:?}  : 0x{:x}", OPCodes::from(op), offset);
        },
        OPCodes::OpAdd | OPCodes::OpAnd => {
            /* destination register (DR) */
            let r0 = get_reg((inst >> 9) & 0x7);
            /* first operand (SR1) */
            let r1 = get_reg((inst >> 6) & 0x7);
            /* whether we are in immediate mode */
            let imm_flag = (((inst >> 5) & 0x1) != 0) as u8;

            println!("{:?} : {}  {} #{}", OPCodes::from(op), r0, r1, imm_flag);
        },
        OPCodes::OpLd | OPCodes::OpSt => {
            let r0 = get_reg((inst >> 9) & 0x7);
            let pc_offset = sign_extend(inst & 0x1FF, 9);

            println!("{:?}  : {} 0x{:x}", OPCodes::from(op), r0, pc_offset);
        },
        OPCodes::OpJsr => {
            let long_flag = (inst >> 11) & 1;

            if (long_flag != 0) {
                let long_pc_offset = sign_extend(inst & 0x7FF, 11);
                println!("{:?} : 0x{:x}", OPCodes::from(op), long_pc_offset);
            } else {
                let r1 = get_reg((inst >> 6) & 0x7);
                println!("{:?} : {}", OPCodes::from(op), r1);
            }
        },
        OPCodes::OpLdr | OPCodes::OpStr => {
            let r0 = get_reg((inst >> 9) & 0x7);
            let r1 = get_reg((inst >> 6) & 0x7);
            let offset = sign_extend(inst & 0x3F, 6);
            
            println!("{:?} : {}  {} 0x{:x}", OPCodes::from(op), r0, r1, offset);
        },
        OPCodes::OpNot => {
            let r0 = get_reg((inst >> 9) & 0x7);
            let r1 = get_reg((inst >> 6) & 0x7);

            println!("{:?} : {}  {}", OPCodes::from(op), r0, r1);
        },
        OPCodes::OpLdi | OPCodes::OpSti => {
            /* destination register (DR) */
            let r0 = get_reg((inst >> 9) & 0x7);

            /* PCoffset 9*/
            let pc_offset = sign_extend(inst & 0x1FF, 9);
            
            println!("{:?} : {} 0x{:x}", OPCodes::from(op), r0, pc_offset);
        },
        OPCodes::OpJmp => {
            let r = get_reg((inst >> 6) & 0x7);

            println!("{:?} : {}", OPCodes::from(op), r);
        },
        OPCodes::OpLea => {
            let r0 = get_reg((inst >> 9) & 0x7);
            let pc_offset = sign_extend(inst & 0x1FF, 9);

            println!("{:?} : {} 0x{:x}", OPCodes::from(op), r0, pc_offset);
        },
        _ => println!("{:?}", OPCodes::from(op))
    }
}