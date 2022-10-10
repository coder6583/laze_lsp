use super::{
    dec,
    exp::{self},
    ifelse::{self, IfElseList},
    var::Var,
};

pub type StmList = Vec<Stm>;

pub type Stm = Box<Stm_>;

#[derive(Clone, Debug)]
pub struct Stm_ {
    pub pos: (usize, usize),
    pub keywords: Vec<(usize, usize)>,
    pub data: StmData,
}

#[derive(Clone, Debug)]
pub enum AssignType {
    Normal,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Debug)]
pub enum StmData {
    Compound(StmList),
    Assign(Var, exp::ASTExp, AssignType),
    Dec(dec::Dec),
    Exp(exp::ASTExp),
    IfElse(ifelse::IfElseList),
    While(exp::ASTExp, Stm),
    For(Stm, exp::ASTExp, Stm, Stm),
    Call(exp::ASTExp, exp::ASTExpList),
    Return(exp::ASTExp),
    Loop(Stm),
    Repeat(exp::ASTExp, Stm),
    Continue,
    Break,

    None,
}

impl Stm_ {
    pub fn none_stm(pos: (usize, usize)) -> Stm {
        Box::new(Stm_ {
            pos,
            keywords: vec![],
            data: StmData::None,
        })
    }
    pub fn compound_stm(
        pos: (usize, usize),
        stmlist: StmList,
        keywords: Vec<(usize, usize)>,
    ) -> Stm {
        Box::new(Stm_ {
            pos,
            keywords,
            data: StmData::Compound(stmlist),
        })
    }
    pub fn assign_stm(
        pos: (usize, usize),
        var: Var,
        init: exp::ASTExp,
        assign_type: AssignType,
        keywords: Vec<(usize, usize)>,
    ) -> Stm {
        Box::new(Stm_ {
            pos,
            keywords,
            data: StmData::Assign(var, init, assign_type),
        })
    }
    pub fn dec_stm(pos: (usize, usize), dec: dec::Dec, keywords: Vec<(usize, usize)>) -> Stm {
        Box::new(Stm_ {
            pos,
            keywords,
            data: StmData::Dec(dec),
        })
    }
    pub fn exp_stm(pos: (usize, usize), exp: exp::ASTExp, keywords: Vec<(usize, usize)>) -> Stm {
        Box::new(Stm_ {
            pos,
            keywords,
            data: StmData::Exp(exp),
        })
    }
    pub fn ifelse_stm(
        pos: (usize, usize),
        ifelselist: IfElseList,
        keywords: Vec<(usize, usize)>,
    ) -> Stm {
        Box::new(Stm_ {
            pos,
            keywords,
            data: StmData::IfElse(ifelselist),
        })
    }
    pub fn while_stm(
        pos: (usize, usize),
        test: exp::ASTExp,
        body: Stm,
        keywords: Vec<(usize, usize)>,
    ) -> Stm {
        Box::new(Stm_ {
            pos,
            keywords,
            data: StmData::While(test, body),
        })
    }
    pub fn for_stm(
        pos: (usize, usize),
        init: Stm,
        test: exp::ASTExp,
        incr: Stm,
        body: Stm,
        keywords: Vec<(usize, usize)>,
    ) -> Stm {
        Box::new(Stm_ {
            pos,
            keywords,
            data: StmData::For(init, test, incr, body),
        })
    }
    pub fn call_stm(
        pos: (usize, usize),
        func: exp::ASTExp,
        args: exp::ASTExpList,
        keywords: Vec<(usize, usize)>,
    ) -> Stm {
        Box::new(Stm_ {
            pos,
            keywords,
            data: StmData::Call(func, args),
        })
    }
    pub fn return_stm(pos: (usize, usize), val: exp::ASTExp, keywords: Vec<(usize, usize)>) -> Stm {
        Box::new(Stm_ {
            pos,
            keywords,
            data: StmData::Return(val),
        })
    }
    pub fn continue_stm(pos: (usize, usize), keywords: Vec<(usize, usize)>) -> Stm {
        Box::new(Stm_ {
            pos,
            keywords,
            data: StmData::Continue,
        })
    }
    pub fn break_stm(pos: (usize, usize), keywords: Vec<(usize, usize)>) -> Stm {
        Box::new(Stm_ {
            pos,
            keywords,
            data: StmData::Break,
        })
    }
    pub fn loop_stm(pos: (usize, usize), body: Stm, keywords: Vec<(usize, usize)>) -> Stm {
        Box::new(Stm_ {
            pos,
            keywords,
            data: StmData::Loop(body),
        })
    }
    pub fn repeat_stm(
        pos: (usize, usize),
        count: exp::ASTExp,
        body: Stm,
        keywords: Vec<(usize, usize)>,
    ) -> Stm {
        Box::new(Stm_ {
            pos,
            keywords,
            data: StmData::Repeat(count, body),
        })
    }
}
