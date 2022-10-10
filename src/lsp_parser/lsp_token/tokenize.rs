use std::io::{stderr, Write};

use crate::lsp_parser::ast::{
    ast::*,
    dec::*,
    exp::{ASTExp, ASTExpData, ASTExpList},
    field::{Field, FieldData, FieldList},
    ifelse::{IfElse, IfElseData, IfElseList},
    op::{Oper, OperList},
    stm::{Stm, StmData, StmList},
    suffix::SuffixData,
    ty::{Type, TypeData, TypeList},
    var::{Var, VarData},
};

use super::tokens::{Token, TokenList, TokenModifier, TokenType};

pub fn tokenize_keywords(
    keywords: &Vec<(usize, usize)>,
    modifier: TokenModifier,
    tokenlist: &mut TokenList,
) {
    for keyword in keywords {
        tokenlist.push(Token::new(*keyword, TokenType::Keyword, modifier));
    }
}

pub fn tokenize_ast(ast: ASTNode) -> TokenList {
    println!("{:?}", ast);
    let mut tokenlist = vec![];
    match ast {
        ASTNode::DecList(declist) => {
            tokenize_declist(&declist, &mut tokenlist);
        }
        _ => {
            let _ = writeln!(stderr(), "The AST's parent node is not a declaration list.");
        }
    }
    tokenlist.sort_by(|a, b| a.pos.cmp(&b.pos));
    tokenlist
}

pub fn tokenize_declist(declist: &DecList, tokenlist: &mut TokenList) {
    for dec in declist {
        tokenize_dec(dec, tokenlist);
    }
}

pub fn tokenize_dec(dec: &Dec, tokenlist: &mut TokenList) {
    tokenize_keywords(&dec.keywords, TokenModifier::Declaration, tokenlist);
    match &dec.data {
        DecData::Class(name, member_list, inheritance) => {
            tokenlist.push(Token::new(
                name.pos,
                TokenType::Class,
                TokenModifier::Definition,
            ));
            for id in inheritance {
                tokenlist.push(Token::new(
                    id.pos,
                    TokenType::Class,
                    TokenModifier::Definition,
                ));
            }
            for member in member_list {
                tokenize_member(&member, tokenlist);
            }
        }
        DecData::Func(name, params, result, func_body) => {
            tokenlist.push(Token::new(
                name.pos,
                TokenType::Function,
                TokenModifier::Definition,
            ));
            for param in params {
                tokenize_field(param, tokenlist);
            }
            for field in result {
                tokenize_field(field, tokenlist);
            }
            tokenize_stm(func_body, tokenlist);
        }
        DecData::JsExport(name, export_name) => {
            tokenlist.push(Token::new(
                name.pos,
                TokenType::Function,
                TokenModifier::Abstract,
            ));
            tokenlist.push(Token::new(
                export_name.pos,
                TokenType::String,
                TokenModifier::Abstract,
            ));
        }
        DecData::JsImport(name, params, result, module, id) => {
            tokenlist.push(Token::new(
                name.pos,
                TokenType::Function,
                TokenModifier::Definition,
            ));
            for param in params {
                tokenize_field(param, tokenlist);
            }
            for field in result {
                tokenize_field(field, tokenlist);
            }
            tokenlist.push(Token::new(
                module.pos,
                TokenType::String,
                TokenModifier::Abstract,
            ));
            tokenlist.push(Token::new(
                id.pos,
                TokenType::String,
                TokenModifier::Abstract,
            ));
        }
        DecData::Oper(op, params, result, func_body) => {
            tokenlist.push(Token::new(
                op.pos,
                TokenType::Function,
                TokenModifier::Definition,
            ));
            for param in params {
                tokenize_field(param, tokenlist);
            }
            for field in result {
                tokenize_field(field, tokenlist);
            }
            tokenize_stm(func_body, tokenlist);
        }
        DecData::Template(dec, type_params) => {
            tokenize_dec(dec, tokenlist);
            for param in type_params {
                tokenlist.push(Token::new(
                    param.pos,
                    TokenType::Class,
                    TokenModifier::Abstract,
                ));
            }
        }
        DecData::Var(var, ty, init) => {
            tokenize_var(var, tokenlist);
            tokenize_ty(ty, tokenlist);
            tokenize_exp(init, tokenlist);
        }
        DecData::None => {}
    }
}

pub fn tokenize_stmlist(stmlist: &StmList, tokenlist: &mut TokenList) {
    for stm in stmlist {
        tokenize_stm(&stm, tokenlist);
    }
}

