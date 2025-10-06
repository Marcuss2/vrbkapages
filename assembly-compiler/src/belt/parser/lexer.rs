use logos::Logos;

#[derive(Logos, Clone, PartialEq, Debug)]
#[logos(skip r"[ \t]+")] // Whitespaces
#[logos(skip r"(/\*([^*]|\*+[^*/])*\*+/)")] // /* */ Comment
#[regex(r"//[^\n]*\n")] // // Comment
pub enum Token<'a> {
    Error,

    #[regex(
        r"[+-]?((0[oO][0-7]+)|(0[xX][0-9A-Fa-f]+)|(0[bB][01]+|[0-9]+))",
        priority = 10
    )]
    Number(&'a str),

    #[regex(r"[a-zA-Z][a-zA-Z0-9_-]*")]
    Text(&'a str),

    #[regex(r"b((1[0-5])|[0-9])", priority = 10)]
    Register(&'a str),

    #[regex(r"\.[a-zA-Z_-]+", priority = 10)]
    Directive(&'a str),

    #[regex(r"[a-zA-Z][a-zA-Z0-9_-]*:", priority = 10)]
    Label(&'a str),

    #[regex("\n", priority = 10)]
    NewLine,
}

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Error => write!(f, "Error"),
            Token::Number(s) => write!(f, "Number({})", s),
            Token::Text(s) => write!(f, "Text({})", s),
            Token::Register(s) => write!(f, "Register({})", s),
            Token::Directive(s) => write!(f, "Directive({})", s),
            Token::Label(s) => write!(f, "Label({})", s),
            Token::NewLine => write!(f, "NewLine"),
        }
    }
}
