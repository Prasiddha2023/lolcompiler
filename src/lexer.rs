
/// Methods used by the lexer to read characters and check commands.
pub trait LexicalAnalyzer {
    // Gets the next character and moves forward in the input.
    fn get_char(&mut self) -> char;
    // Adds a character to the current token.
    fn add_char(&mut self, c: char);
    // Checks whether an annotation is one of the supported LOLCODE commands.
    fn lookup(&self, s: &str) -> bool;
}

/// Reads LOLCODE input and turns it into tokens.
pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    current: String,
}

impl Lexer {
    /// Stores the input as characters so the lexer can read it one character at a time.
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
            current: String::new(),
        }
    }

    // Checks whether there are characters left to read.
    fn has_next(&self) -> bool {
        self.pos < self.source.len()
    }

    // Looks at the next character without moving forward.
    fn peek_char(&self) -> Option<char> {
        self.source.get(self.pos).copied()
    }

    // Reads text until whitespace or another annotation starts.
    fn read_word(&mut self) -> String {
        self.current.clear();

        while let Some(c) = self.peek_char() {
            if c.is_whitespace() || c == '#' {
                break;
            }

            let c = self.get_char();
            self.add_char(c);
        }

        let word = self.current.clone();
        self.current.clear();
        word
    }

    // Reads a command starting with #. Some commands need a second word.
    fn read_annotation(&mut self) -> Result<String, String> {
        self.current.clear();
        let first_char = self.get_char();
        self.add_char(first_char);

        while let Some(c) = self.peek_char() {
            if c.is_whitespace() || c == '#' {
                break;
            }

            let c = self.get_char();
            self.add_char(c);
        }

        let first = self.current.to_uppercase();
        self.current.clear();

        if !self.lookup(&first) {
            return Err(format!("Lexical error: invalid annotation {}", first));
        }

        if first == "#MAEK" || first == "#GIMMEH" {
            while let Some(c) = self.peek_char() {
                if !c.is_whitespace() {
                    break;
                }
                self.get_char();
            }

            if !self.has_next() || self.peek_char() == Some('#') {
                return Err(format!("Lexical error: incomplete annotation {}", first));
            }

            let second = self.read_word().to_uppercase();
            let annotation = format!("{} {}", first, second);

            if !self.lookup(&annotation) {
                return Err(format!("Lexical error: invalid annotation {}", annotation));
            }

            Ok(annotation)
        } else {
            Ok(first)
        }
    }

    /// Skips whitespace and separates the input into text and annotation tokens.
    pub fn tokenize(&mut self) -> Result<Vec<String>, String> {
        let mut tokens = Vec::new();

        while self.has_next() {
            let c = self.peek_char().unwrap();

            if c.is_whitespace() {
                self.get_char();
            } else if c == '#' {
                let annotation = self.read_annotation()?;
                tokens.push(annotation);
            } else {
                let word = self.read_word();
                if !word.is_empty() {
                    tokens.push(word);
                }
            }
        }

        Ok(tokens)
    }
}

impl LexicalAnalyzer for Lexer {
    // Gets the next character and moves forward in the input.
    fn get_char(&mut self) -> char {
        if self.has_next() {
            let c = self.source[self.pos];
            self.pos += 1;
            c
        } else {
            panic!("End of input");
        }
    }

    // Adds a character to the current token.
    fn add_char(&mut self, c: char) {
        self.current.push(c);
    }

    // Checks whether an annotation is one of the supported LOLCODE commands.
    fn lookup(&self, s: &str) -> bool {
        matches!(
            s.to_uppercase().as_str(),
            "#HAI"
                | "#KBYE"
                | "#OBTW"
                | "#TLDR"
                | "#MAEK"
                | "#GIMMEH"
                | "#MKAY"
                | "#OIC"
                | "#IHAZ"
                | "#ITIZ"
                | "#NEWLINE"
                | "#MAEK HEAD"
                | "#MAEK PARAGRAF"
                | "#MAEK LIST"
                | "#GIMMEH TITLE"
                | "#GIMMEH BOLD"
                | "#GIMMEH ITEM"
                | "#GIMMEH LINX"
                | "#GIMMEH VAR"
        )
    }
}