pub fn tokenize_stm(stm: &Stm, tokenlist: &mut TokenList) {
    tokenize_keywords(&stm.keywords, TokenModifier::Abstract, tokenlist);
    match &stm.data {
        StmData::Assign(var, exp, _) => {
            tokenize_var(var, tokenlist);
            tokenize_exp(exp, tokenlist);
        }
        StmData::Break => {}
        StmData::Compound(stmlist) => {
            tokenize_stmlist(stmlist, tokenlist);
        }
        StmData::Dec(dec) => {
            tokenize_dec(dec, tokenlist);
        }
        StmData::Exp(exp) => {
            tokenize_exp(exp, tokenlist);
        }
        StmData::For(init, test, incr, body) => {
            tokenize_stm(init, tokenlist);
            tokenize_exp(test, tokenlist);
            tokenize_stm(incr, tokenlist);
            tokenize_stm(body, tokenlist);
        }
        StmData::IfElse(ifelselist) => {
            tokenize_ifelselist(ifelselist, tokenlist);
        }
        StmData::Loop(body) => {
            tokenize_stm(body, tokenlist);
        }
        StmData::Repeat(limit, body) => {
            tokenize_exp(limit, tokenlist);
            tokenize_stm(body, tokenlist);
        }
        StmData::Return(val) => {
            tokenize_exp(val, tokenlist);
        }
        StmData::While(test_exp, body) => {
            tokenize_exp(test_exp, tokenlist);
            tokenize_stm(body, tokenlist);
        }
        _ => {}
    }
}

pub fn tokenize_explist(explist: &ASTExpList, tokenlist: &mut TokenList) {
    for exp in explist {
        tokenize_exp(exp, tokenlist);
    }
}

pub fn tokenize_exp(exp: &ASTExp, tokenlist: &mut TokenList) {
    tokenize_keywords(&exp.keywords, TokenModifier::Abstract, tokenlist);
    match &exp.data {
        ASTExpData::Array(explist) => {
            tokenize_explist(&explist, tokenlist);
        }
        ASTExpData::BinOp(oper_list, explist) => {
            tokenize_oplist(oper_list, tokenlist);
            tokenize_explist(explist, tokenlist);
        }
        ASTExpData::Bool(b) => {
            tokenlist.push(Token::new(
                b.pos,
                TokenType::Variable,
                TokenModifier::Static,
            ));
        }
        ASTExpData::Func(params, result, func_body) => {
            tokenize_fieldlist(params, tokenlist);
            tokenize_fieldlist(result, tokenlist);
            tokenize_stm(func_body, tokenlist);
        }
        ASTExpData::Int(i) => {
            tokenlist.push(Token::new(
                i.pos,
                TokenType::Number,
                TokenModifier::Abstract,
            ));
        }
        ASTExpData::Paren(e) => {
            tokenize_exp(e, tokenlist);
        }
        ASTExpData::Real(r) => {
            tokenlist.push(Token::new(
                r.pos,
                TokenType::Number,
                TokenModifier::Abstract,
            ));
        }
        ASTExpData::Short(s) => {
            tokenlist.push(Token::new(
                s.pos,
                TokenType::Number,
                TokenModifier::Abstract,
            ));
        }
        ASTExpData::SizeOf(e) => {
            tokenize_exp(e, tokenlist);
        }
        ASTExpData::String(str) => {
            tokenlist.push(Token::new(
                str.pos,
                TokenType::String,
                TokenModifier::Abstract,
            ));
        }
        ASTExpData::UnaryOp(oplist, exp) => {
            tokenize_oplist(oplist, tokenlist);
            tokenize_exp(exp, tokenlist);
        }
        ASTExpData::Var(var) => {
            tokenize_var(var, tokenlist);
        }
        _ => {}
    }
}

