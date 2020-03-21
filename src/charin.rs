use console::Term;

// --------------------------------------------------------------
// Get character-by-character terminal input

pub struct CharIn {
    term: Term,
}

impl Iterator for CharIn {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.term.read_char().ok()
    }
}

impl CharIn {
    pub fn new() -> CharIn {
        CharIn {
            term: Term::stdout(),
        }
    }
}
