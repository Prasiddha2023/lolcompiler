
mod lexer;

use lexer::Lexer;
use std::env;
use std::fs;
use std::process;

pub trait SyntaxAnalyzer {
    fn parse_lolcode(&mut self) -> Result<(), String>;
    fn parse_head(&mut self) -> Result<(), String>;
    fn parse_title(&mut self) -> Result<(), String>;
    fn parse_comment(&mut self) -> Result<(), String>;
    fn parse_body(&mut self) -> Result<(), String>;
    fn parse_paragraph(&mut self) -> Result<(), String>;
    fn parse_inner_paragraph(&mut self) -> Result<(), String>;
    fn parse_inner_text(&mut self) -> Result<(), String>;
    fn parse_variable_define(&mut self) -> Result<(), String>;
    fn parse_variable_use(&mut self) -> Result<(), String>;
    fn parse_bold(&mut self) -> Result<(), String>;
    fn parse_list(&mut self) -> Result<(), String>;
    fn parse_list_items(&mut self) -> Result<(), String>;
    fn parse_inner_list(&mut self) -> Result<(), String>;
    fn parse_link(&mut self) -> Result<(), String>;
    fn parse_newline(&mut self) -> Result<(), String>;
    fn parse_text(&mut self) -> Result<(), String>;
}

#[derive(Clone)]
pub struct Node {
    pub kind: String,
    pub token: Option<String>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(kind: &str) -> Self {
        Node {
            kind: kind.to_string(),
            token: None,
            children: Vec::new(),
        }
    }

    pub fn print(&self, depth: usize) {
        let indent = "  ".repeat(depth);

        match &self.token {
            Some(token) => println!("{}{} '{}'", indent, self.kind, token),
            None => println!("{}{}", indent, self.kind),
        }

        for child in &self.children {
            child.print(depth + 1);
        }
    }
}

pub struct Parser {
    tokens: Vec<String>,
    pos: usize,
    stack: Vec<Node>,
    pub tree: Option<Node>,
}

impl Parser {
    pub fn new(tokens: Vec<String>) -> Self {
        Parser {
            tokens,
            pos: 0,
            stack: Vec::new(),
            tree: None,
        }
    }

    fn enter(&mut self, kind: &str) {
        self.stack.push(Node::new(kind));
    }

    fn leave(&mut self) {
        if let Some(node) = self.stack.pop() {
            if let Some(parent) = self.stack.last_mut() {
                parent.children.push(node);
            } else {
                self.tree = Some(node);
            }
        }
    }

    fn leaf(&mut self, kind: &str, token: String) {
        let node = Node {
            kind: kind.to_string(),
            token: Some(token),
            children: Vec::new(),
        };

        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(node);
        }
    }

    fn peek_tag(&self) -> Option<(String, usize)> {
        let token = self.tokens.get(self.pos)?;

        if !token.starts_with('#') {
            return None;
        }

        let upper = token.to_uppercase();

        if upper == "#MAEK" || upper == "#GIMMEH" {
            if let Some(next) = self.tokens.get(self.pos + 1) {
                return Some((
                    format!("{} {}", upper, next.to_uppercase()),
                    2,
                ));
            }
        }

        Some((upper, 1))
    }

    fn at(&self, tag: &str) -> bool {
        match self.peek_tag() {
            Some((current, _)) => current == tag,
            None => false,
        }
    }

    fn at_text(&self) -> bool {
        match self.tokens.get(self.pos) {
            Some(token) => !token.starts_with('#'),
            None => false,
        }
    }

    fn describe(&self) -> String {
        match self.tokens.get(self.pos) {
            Some(token) => token.clone(),
            None => "end of input".to_string(),
        }
    }

    fn error(&self, message: &str) -> String {
        format!(
            "Syntax error at token {}: {} (found {})",
            self.pos + 1,
            message,
            self.describe()
        )
    }

    fn expect(&mut self, tag: &str) -> Result<(), String> {
        if !self.at(tag) {
            return Err(self.error(&format!("expected {}", tag)));
        }

        let count = self.peek_tag().unwrap().1;

        self.pos += count;
        self.leaf(tag, tag.to_string());

        Ok(())
    }

    fn parse_optional_variable(&mut self) -> Result<(), String> {
        if self.at("#IHAZ") {
            self.parse_variable_define()?;
        }

        Ok(())
    }

    fn parse_comments(&mut self) -> Result<(), String> {
        while self.at("#OBTW") {
            self.parse_comment()?;
        }

        Ok(())
    }
}