pub fn tokenize_var(var: &Var, tokenlist: &mut TokenList) {
    tokenize_keywords(&var.keywords, TokenModifier::Abstract, tokenlist);
    match &var.data {
        VarData::Simple(id) => {
            tokenlist.push(Token::new(
                id.pos,
                TokenType::Variable,
                TokenModifier::Abstract,
            ));
        }
        VarData::SuffixVar(id, suffix_list) => {
            let mut suffix_list_iter = suffix_list.iter().peekable();
            if suffix_list.len() > 0 {
                if let SuffixData::Call(_) = &suffix_list[0].data {
                    tokenlist.push(Token::new(
                        id.pos,
                        TokenType::Function,
                        TokenModifier::Abstract,
                    ));
                } else {
                    tokenlist.push(Token::new(
                        id.pos,
                        TokenType::Variable,
                        TokenModifier::Abstract,
                    ));
                }
            }
            while let Some(suffix) = suffix_list_iter.next() {
                tokenize_keywords(&suffix.keywords, TokenModifier::Abstract, tokenlist);
                match &suffix.data {
                    SuffixData::Call(explist) => {
                        tokenize_explist(&explist, tokenlist);
                    }
                    SuffixData::Arrow(id) | SuffixData::Dot(id) => {
                        if let Some(next_suffix) = suffix_list_iter.peek() {
                            match &next_suffix.data {
                                SuffixData::Call(_) => {
                                    tokenlist.push(Token::new(
                                        id.pos,
                                        TokenType::Function,
                                        TokenModifier::Abstract,
                                    ));
                                }
                                _ => {
                                    tokenlist.push(Token::new(
                                        id.pos,
                                        TokenType::Variable,
                                        TokenModifier::Abstract,
                                    ));
                                }
                            };
                        } else {
                            tokenlist.push(Token::new(
                                id.pos,
                                TokenType::Variable,
                                TokenModifier::Abstract,
                            ));
                        }
                    }
                    SuffixData::Subscript(index) => {
                        tokenize_exp(index, tokenlist);
                    }
                }
            }
        }
        VarData::Pointer(var) => {
            tokenize_var(var, tokenlist);
        }
        VarData::None => {}
    }
}

pub fn tokenize_ty(ty: &Type, tokenlist: &mut TokenList) {
    tokenize_keywords(&ty.keywords, TokenModifier::Abstract, tokenlist);
    match &ty.data {
        TypeData::Array(ty, index) => {
            tokenize_ty(&ty, tokenlist);
            tokenize_exp(index, tokenlist);
        }
        TypeData::Bool
        | TypeData::Char
        | TypeData::Int
        | TypeData::Real
        | TypeData::Short
        | TypeData::Void => {
            tokenlist.push(Token::new(ty.pos, TokenType::Type, TokenModifier::Abstract));
        }
        TypeData::Func(params, result) => {
            tokenize_fieldlist(params, tokenlist);
            tokenize_ty(result, tokenlist);
        }
        TypeData::Name(id) => {
            tokenlist.push(Token::new(
                id.pos,
                TokenType::Class,
                TokenModifier::Abstract,
            ));
        }
        TypeData::Pointer(ty) => {
            tokenize_ty(ty, tokenlist);
        }
        TypeData::Template(id, type_params) => {
            tokenlist.push(Token::new(
                id.pos,
                TokenType::Class,
                TokenModifier::Abstract,
            ));
            tokenize_tylist(type_params, tokenlist);
        }
        TypeData::None => {}
    }
}

pub fn tokenize_tylist(tylist: &TypeList, tokenlist: &mut TokenList) {
    for ty in tylist {
        tokenize_ty(ty, tokenlist);
    }
}

pub fn tokenize_field(field: &Field, tokenlist: &mut TokenList) {
    tokenize_keywords(&field.keywords, TokenModifier::Abstract, tokenlist);
    match &field.data {
        FieldData::Field(var, ty) => {
            tokenize_var(&var, tokenlist);
            tokenize_ty(&ty, tokenlist);
        }
        FieldData::None => {}
    }
}

pub fn tokenize_fieldlist(fieldlist: &FieldList, tokenlist: &mut TokenList) {
    for field in fieldlist {
        tokenize_field(field, tokenlist);
    }
}

pub fn tokenize_member(member: &ClassMember, tokenlist: &mut TokenList) {
    tokenize_dec(&member.dec, tokenlist);
}

pub fn tokenize_op(op: &Oper, tokenlist: &mut TokenList) {
    tokenlist.push(Token::new(
        op.pos,
        TokenType::Operator,
        TokenModifier::Abstract,
    ));
}

pub fn tokenize_oplist(oplist: &OperList, tokenlist: &mut TokenList) {
    for op in oplist {
        tokenize_op(op, tokenlist);
    }
}

pub fn tokenize_ifelse(ifelse: &IfElse, tokenlist: &mut TokenList) {
    tokenize_keywords(&ifelse.keywords, TokenModifier::Abstract, tokenlist);
    match &ifelse.data {
        IfElseData::If(test_exp, body) => {
            tokenize_exp(test_exp, tokenlist);
            tokenize_stm(body, tokenlist);
        }
        IfElseData::ElseIf(test_exp, body) => {
            tokenize_exp(test_exp, tokenlist);
            tokenize_stm(body, tokenlist);
        }
        IfElseData::Else(body) => {
            tokenize_stm(body, tokenlist);
        }
    }
}

pub fn tokenize_ifelselist(ifelselist: &IfElseList, tokenlist: &mut TokenList) {
    for ifelse in ifelselist {
        tokenize_ifelse(ifelse, tokenlist);
    }
}
