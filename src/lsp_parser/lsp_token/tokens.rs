pub type TokenList = Vec<Token>;

#[derive(Debug)]
pub struct Token {
    pub pos: (usize, usize),
    pub token_type: TokenType,
    pub modifier: TokenModifier,
}

impl Token {
    pub fn new(pos: (usize, usize), token_type: TokenType, modifier: TokenModifier) -> Self {
        Self {
            pos,
            token_type,
            modifier,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    Namespace = 0,
    Type = 1,
    Class = 2,
    Enum = 3,
    Interface = 4,
    Struct = 5,
    TypeParameter = 6,
    Parameter = 7,
    Variable = 8,
    Property = 9,
    EnumMember = 10,
    Event = 11,
    Function = 12,
    Method = 13,
    Macro = 14,
    Keyword = 15,
    Modifier = 16,
    Comment = 17,
    String = 18,
    Number = 19,
    Regexp = 20,
    Operator = 21,
}

impl TokenType {
    pub fn into_usize(self) -> usize {
        match self {
            TokenType::Namespace => 0,
            TokenType::Type => 1,
            TokenType::Class => 2,
            TokenType::Enum => 3,
            TokenType::Interface => 4,
            TokenType::Struct => 5,
            TokenType::TypeParameter => 6,
            TokenType::Parameter => 7,
            TokenType::Variable => 8,
            TokenType::Property => 9,
            TokenType::EnumMember => 10,
            TokenType::Event => 11,
            TokenType::Function => 12,
            TokenType::Method => 13,
            TokenType::Macro => 14,
            TokenType::Keyword => 15,
            TokenType::Modifier => 16,
            TokenType::Comment => 17,
            TokenType::String => 18,
            TokenType::Number => 19,
            TokenType::Regexp => 20,
            TokenType::Operator => 21,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenModifier {
    Declaration = 0,
    Definition = 1,
    Readonly = 2,
    Static = 3,
    Deprecated = 4,
    Abstract = 5,
    Async = 6,
    Modification = 7,
    Documentation = 8,
    DefaultLibrary = 9,
}

impl TokenModifier {
    pub fn into_usize(self) -> usize {
        match self {
            TokenModifier::Declaration => 0,
            TokenModifier::Definition => 1,
            TokenModifier::Readonly => 2,
            TokenModifier::Static => 3,
            TokenModifier::Deprecated => 4,
            TokenModifier::Abstract => 5,
            TokenModifier::Async => 6,
            TokenModifier::Modification => 7,
            TokenModifier::Documentation => 8,
            TokenModifier::DefaultLibrary => 9,
        }
    }
}
