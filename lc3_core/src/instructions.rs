
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
            Self::OpBr => 0x0,
            Self::OpAdd => 0x1,
            Self::OpLd => 0x2,
            Self::OpSt => 0x3,
            Self::OpJsr => 0x4,
            Self::OpAnd => 0x5,
            Self::OpLdr => 0x6,
            Self::OpStr => 0x7,
            Self::OpRti => 0x8,
            Self::OpNot => 0x9,
            Self::OpLdi => 0xa,
            Self::OpSti => 0xb,
            Self::OpJmp => 0xc,
            Self::OpRes => 0xd,
            Self::OpLea => 0xe,
            Self::OpTrap => 0xf,
        }
    }

    pub fn from(op: u16) -> OPCodes {
        match op {
            0x0 => Self::OpBr,
            0x1 => Self::OpAdd,
            0x2 => Self::OpLd,
            0x3 => Self::OpSt,
            0x4 => Self::OpJsr,
            0x5 => Self::OpAnd,
            0x6 => Self::OpLdr,
            0x7 => Self::OpStr,
            0x8 => Self::OpRti,
            0x9 => Self::OpNot,
            0xa => Self::OpLdi,
            0xb => Self::OpSti,
            0xc => Self::OpJmp,
            0xd => Self::OpRes,
            0xe => Self::OpLea,
            0xf => Self::OpTrap,
            _ => Self::OpRes,
        }
    }
}

pub enum TrapCodes{
    TrapGetC,
    TrapOut,
    TrapPuts,
    TrapIn,
    TrapPutsP,
    TrapHalt,
}

impl TrapCodes {
    pub fn value(&self) -> u16 {
        match *self {
            Self::TrapGetC => 0x20,  /* get character from keyboard, not echoed onto the terminal */
            Self::TrapOut => 0x21,   /* output a character */
            Self::TrapPuts => 0x22,  /* output a word string */
            Self::TrapIn => 0x23,    /* get character from keyboard, echoed onto the terminal */
            Self::TrapPutsP => 0x24, /* output a byte string */
            Self::TrapHalt => 0x25   /* halt the program */
        }
    }
}