use std::str::Chars;

pub(crate) struct Cursor<'a> {
    chars: Chars<'a>,
    consumed: usize,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Cursor {
            chars: input.chars(),
            consumed: 0,
        }
    }

    pub(crate) fn next(&mut self) -> Option<char> {
        self.consumed += 1;
        self.chars.next()
    }

    pub(crate) fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub(crate) fn get_next_location(&self) -> usize {
        self.consumed
    }

    pub(crate) fn len_left(&self) -> usize {
        self.chars.clone().count()
    }
}
