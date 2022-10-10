pub type OperList = Vec<Oper>;

#[derive(Clone, Debug)]
pub struct Oper {
    pub pos: (usize, usize),
    pub data: OperData,
}

impl Oper {
    pub fn new(pos: (usize, usize), data: OperData) -> Self {
        Self { pos, data }
    }
    pub fn none_op(pos: (usize, usize)) -> Self {
        Self {
            pos,
            data: OperData::None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum OperData {
    Plus,
    Minus,
    Times,
    Divide,
    Mod,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Subscript,
    TypeEq,

    Deref,
    Address,
    UMinus,
    Not,

    None,
}

pub fn string_to_oper(pos: (usize, usize), name: &str) -> Oper {
    match name {
        "AddOp" => Oper::new(pos, OperData::Plus),
        "SubOp" => Oper::new(pos, OperData::Minus),
        "EqOp" => Oper::new(pos, OperData::Eq),
        "NeOp" => Oper::new(pos, OperData::Neq),
        "LtOp" => Oper::new(pos, OperData::Lt),
        "LeOp" => Oper::new(pos, OperData::Le),
        "GtOp" => Oper::new(pos, OperData::Gt),
        "GeOp" => Oper::new(pos, OperData::Ge),
        "AndOp" => Oper::new(pos, OperData::And),
        "OrOp" => Oper::new(pos, OperData::Or),
        "MulOp" => Oper::new(pos, OperData::Times),
        "DivOp" => Oper::new(pos, OperData::Divide),
        "DerefOp" => Oper::new(pos, OperData::Deref),
        "AddressOp" => Oper::new(pos, OperData::Address),
        "NotOp" => Oper::new(pos, OperData::Not),
        _ => Oper::new(pos, OperData::None),
    }
}
