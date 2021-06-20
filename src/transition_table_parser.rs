use crate::flow::Flow;
use crate::row::{Row, RowKind};
use crate::token::Token;

use regex::Regex;


pub(crate) fn parseTransitionTable(tokens: &[Token]) -> Result<Vec<Row>,String>
{
    let parser = Parser::new();
    parser.parse(tokens)
}

struct Parser
{
    state: State,
    rowRegex: Regex,
    rows: Vec<Row>
}

impl Parser
{
    fn new() -> Self
    {
        Self{state: State::Initial, rowRegex: Regex::new(".*[rR]ow$").unwrap(), rows: vec![]}
    }

    fn parse(mut self, tokens: &[Token]) -> Result<Vec<Row>,String>
    {
        let firstRowIndex = match self.findFirstRow(tokens) {
            Some(index) => index,
            None => return Err("Rows were not found in the transition table.".into())
        };

        for token in &tokens[firstRowIndex..] {
            match self.parseToken(token) {
                Ok(flow) =>
                    match flow {
                        Flow::Continue => (),
                        Flow::Break => break
                    },
                Err(e) => return Err(e)
            }
        }

        Ok(self.rows)
    }

    fn findFirstRow(&self, tokens: &[Token]) -> Option<usize>
    {
        for (index, token) in tokens.iter().enumerate() {
            if let Token::Identifier(name) = token {
                if self.rowRegex.is_match(name) {
                    return Some(index);
                }
            }
        }
        None
    }

    fn parseToken(&mut self, token: &Token) -> Result<Flow,String>
    {
        match self.state {
            State::Initial => self.parseTokenInInitialState(token),
            State::ExpectRowTemplateStart => self.parseTokenInExpectRowTemplateStart(token),
            State::ExpectStartState => self.parseTokenInExpectStartState(token),
            State::ExpectCommaAfterStartState => self.parseTokenInExpectCommaAfterStartState(token),
            State::ExpectEvent => self.parseTokenInExpectEvent(token),
            State::ExpectCommaAfterEvent => self.parseTokenInExpectCommaAfterEvent(token),
            State::ExpectTargetState => self.parseTokenInExpectTargetState(token),
            State::AfterTargetState =>  self.parseTokenInAfterTargetState(token),
            State::ExpectAction => self.parseTokenInExpectAction(token),
            State::AfterAction => self.parseTokenInAfterAction(token),
            State::ExpectGuard => self.parseTokenInExpectGuard(token),
            State::ExpectRowEnd => self.parseTokenInExpectRowEnd(token),
            State::AfterRowEnd => self.parseTokenInAfterRowEnd(token)
        }
    }

    fn parseTokenInInitialState(&mut self, token: &Token) -> Result<Flow,String>
    {
        match token {
            Token::Identifier(name) => {
                if self.rowRegex.is_match(name) {
                    self.rows.push(Row::new(selectRowKind(name)));
                    self.state = State::ExpectRowTemplateStart;
                    Ok(Flow::Continue)
                } else {
                    Err(format!("Expected row identifier, got: {}", name))
                }
            },
            x => Err(format!("Expected identifier, got: {:?}", x))
        }
    }

    fn parseTokenInExpectRowTemplateStart(&mut self, token: &Token) -> Result<Flow,String>
    {
        match token {
            Token::TemplateStart => {
                self.state = State::ExpectStartState;
                Ok(Flow::Continue)
            },
            _ => Err(format!("Expected row template start, got: {:?}.", token))
        }
    }

    fn parseTokenInExpectStartState(&mut self, token: &Token) -> Result<Flow,String>
    {
        match token {
            Token::Identifier(name) => {
                self.getLastRow().start = name.clone();
                self.state = State::ExpectCommaAfterStartState;
                Ok(Flow::Continue)
            },
            _ => Err(format!("Expected start state, got: {:?}.", token))
        }
    }

    fn parseTokenInExpectCommaAfterStartState(&mut self, token: &Token) -> Result<Flow,String>
    {
        match token {
            Token::Comma => {
                self.state = State::ExpectEvent;
                Ok(Flow::Continue)
            },
            _ => Err(format!("Expected comma after start state, got: {:?}.", token))
        }
    }

