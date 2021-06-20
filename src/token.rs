#[derive(Debug)]
pub(crate) enum Token
{
    BlockStart,
    BlockEnd,
    Colon,
    Comma,
    Identifier(String),
    InstructionEnd,
    Keyword(String),
    TemplateStart,
    TemplateEnd
}

impl Token
{
    pub(crate) fn new(text: &str) -> Self
    {
        match text {
            "," => Token::Comma,
            ":" => Token::Colon,
            ";" => Token::InstructionEnd,
            "<" => Token::TemplateStart,
            ">" => Token::TemplateEnd,
            "struct" => Token::Keyword(text.into()),
            "{" => Token::BlockStart,
            "}" => Token::BlockEnd,
            _ => Token::Identifier(text.into())
        }
    }
}
