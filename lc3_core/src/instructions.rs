
pub enum Flags {
    FlPos,
    FlZro,
    FlNeg
}

impl Flags {
    pub fn value(&self) -> u16 {
        match *self {
            Flags::FlPos => 1 << 0, /* P */
            Flags::FlZro => 1 << 1, /* Z */
            Flags::FlNeg => 1 << 2, /* N */
        }
    }
}

pub enum OPCodes {
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

impl OPCodes {
    pub fn value(self) -> u16 {
        match self {
            OPCodes::OpBr => 0x0,
            OPCodes::OpAdd => 0x1,
            OPCodes::OpLd => 0x2,
            OPCodes::OpSt => 0x3,
            OPCodes::OpJsr => 0x4,
            OPCodes::OpAnd => 0x5,
            OPCodes::OpLdr => 0x6,
            OPCodes::OpStr => 0x7,
            OPCodes::OpRti => 0x8,
            OPCodes::OpNot => 0x9,
            OPCodes::OpLdi => 0xa,
            OPCodes::OpSti => 0xb,
            OPCodes::OpJmp => 0xc,
            OPCodes::OpRes => 0xd,
            OPCodes::OpLea => 0xe,
            OPCodes::OpTrap => 0xf,
        }
    }

    pub fn from(op: u16) -> OPCodes {
        match op {
            0x0 => OPCodes::OpBr,
            0x1 => OPCodes::OpAdd,
            0x2 => OPCodes::OpLd,
            0x3 => OPCodes::OpSt,
            0x4 => OPCodes::OpJsr,
            0x5 => OPCodes::OpAnd,
            0x6 => OPCodes::OpLdr,
            0x7 => OPCodes::OpStr,
            0x8 => OPCodes::OpRti,
            0x9 => OPCodes::OpNot,
            0xa => OPCodes::OpLdi,
            0xb => OPCodes::OpSti,
            0xc => OPCodes::OpJmp,
            0xd => OPCodes::OpRes,
            0xe => OPCodes::OpLea,
            0xf => OPCodes::OpTrap,
            _ => OPCodes::OpRes,
        }
    }
}