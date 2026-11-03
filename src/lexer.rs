pub trait LexicalAnalyzer {
    fn get_char(&mut self) -> char;
    fn add_char(&mut self, c: char);
    fn lookup(&self, s: &str) -> bool;
}

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    current: String,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
            current: String::new(),
        }
    }

    fn has_next(&self) -> bool {
        self.pos < self.source.len()
    }

    fn peek_char(&self) -> Option<char> {
        self.source.get(self.pos).copied()
    }

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
    fn get_char(&mut self) -> char {
        if self.has_next() {
            let c = self.source[self.pos];
            self.pos += 1;
            c
        } else {
            panic!("End of input");
        }
    }

    fn add_char(&mut self, c: char) {
        self.current.push(c);
    }

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