    fn parseTokenInExpectEvent(&mut self, token: &Token) -> Result<Flow,String>
    {
        match token {
            Token::Identifier(name) => {
                self.getLastRow().event = name.clone();
                self.state = State::ExpectCommaAfterEvent;
                Ok(Flow::Continue)
            },
            _ => Err(format!("Expected event, got: {:?}.", token))
        }
    }

    fn parseTokenInExpectCommaAfterEvent(&mut self, token: &Token) -> Result<Flow,String>
    {
        match token {
            Token::Comma => {
                self.state = State::ExpectTargetState;
                Ok(Flow::Continue)
            },
            _ => Err(format!("Expected comma after event, got: {:?}.", token))
        }
    }

    fn parseTokenInExpectTargetState(&mut self, token: &Token) -> Result<Flow,String>
    {
        match token {
            Token::Identifier(name) => {
                self.getLastRow().target = name.clone();
                self.state = State::AfterTargetState;
                Ok(Flow::Continue)
            },
            _ => Err(format!("Expected target state, got: {:?}.", token))
        }
    }

    fn parseTokenInAfterTargetState(&mut self, token: &Token) -> Result<Flow,String>
    {
        match token {
            Token::Comma => {
                match self.getLastRow().kind {
                    RowKind::WithGuard => self.state = State::ExpectGuard,
                    RowKind::Other => self.state = State::ExpectAction
                }
                Ok(Flow::Continue)
            },
            Token::TemplateEnd => {
                self.state = State::AfterRowEnd;
                Ok(Flow::Continue)
            },
            _ => Err(format!("Expected comma or template end symbol after target state, got: {:?}.", token))
        }
    }

    fn parseTokenInExpectAction(&mut self, token: &Token) -> Result<Flow,String>
    {
        match token {
            Token::Identifier(name) => {
                self.getLastRow().action = name.clone();
                self.state = State::AfterAction;
                Ok(Flow::Continue)
            },
            _ => Err(format!("Expected action, got: {:?}.", token))
        }
    }

    fn parseTokenInAfterAction(&mut self, token: &Token) -> Result<Flow,String>
    {
        match token {
            Token::Comma => {
                self.state = State::ExpectGuard;
                Ok(Flow::Continue)
            },
            Token::TemplateEnd => {
                self.state = State::AfterRowEnd;
                Ok(Flow::Continue)
            },
            _ => Err(format!("Expected comma or template end symbol after action, got: {:?}.", token))
        }
    }

    fn parseTokenInExpectGuard(&mut self, token: &Token) -> Result<Flow,String>
    {
        match token {
            Token::Identifier(name) => {
                self.getLastRow().guard = name.clone();
                self.state = State::ExpectRowEnd;
                Ok(Flow::Continue)
            },
            _ => Err(format!("Expected guard, got: {:?}.", token))
        }
    }

    fn parseTokenInExpectRowEnd(&mut self, token: &Token) -> Result<Flow,String>
    {
        match token {
            Token::TemplateEnd => {
                self.state = State::AfterRowEnd;
                Ok(Flow::Continue)
            },
            _ => Err(format!("Expected template end symbol, got: {:?}.", token))
        }
    }

    fn parseTokenInAfterRowEnd(&mut self, token: &Token) -> Result<Flow,String>
    {
        match token {
            Token::TemplateEnd => Ok(Flow::Break),
            _ => Err(format!("Expected comma or template end symbol after row, got: {:?}.", token))
        }
    }

    fn getLastRow(&mut self) -> &mut Row
    {
        self.rows.last_mut().expect("Parser::rows should have contained elements after first row was found")
    }
}

fn selectRowKind(name: &str) -> RowKind
{
    match name {
        "g_row" => RowKind::WithGuard,
        _ => RowKind::Other
    }
}

#[allow(clippy::enum_variant_names)]
enum State
{
    Initial,
    ExpectRowTemplateStart,
    ExpectStartState,
    ExpectCommaAfterStartState,
    ExpectEvent,
    ExpectCommaAfterEvent,
    ExpectTargetState,
    AfterTargetState,
    ExpectAction,
    AfterAction,
    ExpectGuard,
    ExpectRowEnd,
    AfterRowEnd
}
