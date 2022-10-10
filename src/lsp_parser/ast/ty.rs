use super::{ast::ID, exp::ASTExp, field::FieldList};

pub type TypeList = Vec<Box<Type_>>;
pub type Type = Box<Type_>;

#[derive(Clone, Debug)]
pub struct Type_ {
    pub pos: (usize, usize),
    pub keywords: Vec<(usize, usize)>,
    pub data: TypeData,
}

#[derive(Clone, Debug)]
pub enum TypeData {
    Void,
    Int,
    Short,
    Char,
    Bool,
    Real,
    Name(ID),
    Array(Type, ASTExp),
    Pointer(Type),
    Template(ID, Vec<Type>),
    Func(FieldList, Type),
    None,
}

impl Type_ {
    pub fn none_type(pos: (usize, usize)) -> Type {
        Box::new(Type_ {
            pos,
            keywords: vec![],
            data: TypeData::None,
        })
    }
    pub fn void_type(pos: (usize, usize), keywords: Vec<(usize, usize)>) -> Type {
        Box::new(Type_ {
            pos,
            keywords,
            data: TypeData::Void,
        })
    }
    pub fn int_type(pos: (usize, usize), keywords: Vec<(usize, usize)>) -> Type {
        Box::new(Type_ {
            pos,
            keywords,
            data: TypeData::Int,
        })
    }
    pub fn short_type(pos: (usize, usize), keywords: Vec<(usize, usize)>) -> Type {
        Box::new(Type_ {
            pos,
            keywords,
            data: TypeData::Short,
        })
    }
    pub fn real_type(pos: (usize, usize), keywords: Vec<(usize, usize)>) -> Type {
        Box::new(Type_ {
            pos,
            keywords,
            data: TypeData::Real,
        })
    }
    pub fn char_type(pos: (usize, usize), keywords: Vec<(usize, usize)>) -> Type {
        Box::new(Type_ {
            pos,
            keywords,
            data: TypeData::Char,
        })
    }
    pub fn bool_type(pos: (usize, usize), keywords: Vec<(usize, usize)>) -> Type {
        Box::new(Type_ {
            pos,
            keywords,
            data: TypeData::Bool,
        })
    }
    pub fn name_type(pos: (usize, usize), name: ID, keywords: Vec<(usize, usize)>) -> Type {
        Box::new(Type_ {
            pos,
            keywords,
            data: TypeData::Name(name),
        })
    }
    pub fn array_type(
        pos: (usize, usize),
        ty: Type,
        size: ASTExp,
        keywords: Vec<(usize, usize)>,
    ) -> Type {
        Box::new(Type_ {
            pos,
            keywords,
            data: TypeData::Array(ty, size),
        })
    }
    pub fn pointer_type(pos: (usize, usize), ty: Type, keywords: Vec<(usize, usize)>) -> Type {
        Box::new(Type_ {
            pos,
            keywords,
            data: TypeData::Pointer(ty),
        })
    }
    pub fn template_type(
        pos: (usize, usize),
        name: ID,
        ty_params: Vec<Type>,
        keywords: Vec<(usize, usize)>,
    ) -> Type {
        Box::new(Type_ {
            pos,
            keywords,
            data: TypeData::Template(name, ty_params),
        })
    }
    pub fn func_type(
        pos: (usize, usize),
        params: FieldList,
        result: Type,
        keywords: Vec<(usize, usize)>,
    ) -> Type {
        Box::new(Type_ {
            pos,
            keywords,
            data: TypeData::Func(params, result),
        })
    }
}
