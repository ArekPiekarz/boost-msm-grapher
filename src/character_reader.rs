pub(crate) struct CharacterReader<'a>
{
    text: &'a str,
    index: usize
}

impl<'a> CharacterReader<'a>
{
    pub(crate) fn new(text: &'a str) -> Self
    {
        Self{text, index: 0}
    }

    pub(crate) fn next(&mut self) -> Option<char>
    {
        let character = self.text.chars().nth(self.index);
        self.index += 1;
        character
    }

    pub(crate) fn peek(&mut self) -> Option<char>
    {
        self.text.chars().nth(self.index)
    }
}
