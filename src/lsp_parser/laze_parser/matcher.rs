use std::io::{stderr, Write};

use crate::lsp_parser::{
    ast::{ast::*, dec::*, exp::*, field::*, ifelse::*, op::*, stm::*, suffix::*, ty::*, var::*},
    peg_parser::parser::Parser,
};

use super::extracter::*;

pub fn extract_ast(pos: (usize, usize), name: &str, parser: &mut Parser<ASTNode>) -> ASTNode {
    let keywords = extract_keywords(pos, parser.get_data("keywords"), "keywords", name);
    // println!("Reducing: {}", name);
    match name {
        "String" => {
            // println!("StringContent: {newcontent}");
            ASTNode::String(extract_string_data(
                pos,
                parser.get_data("string"),
                "string",
                name,
            ))
        }
        "Real" => ASTNode::String(extract_string_data(
            pos,
            parser.get_data("real"),
            "real",
            name,
        )),
        "Integer" => ASTNode::String(extract_string_data(
            pos,
            parser.get_data("int"),
            "int",
            name,
        )),
        "ID" => {
            let id_ = extract_string_data(pos, parser.get_data("id"), "id", name);
            let id = id_.str.trim().to_string();
            match parser.get_data_from_parent_scope("ID") {
                Some(node) => match node {
                    ASTNode::ID(ast_id) => ASTNode::IDList(vec![ast_id, ID { pos, id }]),
                    ASTNode::IDList(strlist) => {
                        let mut list = strlist;
                        list.push(ID { pos, id });
                        ASTNode::IDList(list)
                    }
                    _ => {
                        panic!("ID is not type ID.");
                    }
                },
                None => ASTNode::ID(ID { pos, id }),
            }
        }
        "IDList" => ASTNode::IDList(extract_idlist_data(pos, parser.get_data("ID"), "ID", name)),
        "Var" => ASTNode::Var(extract_var_data(pos, parser.get_data("var"), "var", name)),
        "SimpleVar" => ASTNode::Var(Var_::simple_var(
            pos,
            extract_id_data(pos, parser.get_data("ID"), "ID", name),
            keywords,
        )),
        "ParenVar" => ASTNode::Var(extract_var_data(pos, parser.get_data("Var"), "Var", name)),
        "PrimaryVar" => ASTNode::Var(extract_var_data(pos, parser.get_data("var"), "var", name)),
        "SuffixVar" => {
            let suffix = extract_suffixlist_data(pos, parser.get_data("suffix"), "suffix", name);
            if suffix.len() > 0 {
                ASTNode::Var(Var_::suffix_var(
                    pos,
                    extract_var_data(pos, parser.get_data("PrimaryVar"), "PrimaryVar", name),
                    suffix,
                    keywords,
                ))
            } else {
                ASTNode::Var(extract_var_data(
                    pos,
                    parser.get_data("PrimaryVar"),
                    "PrimaryVar",
                    name,
                ))
            }
        }
        "PointerVar" => match parser.get_data("pointer") {
            Some(node) => {
                if let ASTNode::String(str) = node {
                    if str.str.starts_with("*") {
                        let mut var = Var_::pointer_var(
                            pos,
                            extract_var_data(pos, parser.get_data("SuffixVar"), "SuffixVar", name),
                            keywords.clone(),
                        );
                        for index in 1..str.str.len() {
                            let slice = &str.str[index..index + 1];
                            if slice == "*" {
                                var = Var_::pointer_var(pos, var, keywords.clone());
                            } else {
                                break;
                            }
                        }
                        ASTNode::Var(var)
                    } else {
                        ASTNode::Var(extract_var_data(
                            pos,
                            parser.get_data("SuffixVar"),
                            "SuffixVar",
                            name,
                        ))
                    }
                } else {
                    ASTNode::None
                }
            }
            None => ASTNode::None,
        },
        "PointerType" => ASTNode::Type(Type_::pointer_type(
            pos,
            extract_ty_data(pos, parser.get_data("PrimaryType"), "ParenType", name),
            keywords,
        )),
        "ParenType" => ASTNode::Type(extract_ty_data(pos, parser.get_data("Type"), "Type", name)),
        "IntType" => ASTNode::Type(Type_::int_type(pos, keywords)),
        "ShortType" => ASTNode::Type(Type_::short_type(pos, keywords)),
        "CharType" => ASTNode::Type(Type_::char_type(pos, keywords)),
        "RealType" => ASTNode::Type(Type_::int_type(pos, keywords)),
        "BoolType" => ASTNode::Type(Type_::bool_type(pos, keywords)),
        "NameType" => ASTNode::Type(Type_::name_type(
            pos,
            extract_id_data(pos, parser.get_data("ID"), "ID", name),
            keywords,
        )),
        "GenericsType" => ASTNode::Type(Type_::template_type(
            pos,
            extract_id_data(pos, parser.get_data("ID"), "ID", name),
            extract_tylist_data(pos, parser.get_data("IDList"), "IDList", name),
            keywords,
        )),
        "ArrayType" => match parser.get_data("exp") {
            Some(exp) => ASTNode::Type(Type_::array_type(
                pos,
                extract_ty_data(pos, parser.get_data("PrimaryType"), "PrimaryType", name),
                exp.get_exp_data(pos, "exp", name),
                keywords,
            )),
            None => ASTNode::Type(extract_ty_data(
                pos,
                parser.get_data("PrimaryType"),
                "PrimaryType",
                name,
            )),
        },
        "PrimaryType" => ASTNode::Type(extract_ty_data(pos, parser.get_data("type"), "type", name)),
        "Type" => ASTNode::Type(extract_ty_data(pos, parser.get_data("type"), "type", name)),
        "If" | "ElseIf" => {
            let test = extract_exp_data(pos, parser.get_data("Exp"), "Exp", name);
            let body = extract_stm_data(pos, parser.get_data("Stm"), "Stm", name);
            if name == "If" {
                ASTNode::IfElseList(vec![IfElse_::if_(pos, test, body, keywords)])
            } else if name == "ElseIf" {
                match parser.get_data_from_parent_scope("ifelse") {
                    Some(node) => match node {
                        ASTNode::IfElseList(mut list) => {
                            list.push(IfElse_::else_if(pos, test, body, keywords));
                            return ASTNode::IfElseList(list);
                        }
                        _ => ASTNode::None,
                    },
                    None => ASTNode::IfElseList(vec![IfElse_::else_if(pos, test, body, keywords)]),
                }
            } else {
                ASTNode::None
            }
        }
        "Else" => match parser.get_data_from_parent_scope("ifelse") {
            Some(node) => match node {
                ASTNode::IfElseList(mut list) => {
                    list.push(IfElse_::else_(
                        pos,
                        extract_stm_data(pos, parser.get_data("Stm"), "Stm", name),
                        keywords,
                    ));
                    return ASTNode::IfElseList(list);
                }
                _ => ASTNode::None,
            },
            None => ASTNode::IfElseList(vec![IfElse_::else_(
                pos,
                extract_stm_data(pos, parser.get_data("Stm"), "Stm", name),
                keywords,
            )]),
        },
        "IfElseList" => ASTNode::IfElseList(extract_ifelselist_data(
            pos,
            parser.get_data("ifelse"),
            "ifelse",
            name,
        )),
        "LoopStm" => ASTNode::Stm(Stm_::loop_stm(
            pos,
            extract_stm_data(pos, parser.get_data("Stm"), "Stm", name),
            keywords,
        )),
        "ReturnStm" => ASTNode::Stm(Stm_::return_stm(
            pos,
            match parser.get_data("Exp") {
                Some(exp) => match exp {
                    ASTNode::Exp(e) => e,
                    _ => ASTExp_::none_exp(pos),
                },
                None => ASTExp_::none_exp(pos),
            },
            keywords,
        )),
        "ContinueStm" => ASTNode::Stm(Stm_::continue_stm(pos, keywords)),
        "BreakStm" => ASTNode::Stm(Stm_::break_stm(pos, keywords)),
        "RepeatStm" => ASTNode::Stm(Stm_::repeat_stm(
            pos,
            extract_exp_data(pos, parser.get_data("Exp"), "Exp", name),
            extract_stm_data(pos, parser.get_data("Stm"), "Stm", name),
            keywords,
        )),
        "UntilStm" => ASTNode::Stm(Stm_::while_stm(
            pos,
            ASTExp_::unaryop_exp(
                pos,
                vec![Oper::new(pos, OperData::Not)],
                extract_exp_data(pos, parser.get_data("Exp"), "Exp", name),
                Vec::new(),
            ),
            extract_stm_data(pos, parser.get_data("Stm"), "Stm", name),
            keywords,
        )),
        "WhileStm" => ASTNode::Stm(Stm_::while_stm(
            pos,
            extract_exp_data(pos, parser.get_data("Exp"), "Exp", name),
            extract_stm_data(pos, parser.get_data("Stm"), "Stm", name),
            keywords,
        )),
        "IfStm" => ASTNode::Stm(Stm_::ifelse_stm(
            pos,
            extract_ifelselist_data(pos, parser.get_data("IfElseList"), "IfElseList", name),
            keywords,
        )),
        "AssignStm" => ASTNode::Stm(extract_stm_data(pos, parser.get_data("stm"), "stm", name)),
        "NormalAssign" | "AddAssign" | "SubAssign" | "MulAssign" | "DivAssign" => {
            ASTNode::Stm(Stm_::assign_stm(
                pos,
                extract_var_data(pos, parser.get_data("Var"), "Var", name),
                extract_exp_data(pos, parser.get_data("Exp"), "Exp", name),
                match name {
                    "NormalAssign" => AssignType::Normal,
                    "AddAssign" => AssignType::Add,
                    "SubAssign" => AssignType::Sub,
                    "MulAssign" => AssignType::Mul,
                    "DivAssign" => AssignType::Div,
                    _ => AssignType::Normal,
                },
                keywords,
            ))
        }
        "DecStm" => ASTNode::Stm(Stm_::dec_stm(
            pos,
            extract_dec_data(pos, parser.get_data("Dec"), "Dec", name),
            keywords,
        )),
        "ExpStm" => ASTNode::Stm(Stm_::exp_stm(
            pos,
            extract_exp_data(pos, parser.get_data("Exp"), "Exp", name),
            keywords,
        )),
        "Stm" => match parser.get_data_from_parent_scope("Stm") {
            Some(node) => match node {
                ASTNode::Stm(stm) => ASTNode::StmList(vec![
                    stm,
                    extract_stm_data(pos, parser.get_data("stm"), "stm", name),
                ]),
                ASTNode::StmList(mut stmlist) => {
                    stmlist.push(extract_stm_data(pos, parser.get_data("stm"), "stm", name));
                    ASTNode::StmList(stmlist)
                }
                _ => {
                    let _ = writeln!(stderr(), "stm is not a statement or a statement list.");
                    ASTNode::None
                }
            },
            None => ASTNode::Stm(extract_stm_data(pos, parser.get_data("stm"), "stm", name)),
        },
        "StmList" => match parser.get_data("Stm") {
            Some(node) => match node {
                ASTNode::Stm(stm) => ASTNode::StmList(vec![stm]),
                ASTNode::StmList(stmlist) => ASTNode::StmList(stmlist),
                _ => ASTNode::None,
            },
            None => ASTNode::StmList(vec![]),
        },
        "CompoundStm" => ASTNode::Stm(Stm_::compound_stm(
            pos,
            extract_stmlist_data(pos, parser.get_data("StmList"), "StmList", name),
            keywords,
        )),
        "IntExp" => ASTNode::Exp(ASTExp_::int_exp(
            pos,
            extract_string_data(pos, parser.get_data("Integer"), "Integer", name),
            keywords,
        )),
        "RealExp" => ASTNode::Exp(ASTExp_::real_exp(
            pos,
            extract_string_data(pos, parser.get_data("Real"), "Real", name),
            keywords,
        )),
        "StringExp" => ASTNode::Exp(ASTExp_::string_exp(
            pos,
            extract_string_data(pos, parser.get_data("String"), "String", name),
            keywords,
        )),
        "ConstantExp" | "PrimaryExp" => {
            let exp = ASTNode::Exp(extract_exp_data(pos, parser.get_data("exp"), "exp", name));
            match parser.get_data_from_parent_scope("exp") {
                Some(node) => match node {
                    ASTNode::ExpList(mut explist) => {
                        explist.push(exp.get_exp_data(pos, "exp", name));
                        ASTNode::ExpList(explist)
                    }
                    ASTNode::Exp(e) => {
                        ASTNode::ExpList(vec![e, exp.get_exp_data(pos, "exp", name)])
                    }
                    _ => ASTNode::ExpList(vec![exp.get_exp_data(pos, "exp", name)]),
                },
                None => ASTNode::ExpList(vec![exp.get_exp_data(pos, "exp", name)]),
            }
        }
        "ParenExp" => ASTNode::Exp(ASTExp_::paren_exp(
            pos,
            extract_exp_data(pos, parser.get_data("exp"), "exp", name),
            keywords,
        )),
        "SizeOfExp" => ASTNode::Exp(ASTExp_::sizeof_exp(
            pos,
            extract_exp_data(pos, parser.get_data("exp"), "exp", name),
            keywords,
        )),
        "ArrayExp" => ASTNode::Exp(ASTExp_::array_exp(
            pos,
            extract_explist_data(pos, parser.get_data("ExpList"), "ExpList", name),
            keywords,
        )),
        "FuncExp" => ASTNode::Exp(ASTExp_::func_exp(
            pos,
            extract_fieldlist_data(pos, parser.get_data("params"), "params", name),
            extract_fieldlist_data(pos, parser.get_data("result"), "result", name),
            extract_stm_data(pos, parser.get_data("Stm"), "Stm", name),
            keywords,
        )),
        "VarExp" => ASTNode::Exp(ASTExp_::var_exp(
            pos,
            extract_var_data(pos, parser.get_data("Var"), "Var", name),
            keywords,
        )),
        "BoolExp" => ASTNode::Exp(extract_exp_data(pos, parser.get_data("bool"), "bool", name)),
        "CallSuffix" | "DotSuffix" | "ArrowSuffix" | "SubscriptSuffix" => {
            let data = if name == "CallSuffix" {
                ASTExpSuffix_::call_suffix(
                    pos,
                    extract_explist_data(pos, parser.get_data("explist"), "explist", name),
                    keywords,
                )
            } else {
                if name == "DotSuffix" {
                    ASTExpSuffix_::dot_suffix(
                        pos,
                        extract_id_data(pos, parser.get_data("ID"), "ID", name),
                        keywords,
                    )
                } else if name == "ArrowSuffix" {
                    ASTExpSuffix_::arrow_suffix(
                        pos,
                        extract_id_data(pos, parser.get_data("ID"), "ID", name),
                        keywords,
                    )
                } else if name == "SubscriptSuffix" {
                    ASTExpSuffix_::subscript_suffix(
                        pos,
                        extract_exp_data(pos, parser.get_data("exp"), "exp", name),
                        keywords,
                    )
                } else {
                    panic!("suffix is not dot nor arrow nor subscript.");
                }
            };
            match parser.get_data_from_parent_scope("suffix") {
                Some(node) => match node {
                    ASTNode::ExpSuffixList(mut list) => {
                        list.push(data);
                        ASTNode::ExpSuffixList(list)
                    }
                    _ => {
                        let _ = writeln!(stderr(), "suffix is not ExpSuffixList.");
                        ASTNode::ExpSuffixList(vec![data])
                    }
                },
                None => ASTNode::ExpSuffixList(vec![data]),
            }
        }
        "AndOp" | "OrOp" | "EqOp" | "NeOp" | "LtOp" | "LeOp" | "GtOp" | "GeOp" | "AddOp"
        | "SubOp" | "MulOp" | "DivOp" | "DerefOp" | "AddressOp" | "NotOp" => {
            match parser.get_data_from_parent_scope("op") {
                Some(oplist) => match oplist {
                    ASTNode::OperList(mut list) => {
                        list.push(string_to_oper(pos, name));
                        ASTNode::OperList(list)
                    }
                    _ => {
                        let _ = writeln!(stderr(), "\"op\" is not an operator list.");
                        ASTNode::None
                    }
                },
                None => ASTNode::OperList(vec![string_to_oper(pos, name)]),
            }
        }
        "True" => ASTNode::Exp(ASTExp_::bool_exp(pos, true, keywords)),
        "False" => ASTNode::Exp(ASTExp_::bool_exp(pos, false, keywords)),
        "Exp" => {
            let new_exp = extract_exp_data(pos, parser.get_data("exp"), "exp", name);
            match parser.get_data_from_parent_scope("exp") {
                Some(node) => match node {
                    ASTNode::Exp(exp) => ASTNode::ExpList(vec![exp, new_exp]),
                    ASTNode::ExpList(mut explist) => {
                        explist.push(new_exp);
                        ASTNode::ExpList(explist)
                    }
                    _ => ASTNode::ExpList(vec![new_exp]),
                },
                None => ASTNode::ExpList(vec![new_exp]),
            }
        }
        "UnaryOpExp" | "ProdExp" | "SumExp" | "CompOpExp" | "BinOpExp" => {
            let handled_exp = match parser.get_data("op") {
                Some(node) => {
                    let oplist = node.get_operlist_data(pos, "op", name);
                    if name == "UnaryOpExp" {
                        ASTExp_::unaryop_exp(
                            pos,
                            oplist,
                            extract_exp_data(pos, parser.get_data("exp"), "exp", name),
                            keywords,
                        )
                    } else {
                        ASTExp_::binop_exp(
                            pos,
                            oplist,
                            extract_explist_data(pos, parser.get_data("exp"), "exp", name),
                            keywords,
                        )
                    }
                }
                None => extract_exp_data(pos, parser.get_data("exp"), "exp", name),
            };
            match parser.get_data_from_parent_scope("exp") {
                Some(node) => match node {
                    ASTNode::ExpList(mut list) => {
                        list.push(handled_exp);
                        ASTNode::ExpList(list)
                    }
                    ASTNode::Exp(exp) => ASTNode::ExpList(vec![exp, handled_exp]),
                    _ => {
                        let _ = writeln!(stderr(), "exp is not an explist. {:?}", node);
                        ASTNode::None
                    }
                },
                None => ASTNode::ExpList(vec![handled_exp]),
            }
        }
        "ExpList" => match parser.get_data("exp") {
            Some(explist) => explist,
            None => ASTNode::ExpList(vec![]),
        },
        "Field" => {
            let new_node = Field_::new(
                pos,
                extract_var_data(pos, parser.get_data("Var"), "Var", name),
                extract_ty_data(pos, parser.get_data("Type"), "Type", name),
                keywords,
            );
            match parser.get_data_from_parent_scope("Field") {
                Some(node) => match node {
                    ASTNode::FieldList(mut list) => {
                        list.push(new_node);
                        ASTNode::FieldList(list)
                    }
                    _ => {
                        let _ = writeln!(stderr(), "Field is not a Fieldlist");
                        ASTNode::None
                    }
                },
                None => ASTNode::FieldList(vec![new_node]),
            }
        }
        "FieldList" => match parser.get_data("Field") {
            Some(fieldlist) => fieldlist,
            None => ASTNode::FieldList(vec![]),
        },
        "PublicMembers" | "PrivateMembers" => {
            let new_list = ClassMemberList::new_list(
                extract_declist_data(pos, parser.get_data("DecList"), "DecList", name),
                if name == "PublicMembers" {
                    MemberSpecifier::Public
                } else {
                    MemberSpecifier::Private
                },
            );
            match parser.get_data_from_parent_scope("members") {
                Some(node) => match node {
                    ASTNode::ClassMemberList(mut list) => {
                        list.append_list(new_list);
                        ASTNode::ClassMemberList(list)
                    }
                    _ => {
                        let _ = writeln!(stderr(), "members is not a class member list.");
                        ASTNode::ClassMemberList(new_list)
                    }
                },
                None => ASTNode::ClassMemberList(new_list),
            }
        }
        "ClassMemberList" => ASTNode::ClassMemberList(extract_classmembers_data(
            pos,
            parser.get_data("members"),
            "members",
            name,
        )),
        "ClassDec" => ASTNode::Dec(Dec_::class_dec(
            pos,
            extract_id_data(pos, parser.get_data("ID"), "ID", name),
            extract_classmembers_data(
                pos,
                parser.get_data("ClassMemberList"),
                "ClassMemberList",
                name,
            ),
            extract_idlist_data(pos, parser.get_data("IDList"), "IDList", name),
            keywords,
        )),
        "OperDec" | "FuncDec" | "JsImportDec" => {
            let id = extract_id_data(pos, parser.get_data("ID"), "ID", name);
            let params = extract_fieldlist_data(pos, parser.get_data("params"), "params", name);
            let result = extract_fieldlist_data(pos, parser.get_data("result"), "result", name);

            if name == "OperDec" || name == "FuncDec" {
                let body = Stm_::compound_stm(
                    pos,
                    extract_stmlist_data(pos, parser.get_data("StmList"), "StmList", name),
                    Vec::new(),
                );
                if name == "OperDec" {
                    ASTNode::Dec(Dec_::oper_dec(pos, id, params, result, body, keywords))
                } else {
                    ASTNode::Dec(Dec_::func_dec(pos, id, params, result, body, keywords))
                }
            } else {
                let module = extract_string_data(pos, parser.get_data("module"), "module", name);
                let name = extract_string_data(pos, parser.get_data("name"), "name", name);
                ASTNode::Dec(Dec_::js_import_dec(
                    pos, id, params, result, module, name, keywords,
                ))
            }
        }
        "JsExportDec" => ASTNode::Dec(Dec_::js_export_dec(
            pos,
            extract_id_data(pos, parser.get_data("ID"), "ID", name),
            extract_string_data(pos, parser.get_data("String"), "String", name),
            keywords,
        )),
        "TemplateDec" => ASTNode::Dec(Dec_::template_dec(
            pos,
            extract_dec_data(pos, parser.get_data("Dec"), "Dec", name),
            extract_idlist_data(pos, parser.get_data("IDList"), "IDList", name),
            keywords,
        )),
        "VarDecInit" | "VarDecNoInit" => {
            let var = extract_var_data(pos, parser.get_data("Var"), "Var", name);
            let ty = extract_ty_data(pos, parser.get_data("Type"), "Type", name);
            if name == "VarDecInit" {
                ASTNode::Dec(Dec_::var_dec(
                    pos,
                    var,
                    ty,
                    extract_exp_data(pos, parser.get_data("Exp"), "Exp", name),
                    keywords,
                ))
            } else {
                ASTNode::Dec(Dec_::var_dec(
                    pos,
                    var,
                    ty,
                    ASTExp_::none_exp(pos),
                    keywords,
                ))
            }
        }
        "VarDec" => ASTNode::Dec(extract_dec_data(
            pos,
            parser.get_data("vardec"),
            "vardec",
            name,
        )),
        "Dec" => {
            let new_dec = extract_dec_data(pos, parser.get_data("dec"), "dec", name);
            match parser.get_data_from_parent_scope("Dec") {
                Some(dec) => match dec {
                    ASTNode::DecList(mut list) => {
                        list.push(new_dec);
                        ASTNode::DecList(list)
                    }
                    _ => {
                        let _ = writeln!(stderr(), "dec is not a declaration list.");
                        ASTNode::DecList(vec![new_dec])
                    }
                },
                None => ASTNode::DecList(vec![new_dec]),
            }
        }
        "DecList" => ASTNode::DecList(extract_declist_data(
            pos,
            parser.get_data("Dec"),
            "Dec",
            name,
        )),
        "Start" => ASTNode::DecList(extract_declist_data(
            pos,
            parser.get_data("DecList"),
            "DecList",
            name,
        )),
        _ => {
            let _ = writeln!(stderr(), "What is this token: {name}");
            ASTNode::None
        }
    }
}
