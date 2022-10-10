use std::io::{stderr, Write};
use std::mem;

use crate::lsp_parser::laze_parser::matcher::extract_ast;
use crate::lsp_parser::peg_parser::parser::{Parser, ParserData};

use super::{
    dec::{self, ClassMemberList, Dec, DecList, Dec_},
    exp::{ASTExp, ASTExpList, ASTExp_},
    field::{Field, FieldList, Field_},
    ifelse::{IfElse, IfElseList},
    op::{Oper, OperList},
    stm::{Stm, StmList, Stm_},
    suffix::ASTExpSuffixList,
    ty::{Type, TypeList, Type_},
    var::{Var, Var_},
};

pub type AST = dec::DecList;

#[derive(Clone, Debug)]
pub struct ID {
    pub pos: (usize, usize),
    pub id: String,
}

impl ID {
    pub fn new(pos: (usize, usize)) -> Self {
        Self {
            pos,
            id: "".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ASTString {
    pub pos: (usize, usize),
    pub str: String,
}

impl ASTString {
    pub fn new(pos: (usize, usize), str: String) -> Self {
        Self { pos, str }
    }
}

#[derive(Clone, Debug)]
pub struct ASTBool {
    pub pos: (usize, usize),
    pub boolean: bool,
}

impl ASTBool {
    pub fn new(pos: (usize, usize), boolean: bool) -> Self {
        Self { pos, boolean }
    }
}

#[derive(Clone, Debug)]
pub enum ASTNode {
    ID(ID),
    IDList(Vec<ID>),
    Dec(Dec),
    Stm(Stm),
    Exp(ASTExp),
    Type(Type),
    Field(Field),
    String(ASTString),
    Var(Var),
    IfElse(IfElse),
    Op(Oper),
    DecList(DecList),
    StmList(StmList),
    ExpList(ASTExpList),
    FieldList(FieldList),
    TypeList(TypeList),
    StringList(Vec<ASTString>),
    IfElseList(IfElseList),
    ExpSuffixList(ASTExpSuffixList),
    OperList(OperList),
    ClassMemberList(ClassMemberList),
    Keywords(Vec<(usize, usize)>),
    None,
}

impl ASTNode {
    pub fn get_keywords(self, _pos: (usize, usize), name: &str, rule: &str) -> Vec<(usize, usize)> {
        if let ASTNode::Keywords(keywords) = self {
            keywords
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a keyword list.");
            vec![]
        }
    }
    pub fn get_var_data(self, pos: (usize, usize), name: &str, rule: &str) -> Var {
        if let ASTNode::Var(var) = self {
            var
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a declaration.");
            Var_::none_var(pos)
        }
    }
    pub fn get_dec_data(self, pos: (usize, usize), name: &str, rule: &str) -> Dec {
        match self {
            ASTNode::Dec(dec) => dec,
            ASTNode::DecList(mut declist) => {
                if declist.len() == 1 {
                    let mut temp_dec = Dec_::none_dec(pos);
                    mem::swap(&mut declist[0], &mut temp_dec);
                    temp_dec
                } else {
                    let _ = writeln!(stderr(), "{name} in {rule} is not a declaration.");
                    Dec_::none_dec(pos)
                }
            }
            _ => {
                let _ = writeln!(stderr(), "{name} in {rule} is not a declaration.");
                Dec_::none_dec(pos)
            }
        }
    }
    pub fn get_declist_data(self, _pos: (usize, usize), name: &str, rule: &str) -> DecList {
        if let ASTNode::DecList(declist) = self {
            declist
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a declaration list.");
            vec![]
        }
    }
    pub fn get_stm_data(self, pos: (usize, usize), name: &str, rule: &str) -> Stm {
        match self {
            ASTNode::Stm(stm) => stm,
            ASTNode::StmList(mut stmlist) => {
                if stmlist.len() == 1 {
                    let mut temp_stm = Stm_::none_stm(pos);
                    mem::swap(&mut stmlist[0], &mut temp_stm);
                    temp_stm
                } else {
                    let _ = writeln!(stderr(), "{name} in {rule} is not a statement.");
                    Stm_::none_stm(pos)
                }
            }
            _ => {
                let _ = writeln!(stderr(), "{name} in {rule} is not a statement.");
                Stm_::none_stm(pos)
            }
        }
    }
    pub fn get_stmlist_data(self, _pos: (usize, usize), name: &str, rule: &str) -> StmList {
        if let ASTNode::StmList(stmlist) = self {
            stmlist
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a statement list.");
            vec![]
        }
    }
    pub fn get_exp_data(self, pos: (usize, usize), name: &str, rule: &str) -> ASTExp {
        match self {
            ASTNode::Exp(exp) => exp,
            ASTNode::ExpList(mut explist) => {
                let mut temp_exp = ASTExp_::none_exp(pos);
                if explist.len() == 1 {
                    mem::swap(&mut explist[0], &mut temp_exp);
                } else if explist.len() == 0 {
                } else {
                    // let _ = writeln!(stderr(), "{name} in {rule} is not an expression.");
                    mem::swap(&mut explist[0], &mut temp_exp);
                }
                temp_exp
            }
            _ => {
                let _ = writeln!(stderr(), "{name} in {rule} is not an expression.");
                return ASTExp_::none_exp(pos);
            }
        }
    }
    pub fn get_explist_data(self, _pos: (usize, usize), name: &str, rule: &str) -> ASTExpList {
        if let ASTNode::ExpList(explist) = self {
            explist
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not an expression list.");
            vec![]
        }
    }
    pub fn get_classmembers_data(
        self,
        _pos: (usize, usize),
        name: &str,
        rule: &str,
    ) -> ClassMemberList {
        if let ASTNode::ClassMemberList(members) = self {
            members
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not an class members list.");
            vec![]
        }
    }
    pub fn get_ty_data(self, pos: (usize, usize), name: &str, rule: &str) -> Type {
        if let ASTNode::Type(ty) = self {
            ty
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a type.");
            Type_::none_type(pos)
        }
    }
    pub fn get_tylist_data(self, _pos: (usize, usize), name: &str, rule: &str) -> TypeList {
        if let ASTNode::TypeList(tylist) = self {
            tylist
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a type.");
            vec![]
        }
    }
    pub fn get_field_data(self, pos: (usize, usize), name: &str, rule: &str) -> Field {
        if let ASTNode::Field(field) = self {
            field
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a field.");
            Field_::none(pos)
        }
    }
    pub fn get_oper_data(self, pos: (usize, usize), name: &str, rule: &str) -> Oper {
        if let ASTNode::Op(oper) = self {
            oper
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a field.");
            Oper::none_op(pos)
        }
    }
    pub fn get_fieldlist_data(self, _pos: (usize, usize), name: &str, rule: &str) -> FieldList {
        if let ASTNode::FieldList(fieldlist) = self {
            fieldlist
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a fieldlist.");
            vec![]
        }
    }
    pub fn get_string_data(self, pos: (usize, usize), name: &str, rule: &str) -> ASTString {
        if let ASTNode::String(str) = self {
            str
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a string.");
            ASTString::new(pos, "".to_string())
        }
    }
    pub fn get_stringlist_data(
        self,
        _pos: (usize, usize),
        name: &str,
        rule: &str,
    ) -> Vec<ASTString> {
        match self {
            ASTNode::StringList(strlist) => strlist,
            ASTNode::String(str) => vec![str],
            _ => {
                let _ = writeln!(stderr(), "{name} in {rule} is not a string list.");
                vec![]
            }
        }
    }
    pub fn get_id_data(self, pos: (usize, usize), name: &str, rule: &str) -> ID {
        if let ASTNode::ID(id) = self {
            id
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a string.");
            ID::new(pos)
        }
    }
    pub fn get_idlist_data(self, _pos: (usize, usize), name: &str, rule: &str) -> Vec<ID> {
        match self {
            ASTNode::IDList(idlist) => idlist,
            ASTNode::ID(id) => vec![id],
            _ => {
                let _ = writeln!(stderr(), "{name} in {rule} is not a string list.");
                vec![]
            }
        }
    }
    pub fn get_suffixlist_data(
        self,
        _pos: (usize, usize),
        name: &str,
        rule: &str,
    ) -> ASTExpSuffixList {
        if let ASTNode::ExpSuffixList(suffixlist) = self {
            suffixlist
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a string list.");
            vec![]
        }
    }
    pub fn get_ifelselist_data(self, _pos: (usize, usize), name: &str, rule: &str) -> IfElseList {
        if let ASTNode::IfElseList(ifelselist) = self {
            ifelselist
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a string list.");
            vec![]
        }
    }
    pub fn get_operlist_data(self, _pos: (usize, usize), name: &str, rule: &str) -> OperList {
        if let ASTNode::OperList(oplist) = self {
            oplist
        } else {
            let _ = writeln!(stderr(), "{name} in {rule} is not a string list.");
            vec![]
        }
    }
}

impl ParserData for ASTNode {
    fn string(pos: (usize, usize), str: String) -> Self {
        Self::String(ASTString::new(pos, str))
    }
    fn null() -> Self {
        Self::None
    }
    fn keywords(pos: (usize, usize), parser: &mut Parser<Self>) -> Self {
        match parser.get_data("keywords") {
            Some(node) => match node {
                ASTNode::Keywords(mut keywords) => {
                    keywords.push(pos);
                    ASTNode::Keywords(keywords)
                }
                _ => {
                    let _ = writeln!(stderr(), "Keywords is not a keyword list.");
                    ASTNode::None
                }
            },
            None => ASTNode::Keywords(vec![pos]),
        }
    }
    fn data(pos: (usize, usize), name: &str, parser: &mut Parser<Self>) -> Self {
        extract_ast(pos, name, parser)
    }
    fn is_null(&self) -> bool {
        if let Self::None = self {
            true
        } else {
            false
        }
    }
}
