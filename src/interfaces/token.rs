#[repr(transparent)]
pub struct Token(pub usize);

impl From<usize> for Token {
    fn from(value: usize) -> Self {
        Token(value)
    }
}

impl From<Token> for usize {
    fn from(value: Token) -> Self {
        value.0
    }
}
