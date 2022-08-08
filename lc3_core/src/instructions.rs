
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
