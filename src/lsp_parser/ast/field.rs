use super::{ty, var::Var};

pub type Field = Box<Field_>;
pub type FieldList = Vec<Field>;

#[derive(Clone, Debug)]
pub struct Field_ {
    pub pos: (usize, usize),
    pub data: FieldData,
    pub keywords: Vec<(usize, usize)>,
}

#[derive(Clone, Debug)]
pub enum FieldData {
    Field(Var, ty::Type),
    None,
}

impl Field_ {
    pub fn new(
        pos: (usize, usize),
        var: Var,
        ty: ty::Type,
        keywords: Vec<(usize, usize)>,
    ) -> Field {
        Box::new(Field_ {
            pos,
            data: FieldData::Field { 0: var, 1: ty },
            keywords,
        })
    }
    pub fn none(pos: (usize, usize)) -> Field {
        Box::new(Field_ {
            pos,
            data: FieldData::None,
            keywords: vec![],
        })
    }
}
