use crate::token::Token;

use std::iter::Peekable;
use std::slice::Iter;


pub(crate) struct RowSectionParser
{
    name: &'static str,
    state: State,
    templateDepth: u32,
    output: String
}

impl RowSectionParser
{
    pub(crate) fn new(name: &'static str) -> Self
    {
        Self{name, state: State::ExpectIdentifier, templateDepth: 0, output: String::new()}
    }

    pub(crate) fn parse(mut self, iterator: &mut Peekable<Iter<Token>>) -> Result<String, String>
    {
        loop {
            match iterator.peek() {
                Some(token) => {
                    match self.parseToken(token, iterator) {
                        Ok(Flow::Continue) => (),
                        Ok(Flow::Break) => break,
                        Err(e) => return Err(e)
                    }
                },
                None => return Err(format!("While parsing {}, tokens ended prematurely.", self.name))
            }
        }
        Ok(self.output)
    }

    fn parseToken(&mut self, token: &Token, iterator: &mut Peekable<Iter<Token>>) -> Result<Flow,String>
    {
        match self.state {
            State::ExpectIdentifier => self.parseInExpectIdentifier(token, iterator),
            State::AfterIdentifier => self.parseInAfterIdentifier(token, iterator),
            State::AfterTemplateStart => self.parseInAfterTemplateStart(token, iterator),
            State::AfterIdentifierInTemplate => self.parseInAfterIdentifierInTemplate(token, iterator),
            State::AfterInnerTemplateEnd => self.parseInAfterInnerTemplateEnd(token, iterator),
            State::ExpectIdentifierInTemplate => self.parseInExpectIdentifierInTemplate(token, iterator)
        }
    }

    fn parseInExpectIdentifier(&mut self, token: &Token, iterator: &mut Peekable<Iter<Token>>) -> Result<Flow, String>
    {
        match token {
            Token::Identifier(name) => {
                self.output.push_str(name);
                self.state = State::AfterIdentifier;
                iterator.next();
                Ok(Flow::Continue)
            },
            _ => Err(format!("Expected {}, got: {:?}.", self.name, token))
        }
    }

    fn parseInAfterIdentifier(&mut self, token: &Token, iterator: &mut Peekable<Iter<Token>>) -> Result<Flow, String>
    {
        match token {
            Token::TemplateStart => {
                self.output.push('<');
                self.templateDepth += 1;
                self.state = State::AfterTemplateStart;
                iterator.next();
                Ok(Flow::Continue)
            },
            _ => Ok(Flow::Break)
        }
    }

    fn parseInAfterTemplateStart(&mut self, token: &Token, iterator: &mut Peekable<Iter<Token>>) -> Result<Flow, String>
    {
        match token {
            Token::Identifier(name) => {
                self.output.push_str(name);
                self.state = State::AfterIdentifierInTemplate;
                iterator.next();
                Ok(Flow::Continue)
            },
            Token::TemplateEnd => {
                self.output.push('>');
                self.templateDepth -= 1;
                match self.templateDepth {
                    0 => {
                        iterator.next();
                        Ok(Flow::Break)
                    },
                    _ => {
                        self.state = State::AfterInnerTemplateEnd;
                        iterator.next();
                        Ok(Flow::Continue)
                    }
                }
            },
            _ => Err(format!("Expected an identifier or a template end, got: {:?}.", token))
        }
    }

    fn parseInAfterIdentifierInTemplate(&mut self, token: &Token, iterator: &mut Peekable<Iter<Token>>) -> Result<Flow, String>
    {
        match token {
            Token::Comma => {
                self.output.push_str(", ");
                self.state = State::ExpectIdentifierInTemplate;
                iterator.next();
                Ok(Flow::Continue)
            },
            Token::TemplateStart => {
                self.output.push('<');
                self.templateDepth += 1;
                self.state = State::AfterTemplateStart;
                iterator.next();
                Ok(Flow::Continue)
            },
            Token::TemplateEnd => {
                self.output.push('>');
                self.templateDepth -= 1;
                iterator.next();
                match self.templateDepth {
                    0 => Ok(Flow::Break),
                    _ => {
                        self.state = State::AfterInnerTemplateEnd;
                        Ok(Flow::Continue)
                    }
                }
            }
            _ => Err(format!("Expected a comma, template start or template end, got: {:?}.", token))
        }
    }

    fn parseInAfterInnerTemplateEnd(&mut self, token: &Token, iterator: &mut Peekable<Iter<Token>>) -> Result<Flow, String>
    {
        match token {
            Token::Comma => unimplemented!(),
            Token::TemplateEnd => {
                self.output.push('>');
                self.templateDepth -= 1;
                iterator.next();
                match self.templateDepth {
                    0 => Ok(Flow::Break),
                    _ => Ok(Flow::Continue)
                }
            },
            _ => Err(format!("Expected a comma or a template end, got: {:?}.", token))
        }
    }

    fn parseInExpectIdentifierInTemplate(&mut self, token: &Token, iterator: &mut Peekable<Iter<Token>>) -> Result<Flow, String>
    {
        match token {
            Token::Identifier(name) => {
                self.output.push_str(name);
                self.state = State::AfterIdentifierInTemplate;
                iterator.next();
                Ok(Flow::Continue)
            },
            _ => Err(format!("Expected an identifier, got: {:?}.", token))
        }
    }
}

enum State
{
    ExpectIdentifier,
    AfterIdentifier,
    AfterTemplateStart,
    AfterIdentifierInTemplate,
    AfterInnerTemplateEnd,
    ExpectIdentifierInTemplate
}

enum Flow
{
    Continue,
    Break
}
