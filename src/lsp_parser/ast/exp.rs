use super::{
    ast::{ASTBool, ASTString},
    field, op, stm,
    suffix::ASTExpSuffixList,
    var::Var,
};

pub type ASTExp = Box<ASTExp_>;
pub type ASTExpList = Vec<ASTExp>;

#[derive(Clone, Debug)]
pub struct ASTExp_ {
    pub pos: (usize, usize),
    pub keywords: Vec<(usize, usize)>,
    pub data: ASTExpData,
}

#[derive(Clone, Debug)]
pub enum ASTExpData {
    Int(ASTString),
    Short(ASTString),
    Real(ASTString),
    Char(char),
    String(ASTString),
    Bool(ASTBool),

    Var(Var),
    Call(ASTExp, ASTExpList),
    BinOp(op::OperList, ASTExpList),
    UnaryOp(op::OperList, ASTExp),
    Func(field::FieldList, field::FieldList, stm::Stm),
    Field(ASTExp, String),
    Array(ASTExpList),
    SizeOf(ASTExp),
    Paren(ASTExp),
    Suffix(ASTExp, ASTExpSuffixList),

    None,
}

impl ASTExp_ {
    pub fn none_exp(pos: (usize, usize)) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            keywords: vec![],
            data: ASTExpData::None,
        })
    }
    pub fn int_exp(pos: (usize, usize), data: ASTString, keywords: Vec<(usize, usize)>) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            keywords,
            data: ASTExpData::Int(data),
        })
    }
    pub fn real_exp(pos: (usize, usize), data: ASTString, keywords: Vec<(usize, usize)>) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            keywords,
            data: ASTExpData::Real(data),
        })
    }
    pub fn char_exp(pos: (usize, usize), data: char, keywords: Vec<(usize, usize)>) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            keywords,
            data: ASTExpData::Char(data),
        })
    }
    pub fn string_exp(
        pos: (usize, usize),
        data: ASTString,
        keywords: Vec<(usize, usize)>,
    ) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            keywords,
            data: ASTExpData::String(data),
        })
    }
    pub fn bool_exp(pos: (usize, usize), data: bool, keywords: Vec<(usize, usize)>) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            keywords,
            data: ASTExpData::Bool(ASTBool::new(pos, data)),
        })
    }

    pub fn var_exp(pos: (usize, usize), data: Var, keywords: Vec<(usize, usize)>) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            keywords,
            data: ASTExpData::Var(data),
        })
    }
    pub fn call_exp(
        pos: (usize, usize),
        func: ASTExp,
        args: ASTExpList,
        keywords: Vec<(usize, usize)>,
    ) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            keywords,
            data: ASTExpData::Call(func, args),
        })
    }
    pub fn binop_exp(
        pos: (usize, usize),
        oplist: op::OperList,
        explist: ASTExpList,
        keywords: Vec<(usize, usize)>,
    ) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            keywords,
            data: ASTExpData::BinOp(oplist, explist),
        })
    }
    pub fn unaryop_exp(
        pos: (usize, usize),
        oplist: op::OperList,
        exp: ASTExp,
        keywords: Vec<(usize, usize)>,
    ) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            keywords,
            data: ASTExpData::UnaryOp(oplist, exp),
        })
    }
    pub fn func_exp(
        pos: (usize, usize),
        params: field::FieldList,
        result: field::FieldList,
        stm: stm::Stm,
        keywords: Vec<(usize, usize)>,
    ) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            keywords,
            data: ASTExpData::Func(params, result, stm),
        })
    }
    pub fn field_exp(
        pos: (usize, usize),
        field: ASTExp,
        member: String,
        keywords: Vec<(usize, usize)>,
    ) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            keywords,
            data: ASTExpData::Field(field, member),
        })
    }
    pub fn array_exp(
        pos: (usize, usize),
        explist: ASTExpList,
        keywords: Vec<(usize, usize)>,
    ) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            keywords,
            data: ASTExpData::Array(explist),
        })
    }
    pub fn sizeof_exp(pos: (usize, usize), var: ASTExp, keywords: Vec<(usize, usize)>) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            keywords,
            data: ASTExpData::SizeOf(var),
        })
    }
    pub fn paren_exp(pos: (usize, usize), exp: ASTExp, keywords: Vec<(usize, usize)>) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            keywords,
            data: ASTExpData::Paren(exp),
        })
    }
    pub fn suffix_exp(
        pos: (usize, usize),
        exp: ASTExp,
        suffix: ASTExpSuffixList,
        keywords: Vec<(usize, usize)>,
    ) -> ASTExp {
        Box::new(ASTExp_ {
            pos,
            keywords,
            data: ASTExpData::Suffix(exp, suffix),
        })
    }
}
