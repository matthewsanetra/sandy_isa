use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("%")]
    Percentage,
    #[token("#")]
    Hashtag,
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(i64),
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[token("add")]
    Add,
    #[token("sub")]
    Sub,
    #[token("mul")]
    Mul,
    #[token("div")]
    Div,
    #[token("mod")]
    Mod,
    #[token("neg")]
    Neg,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("not")]
    Not,
    #[token("xor")]
    Xor,
    #[token("load")]
    Load,
    #[token("call")]
    Call,
    #[token("jump")]
    Jump,
    #[token("jez")]
    Jez,
    #[token("jnz")]
    Jnz,
    #[token("jgz")]
    Jgz,
    #[token("jlz")]
    Jlz,
    #[token("ret")]
    Ret,
    #[token("gt")]
    Gt,
    #[token("gte")]
    Gte,
    #[token("lt")]
    Lt,
    #[token("lte")]
    Lte,
    #[token("eq")]
    Eq,
    #[token("neq")]
    Neq,
    #[token("push")]
    Push,
    #[token("pop")]
    Pop,
    #[token("halt")]
    Halt,

    #[error]
    #[regex(r"[ \t\r\n]+", logos::skip)]
    Error,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LexError(logos::Span);

pub(crate) struct Lexer<'a> {
    lexer: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(lines: &'a String) -> Self {
        Self {
            lexer: Token::lexer(lines),
        }
    }
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;
pub type SpannedLexErr = Spanned<Token, usize, LexError>;

impl Iterator for Lexer<'_> {
    type Item = SpannedLexErr;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.lexer.next()?;
        let span = self.lexer.span();

        if let Token::Error = t {
            return Some(Err(LexError(span)));
        } else {
            return Some(Ok((span.start, t, span.end)));
        }
    }
}
