use crate::lsp_parser::ast::{
    ast::*, dec::*, exp::*, field::*, ifelse::*, op::*, stm::*, suffix::*, ty::*, var::*,
};

pub fn extract_keywords(
    pos: (usize, usize),
    data: Option<ASTNode>,
    name: &str,
    rule: &str,
) -> Vec<(usize, usize)> {
    match data {
        Some(data) => data.get_keywords(pos, name, rule),
        None => vec![],
    }
}
pub fn extract_var_data(pos: (usize, usize), data: Option<ASTNode>, name: &str, rule: &str) -> Var {
    match data {
        Some(data) => data.get_var_data(pos, name, rule),
        None => Var_::none_var(pos),
    }
}
pub fn extract_suffixlist_data(
    pos: (usize, usize),
    data: Option<ASTNode>,
    name: &str,
    rule: &str,
) -> ASTExpSuffixList {
    match data {
        Some(data) => data.get_suffixlist_data(pos, name, rule),
        None => vec![],
    }
}
pub fn extract_string_data(
    pos: (usize, usize),
    data: Option<ASTNode>,
    name: &str,
    rule: &str,
) -> ASTString {
    match data {
        Some(data) => data.get_string_data(pos, name, rule),
        None => ASTString::new(pos, "".to_string()),
    }
}
pub fn extract_id_data(pos: (usize, usize), data: Option<ASTNode>, name: &str, rule: &str) -> ID {
    match data {
        Some(data) => data.get_id_data(pos, name, rule),
        None => ID::new(pos),
    }
}
pub fn extract_oper_data(
    pos: (usize, usize),
    data: Option<ASTNode>,
    name: &str,
    rule: &str,
) -> Oper {
    match data {
        Some(data) => data.get_oper_data(pos, name, rule),
        None => Oper::none_op(pos),
    }
}
pub fn extract_operlist_data(
    pos: (usize, usize),
    data: Option<ASTNode>,
    name: &str,
    rule: &str,
) -> OperList {
    match data {
        Some(data) => data.get_operlist_data(pos, name, rule),
        None => vec![],
    }
}
pub fn extract_stringlist_data(
    pos: (usize, usize),
    data: Option<ASTNode>,
    name: &str,
    rule: &str,
) -> Vec<ASTString> {
    match data {
        Some(data) => data.get_stringlist_data(pos, name, rule),
        None => vec![],
    }
}
pub fn extract_idlist_data(
    pos: (usize, usize),
    data: Option<ASTNode>,
    name: &str,
    rule: &str,
) -> Vec<ID> {
    match data {
        Some(data) => data.get_idlist_data(pos, name, rule),
        None => vec![],
    }
}

pub fn extract_dec_data(pos: (usize, usize), data: Option<ASTNode>, name: &str, rule: &str) -> Dec {
    match data {
        Some(data) => data.get_dec_data(pos, name, rule),
        None => Box::new(Dec_ {
            pos,
            keywords: vec![],
            data: DecData::None,
        }),
    }
}

pub fn extract_declist_data(
    pos: (usize, usize),
    data: Option<ASTNode>,
    name: &str,
    rule: &str,
) -> DecList {
    match data {
        Some(data) => data.get_declist_data(pos, name, rule),
        None => vec![],
    }
}

pub fn extract_stm_data(pos: (usize, usize), data: Option<ASTNode>, name: &str, rule: &str) -> Stm {
    match data {
        Some(data) => data.get_stm_data(pos, name, rule),
        None => Stm_::none_stm(pos),
    }
}
pub fn extract_stmlist_data(
    pos: (usize, usize),
    data: Option<ASTNode>,
    name: &str,
    rule: &str,
) -> StmList {
    match data {
        Some(data) => data.get_stmlist_data(pos, name, rule),
        None => vec![],
    }
}

pub fn extract_exp_data(
    pos: (usize, usize),
    data: Option<ASTNode>,
    name: &str,
    rule: &str,
) -> ASTExp {
    match data {
        Some(data) => data.get_exp_data(pos, name, rule),
        None => ASTExp_::none_exp(pos),
    }
}
pub fn extract_explist_data(
    pos: (usize, usize),
    data: Option<ASTNode>,
    name: &str,
    rule: &str,
) -> ASTExpList {
    match data {
        Some(data) => data.get_explist_data(pos, name, rule),
        None => vec![],
    }
}

pub fn extract_ty_data(pos: (usize, usize), data: Option<ASTNode>, name: &str, rule: &str) -> Type {
    match data {
        Some(data) => data.get_ty_data(pos, name, rule),
        None => Type_::none_type(pos),
    }
}
pub fn extract_tylist_data(
    pos: (usize, usize),
    data: Option<ASTNode>,
    name: &str,
    rule: &str,
) -> TypeList {
    match data {
        Some(data) => data.get_tylist_data(pos, name, rule),
        None => vec![],
    }
}
pub fn extract_classmembers_data(
    pos: (usize, usize),
    data: Option<ASTNode>,
    name: &str,
    rule: &str,
) -> ClassMemberList {
    match data {
        Some(data) => data.get_classmembers_data(pos, name, rule),
        None => vec![],
    }
}

pub fn extract_field_data(
    pos: (usize, usize),
    data: Option<ASTNode>,
    name: &str,
    rule: &str,
) -> Field {
    match data {
        Some(data) => data.get_field_data(pos, name, rule),
        None => Field_::none(pos),
    }
}
pub fn extract_fieldlist_data(
    pos: (usize, usize),
    data: Option<ASTNode>,
    name: &str,
    rule: &str,
) -> FieldList {
    match data {
        Some(data) => data.get_fieldlist_data(pos, name, rule),
        None => vec![],
    }
}
pub fn extract_ifelselist_data(
    pos: (usize, usize),
    data: Option<ASTNode>,
    name: &str,
    rule: &str,
) -> IfElseList {
    match data {
        Some(data) => data.get_ifelselist_data(pos, name, rule),
        None => vec![],
    }
}
