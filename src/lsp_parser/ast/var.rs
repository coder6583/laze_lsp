use super::{ast::ID, suffix::ASTExpSuffixList};

pub type Var = Box<Var_>;

#[derive(Clone, Debug)]
pub struct Var_ {
    pub pos: (usize, usize),
    pub keywords: Vec<(usize, usize)>,
    pub data: VarData,
}

#[derive(Clone, Debug)]
pub enum VarData {
    Simple(ID),
    SuffixVar(Var, ASTExpSuffixList),
    Pointer(Var),
    None,
}

impl Var_ {
    pub fn none_var(pos: (usize, usize)) -> Var {
        Box::new(Var_ {
            pos,
            keywords: vec![],
            data: VarData::None,
        })
    }
    pub fn simple_var(pos: (usize, usize), name: ID, keywords: Vec<(usize, usize)>) -> Var {
        Box::new(Var_ {
            pos,
            keywords,
            data: VarData::Simple(name),
        })
    }
    pub fn suffix_var(
        pos: (usize, usize),
        var: Var,
        suffixlist: ASTExpSuffixList,
        keywords: Vec<(usize, usize)>,
    ) -> Var {
        Box::new(Var_ {
            pos,
            keywords,
            data: VarData::SuffixVar(var, suffixlist),
        })
    }
    pub fn pointer_var(pos: (usize, usize), var: Var, keywords: Vec<(usize, usize)>) -> Var {
        Box::new(Var_ {
            pos,
            keywords,
            data: VarData::Pointer(var),
        })
    }
}
