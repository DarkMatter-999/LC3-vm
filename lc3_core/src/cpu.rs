use crate::memory::Mem;


const PC_START: i32 = 0x3000;

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
    rr0 : i32,
    rr1 : i32,
    rr2 : i32,
    rr3 : i32,
    rr4 : i32,
    rr5 : i32,
    rr6 : i32,
    rr7 : i32,
    pc : i32,
    rcond : i32,
    rcount : i32,
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
}