impl SyntaxAnalyzer for Parser {
    fn parse_lolcode(&mut self) -> Result<(), String> {
        self.enter("lolcode");

        self.expect("#HAI")?;

        self.parse_optional_variable()?;
        self.parse_comments()?;

        if self.at("#MAEK HEAD") {
            self.parse_head()?;
        }

        self.parse_body()?;

        self.expect("#KBYE")?;

        if self.pos != self.tokens.len() {
            return Err(self.error("unexpected tokens after #KBYE"));
        }

        self.leave();

        Ok(())
    }

    fn parse_head(&mut self) -> Result<(), String> {
        self.enter("head");

        self.expect("#MAEK HEAD")?;
        self.parse_optional_variable()?;
        self.parse_comments()?;

        if self.at("#GIMMEH TITLE") {
            self.parse_title()?;
        }

        self.parse_comments()?;
        self.expect("#MKAY")?;

        self.leave();

        Ok(())
    }

    fn parse_title(&mut self) -> Result<(), String> {
        self.enter("title");

        self.expect("#GIMMEH TITLE")?;
        self.parse_optional_variable()?;
        self.parse_inner_text()?;
        self.expect("#OIC")?;

        self.leave();

        Ok(())
    }

    fn parse_comment(&mut self) -> Result<(), String> {
        self.enter("comment");

        self.expect("#OBTW")?;

        while !self.at("#TLDR") {
            if self.pos >= self.tokens.len() {
                return Err(self.error("missing #TLDR"));
            }

            let token = self.tokens[self.pos].clone();
            self.leaf("comment_text", token);
            self.pos += 1;
        }

        self.expect("#TLDR")?;

        self.leave();

        Ok(())
    }

    fn parse_body(&mut self) -> Result<(), String> {
        self.enter("body");

        while self.pos < self.tokens.len() && !self.at("#KBYE") {
            if self.at("#OBTW") {
                self.parse_comment()?;
            } else if self.at("#MAEK PARAGRAF") {
                self.parse_paragraph()?;
            } else if self.at("#MAEK LIST") {
                self.parse_list()?;
            } else if self.at("#GIMMEH BOLD") {
                self.parse_bold()?;
            } else if self.at("#GIMMEH LINX") {
                self.parse_link()?;
            } else if self.at("#GIMMEH VAR") {
                self.parse_variable_use()?;
            } else if self.at("#NEWLINE") {
                self.parse_newline()?;
            } else if self.at_text() {
                self.parse_text()?;
            } else {
                return Err(self.error("unexpected annotation in body"));
            }
        }

        self.leave();

        Ok(())
    }

    fn parse_paragraph(&mut self) -> Result<(), String> {
        self.enter("paragraph");

        self.expect("#MAEK PARAGRAF")?;
        self.parse_optional_variable()?;
        self.parse_inner_paragraph()?;
        self.expect("#MKAY")?;

        self.leave();

        Ok(())
    }

    fn parse_inner_paragraph(&mut self) -> Result<(), String> {
        self.enter("inner_paragraph");

        while self.pos < self.tokens.len() && !self.at("#MKAY") {
            if self.at("#OBTW") {
                self.parse_comment()?;
            } else if self.at("#GIMMEH BOLD") {
                self.parse_bold()?;
            } else if self.at("#GIMMEH VAR") {
                self.parse_variable_use()?;
            } else if self.at("#GIMMEH LINX") {
                self.parse_link()?;
            } else if self.at("#MAEK LIST") {
                self.parse_list()?;
            } else if self.at("#NEWLINE") {
                self.parse_newline()?;
            } else if self.at_text() {
                self.parse_text()?;
            } else {
                return Err(self.error("unexpected annotation in paragraph"));
            }
        }

        self.leave();

        Ok(())
    }

