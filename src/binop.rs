#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add = 0,
    Sub,
    Mul,
    Div,
    Mod,

    LE,
    LT,
    GE,
    GT,
    Eq,
    Ne,
}
