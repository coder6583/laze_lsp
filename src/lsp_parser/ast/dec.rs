use super::{
    ast::{ASTString, ID},
    exp, field, stm, ty,
    var::Var,
};

pub type Dec = Box<Dec_>;
pub type DecList = Vec<Dec>;

#[derive(Clone, Debug)]
pub struct Dec_ {
    pub pos: (usize, usize),
    pub keywords: Vec<(usize, usize)>,
    pub data: DecData,
}

#[derive(Clone, Debug)]
pub enum DecData {
    Func(ID, field::FieldList, field::FieldList, stm::Stm),
    Oper(ID, field::FieldList, field::FieldList, stm::Stm),
    JsImport(ID, field::FieldList, field::FieldList, ASTString, ASTString),
    JsExport(ID, ASTString),

    Var(Var, ty::Type, exp::ASTExp),
    Class(ID, ClassMemberList, Vec<ID>),
    Template(Dec, Vec<ID>),
    None,
}

impl Dec_ {
    pub fn none_dec(pos: (usize, usize)) -> Dec {
        Box::new(Dec_ {
            pos,
            keywords: vec![],
            data: DecData::None,
        })
    }
    pub fn func_dec(
        pos: (usize, usize),
        name: ID,
        params: field::FieldList,
        result: field::FieldList,
        body: stm::Stm,
        keywords: Vec<(usize, usize)>,
    ) -> Dec {
        Box::new(Dec_ {
            pos,
            keywords,
            data: DecData::Func(name, params, result, body),
        })
    }
    pub fn oper_dec(
        pos: (usize, usize),
        op: ID,
        params: field::FieldList,
        result: field::FieldList,
        body: stm::Stm,
        keywords: Vec<(usize, usize)>,
    ) -> Dec {
        Box::new(Dec_ {
            pos,
            keywords,
            data: DecData::Oper(op, params, result, body),
        })
    }
    pub fn js_import_dec(
        pos: (usize, usize),
        name: ID,
        params: field::FieldList,
        result: field::FieldList,
        module: ASTString,
        id: ASTString,
        keywords: Vec<(usize, usize)>,
    ) -> Dec {
        Box::new(Dec_ {
            pos,
            keywords,
            data: DecData::JsImport(name, params, result, module, id),
        })
    }
    pub fn js_export_dec(
        pos: (usize, usize),
        name: ID,
        export_name: ASTString,
        keywords: Vec<(usize, usize)>,
    ) -> Dec {
        Box::new(Dec_ {
            pos,
            keywords,
            data: DecData::JsExport(name, export_name),
        })
    }
    pub fn var_dec(
        pos: (usize, usize),
        var: Var,
        ty: ty::Type,
        init: exp::ASTExp,
        keywords: Vec<(usize, usize)>,
    ) -> Dec {
        Box::new(Dec_ {
            pos,
            keywords,
            data: DecData::Var(var, ty, init),
        })
    }
    pub fn class_dec(
        pos: (usize, usize),
        name: ID,
        class_members: ClassMemberList,
        inheritance: Vec<ID>,
        keywords: Vec<(usize, usize)>,
    ) -> Dec {
        Box::new(Dec_ {
            pos,
            keywords,
            data: DecData::Class(name, class_members, inheritance),
        })
    }
    pub fn template_dec(
        pos: (usize, usize),
        dec: Dec,
        ty_params: Vec<ID>,
        keywords: Vec<(usize, usize)>,
    ) -> Dec {
        Box::new(Dec_ {
            pos,
            keywords,
            data: DecData::Template(dec, ty_params),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MemberSpecifier {
    Public,
    Private,
}

#[derive(Clone, Debug)]
pub struct ClassMember_ {
    pub specifier: MemberSpecifier,
    pub dec: Dec,
}

pub type ClassMember = Box<ClassMember_>;
pub type ClassMemberList = Vec<ClassMember>;

pub trait MemberList {
    fn new_list(declist: Vec<Dec>, specifier: MemberSpecifier) -> ClassMemberList;
    fn append_list(&mut self, declist: ClassMemberList);
}

impl MemberList for Vec<ClassMember> {
    fn new_list(declist: Vec<Dec>, specifier: MemberSpecifier) -> ClassMemberList {
        let mut new_list: ClassMemberList = vec![];
        for dec in declist {
            new_list.push(Box::new(ClassMember_ { specifier, dec }));
        }
        new_list
    }
    fn append_list(&mut self, declist: ClassMemberList) {
        for dec in declist {
            self.push(dec);
        }
    }
}
