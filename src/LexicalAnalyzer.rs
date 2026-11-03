
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

    pub fn tokenize(&mut self) -> Result<Vec<String>, String> {
        let mut tokens = Vec::new();

        while self.has_next() {
            let c = self.get_char();

            if c.is_whitespace() {
                if !self.current.is_empty() {
                    self.save_token(&mut tokens)?;
                }
            } else {
                self.add_char(c);
            }
        }

        if !self.current.is_empty() {
            self.save_token(&mut tokens)?;
        }

        let mut combined = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            let token = tokens[i].to_uppercase();

            if token == "#MAEK" || token == "#GIMMEH" {
                if i + 1 >= tokens.len() {
                    return Err(format!(
                        "Lexical error: incomplete annotation {}",
                        token
                    ));
                }

                let next = tokens[i + 1].to_uppercase();
                let tag = format!("{} {}", token, next);

                if !self.lookup(&tag) {
                    return Err(format!(
                        "Lexical error: invalid annotation {}",
                        tag
                    ));
                }

                combined.push(tag);
                i += 2;
            } else {
                combined.push(tokens[i].clone());
                i += 1;
            }
        }

        Ok(combined)
    }

    fn save_token(&mut self, tokens: &mut Vec<String>) -> Result<(), String> {
        let token = self.current.clone();
        self.current.clear();

        if token.starts_with('#') {
            if !self.lookup(&token) {
                return Err(format!(
                    "Lexical error: invalid annotation {}",
                    token
                ));
            }

            tokens.push(token.to_uppercase());
        } else {
            tokens.push(token);
        }

        Ok(())
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
