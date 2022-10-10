use super::{
    ast::ID,
    exp::{ASTExp, ASTExpList},
};

pub type ASTExpSuffixList = Vec<ASTExpSuffix>;
pub type ASTExpSuffix = Box<ASTExpSuffix_>;

#[derive(Clone, Debug)]
pub struct ASTExpSuffix_ {
    pub pos: (usize, usize),
    pub keywords: Vec<(usize, usize)>,
    pub data: SuffixData,
}

#[derive(Clone, Debug)]
pub enum SuffixData {
    Call(ASTExpList),
    Dot(ID),
    Arrow(ID),
    Subscript(ASTExp),
}

impl ASTExpSuffix_ {
    pub fn call_suffix(
        pos: (usize, usize),
        explist: ASTExpList,
        keywords: Vec<(usize, usize)>,
    ) -> ASTExpSuffix {
        Box::new(ASTExpSuffix_ {
            pos,
            keywords,
            data: SuffixData::Call(explist),
        })
    }
    pub fn dot_suffix(
        pos: (usize, usize),
        field: ID,
        keywords: Vec<(usize, usize)>,
    ) -> ASTExpSuffix {
        Box::new(ASTExpSuffix_ {
            pos,
            keywords,
            data: SuffixData::Dot(field),
        })
    }
    pub fn arrow_suffix(
        pos: (usize, usize),
        field: ID,
        keywords: Vec<(usize, usize)>,
    ) -> ASTExpSuffix {
        Box::new(ASTExpSuffix_ {
            pos,
            keywords,
            data: SuffixData::Arrow(field),
        })
    }
    pub fn subscript_suffix(
        pos: (usize, usize),
        index: ASTExp,
        keywords: Vec<(usize, usize)>,
    ) -> ASTExpSuffix {
        Box::new(ASTExpSuffix_ {
            pos,
            keywords,
            data: SuffixData::Subscript(index),
        })
    }
}
