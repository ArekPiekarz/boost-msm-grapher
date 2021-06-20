use crate::character_reader::CharacterReader;
use crate::flow::Flow;
use crate::token::Token;


pub(crate) fn lexTransitionTable(characterReader: CharacterReader) -> Vec<Token>
{
    let lexer = Lexer::new(characterReader);
    lexer.lex()
}

struct Lexer<'a>
{
    characterReader: CharacterReader<'a>,
    state: State,
    currentToken: String,
    tokens: Vec<Token>
}

impl<'a> Lexer<'a>
{
    fn new(characterReader: CharacterReader<'a>) -> Self
    {
        Self{characterReader, state: State::Empty, currentToken: String::new(), tokens: vec![]}
    }

    fn lex(mut self) -> Vec<Token>
    {
        while let Some(character) = self.characterReader.next() {
            match self.lexCharacter(character) {
                Flow::Continue => (),
                Flow::Break => break
            }
        }
        self.tokens
    }

    fn lexCharacter(&mut self, character: char) -> Flow
    {
        match &self.state {
            State::Empty => self.lexCharacterInEmptyState(character),
            State::Collecting => self.lexCharacterInCollectingState(character),
            State::Comment => self.lexCharacterInCommentState(character)
        }
    }

    fn lexCharacterInEmptyState(&mut self, character: char) -> Flow
    {
        match character {
            x if x.is_whitespace() => Flow::Continue,
            '/' => self.lexForwardSlashInEmptyState(),
            '<' | '>' | ',' | '{' | '}' => self.lexSymbolInEmptyState(character),
            ';' => self.lexSemicolonInEmptyState(),
            _ => self.lexIdentifierInEmptyState(character)
        }
    }

    fn lexForwardSlashInEmptyState(&mut self) -> Flow
    {
        match self.characterReader.peek() {
            Some('/') => {
                self.characterReader.next();
                self.state = State::Comment;
            },
            _ => self.tokens.push(Token::new("/"))
        }
        Flow::Continue
    }

    fn lexSymbolInEmptyState(&mut self, symbol: char) -> Flow
    {
        self.tokens.push(Token::new(&symbol.to_string()));
        Flow::Continue
    }

    fn lexSemicolonInEmptyState(&mut self) -> Flow
    {
        self.tokens.push(Token::new(";"));
        Flow::Break
    }

    fn lexIdentifierInEmptyState(&mut self, character: char) -> Flow
    {
        self.currentToken.push(character);
        self.state = State::Collecting;
        Flow::Continue
    }

    fn lexCharacterInCollectingState(&mut self, character: char) -> Flow
    {
        match character {
            x if x.is_whitespace() => {
                self.tokens.push(Token::new(&self.currentToken));
                self.currentToken.clear();
                self.state = State::Empty;
            },
            '<' | '>' | ',' => {
                self.tokens.push(Token::new(&self.currentToken));
                self.tokens.push(Token::new(&character.to_string()));
                self.currentToken.clear();
                self.state = State::Empty;
            },
            _ => {
                self.currentToken.push(character);
            }
        }
        Flow::Continue
    }

    fn lexCharacterInCommentState(&mut self, character: char) -> Flow
    {
        if character == '\n' {
            self.state = State::Empty
        }
        Flow::Continue
    }
}

enum State
{
    Empty,
    Collecting,
    Comment
}
