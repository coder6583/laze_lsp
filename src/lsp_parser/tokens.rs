pub type TokenList = Vec<Token>;

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

pub enum TokenType {
    Namespace,
    Type,
    Class,
    Enum,
    Interface,
    Struct,
    TypeParameter,
    Parameter,
    Variable,
    Property,
    EnumMember,
    Event,
    Function,
    Method,
    Macro,
    Keyword,
    Modifier,
    Comment,
    String,
    Number,
    Regexp,
    Operator,
}

pub enum TokenModifier {
    Declaration,
    Definition,
    Readonly,
    Static,
    Deprecated,
    Abstract,
    Async,
    Modification,
    Documentation,
    DefaultLibrary,
}