    fn parse_inner_text(&mut self) -> Result<(), String> {
        self.enter("inner_text");

        while self.pos < self.tokens.len() && !self.at("#OIC") {
            if self.at("#GIMMEH VAR") {
                self.parse_variable_use()?;
            } else if self.at_text() {
                self.parse_text()?;
            } else {
                return Err(self.error("unexpected annotation in text"));
            }
        }

        self.leave();

        Ok(())
    }

    fn parse_variable_define(&mut self) -> Result<(), String> {
        self.enter("variable_define");

        self.expect("#IHAZ")?;

        if !self.at_text() {
            return Err(self.error("expected variable name"));
        }

        let name = self.tokens[self.pos].clone();
        self.leaf("variable_name", name);
        self.pos += 1;

        self.expect("#ITIZ")?;

        if !self.at_text() {
            return Err(self.error("expected variable value"));
        }

        while self.at_text() {
            self.parse_text()?;
        }

        self.expect("#MKAY")?;

        self.leave();

        Ok(())
    }

    fn parse_variable_use(&mut self) -> Result<(), String> {
        self.enter("variable_use");

        self.expect("#GIMMEH VAR")?;

        if !self.at_text() {
            return Err(self.error("expected variable name"));
        }

        let name = self.tokens[self.pos].clone();
        self.leaf("variable_name", name);
        self.pos += 1;

        self.expect("#OIC")?;

        self.leave();

        Ok(())
    }

    fn parse_bold(&mut self) -> Result<(), String> {
        self.enter("bold");

        self.expect("#GIMMEH BOLD")?;
        self.parse_optional_variable()?;
        self.parse_inner_text()?;
        self.expect("#OIC")?;

        self.leave();

        Ok(())
    }

    fn parse_list(&mut self) -> Result<(), String> {
        self.enter("list");

        self.expect("#MAEK LIST")?;
        self.parse_optional_variable()?;
        self.parse_comments()?;
        self.parse_list_items()?;
        self.parse_comments()?;
        self.expect("#MKAY")?;

        self.leave();

        Ok(())
    }

    fn parse_list_items(&mut self) -> Result<(), String> {
        self.enter("list_items");

        if !self.at("#GIMMEH ITEM") {
            return Err(self.error("expected at least one list item"));
        }

        while self.at("#GIMMEH ITEM") {
            self.parse_inner_list()?;
        }

        self.leave();

        Ok(())
    }

    fn parse_inner_list(&mut self) -> Result<(), String> {
        self.enter("inner_list");

        self.expect("#GIMMEH ITEM")?;
        self.parse_optional_variable()?;

        while self.pos < self.tokens.len() && !self.at("#OIC") {
            if self.at("#GIMMEH VAR") {
                self.parse_variable_use()?;
            } else if self.at("#GIMMEH BOLD") {
                self.parse_bold()?;
            } else if self.at_text() {
                self.parse_text()?;
            } else {
                return Err(self.error("unexpected annotation in list item"));
            }
        }

        self.expect("#OIC")?;
        self.parse_comments()?;

        self.leave();

        Ok(())
    }

    fn parse_link(&mut self) -> Result<(), String> {
        self.enter("link");

        self.expect("#GIMMEH LINX")?;

        if !self.at_text() {
            return Err(self.error("expected link address"));
        }

        while self.at_text() {
            self.parse_text()?;
        }

        self.expect("#OIC")?;

        self.leave();

        Ok(())
    }

    fn parse_newline(&mut self) -> Result<(), String> {
        self.enter("newline");

        self.expect("#NEWLINE")?;

        self.leave();

        Ok(())
    }

    fn parse_text(&mut self) -> Result<(), String> {
        if !self.at_text() {
            return Err(self.error("expected text"));
        }

        let token = self.tokens[self.pos].clone();

        self.leaf("text", token);
        self.pos += 1;

        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: lolcompiler <file.lol>");
        process::exit(1);
    }

    let filename = &args[1];

    if !filename.to_lowercase().ends_with(".lol") {
        eprintln!("Error: input file must have a .lol extension");
        process::exit(1);
    }

    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source);

    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);

    match parser.parse_lolcode() {
        Ok(()) => {
            println!("Syntax analysis successful.");

            if let Some(tree) = &parser.tree {
                tree.print(0);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
