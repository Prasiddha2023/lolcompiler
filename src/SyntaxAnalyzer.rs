
/// Methods used to parse the different LOLCODE parts.
pub trait SyntaxAnalyzer {
    // Checks the full program, starting with #HAI and ending with #KBYE.
    fn parse_lolcode(&mut self) -> Result<(), String>;
    // Parses the head section and its optional title.
    fn parse_head(&mut self) -> Result<(), String>;
    // Parses the title text until #OIC.
    fn parse_title(&mut self) -> Result<(), String>;
    // Reads comment tokens until #TLDR.
    fn parse_comment(&mut self) -> Result<(), String>;
    // Keeps parsing body elements until #KBYE.
    fn parse_body(&mut self) -> Result<(), String>;
    // Parses one paragraph until its closing #MKAY.
    fn parse_paragraph(&mut self) -> Result<(), String>;
    // Allows text, variables, and other supported elements inside a paragraph.
    fn parse_inner_paragraph(&mut self) -> Result<(), String>;
    // Reads plain text and variable uses until #OIC.
    fn parse_inner_text(&mut self) -> Result<(), String>;
    // Reads a variable name and its value after #IHAZ and #ITIZ.
    fn parse_variable_define(&mut self) -> Result<(), String>;
    // Reads the variable name requested by #GIMMEH VAR.
    fn parse_variable_use(&mut self) -> Result<(), String>;
    // Parses the text inside a bold section.
    fn parse_bold(&mut self) -> Result<(), String>;
    // Parses a list and its items until #MKAY.
    fn parse_list(&mut self) -> Result<(), String>;
    // Requires at least one item and parses any following items.
    fn parse_list_items(&mut self) -> Result<(), String>;
    // Parses the contents of one list item.
    fn parse_inner_list(&mut self) -> Result<(), String>;
    // Reads the link address until #OIC.
    fn parse_link(&mut self) -> Result<(), String>;
    // Parses a newline command.
    fn parse_newline(&mut self) -> Result<(), String>;
    // Adds the next plain-text token to the tree.
    fn parse_text(&mut self) -> Result<(), String>;
}

#[derive(Clone)]
/// Stores a syntax tree element and its child nodes.
pub struct Node {
    pub kind: String,
    pub token: Option<String>,
    pub children: Vec<Node>,
}

impl Node {
    /// Creates a syntax tree node with no children.
    pub fn new(kind: &str) -> Self {
        Node {
            kind: kind.to_string(),
            token: None,
            children: Vec::new(),
        }
    }

    /// Prints each tree node with indentation to show its level.
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

/// Checks token order and builds the syntax tree.
pub struct Parser {
    tokens: Vec<String>,
    pos: usize,
    stack: Vec<Node>,
    pub tree: Option<Node>,
}

impl Parser {
    /// Creates a parser ready to read tokens.
    pub fn new(tokens: Vec<String>) -> Self {
        Parser {
            tokens,
            pos: 0,
            stack: Vec::new(),
            tree: None,
        }
    }

    // Starts a node and keeps it on the stack while its contents are parsed.
    fn enter(&mut self, kind: &str) {
        self.stack.push(Node::new(kind));
    }

    // Finishes the current node and attaches it to its parent.
    fn leave(&mut self) {
        if let Some(node) = self.stack.pop() {
            if let Some(parent) = self.stack.last_mut() {
                parent.children.push(node);
            } else {
                self.tree = Some(node);
            }
        }
    }

    // Adds a node that holds one token.
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

    // Looks ahead at the next annotation. Two-word commands count as one tag.
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

    // Checks whether the next annotation matches the expected tag.
    fn at(&self, tag: &str) -> bool {
        match self.peek_tag() {
            Some((current, _)) => current == tag,
            None => false,
        }
    }

    // Checks whether the next token is normal text instead of an annotation.
    fn at_text(&self) -> bool {
        match self.tokens.get(self.pos) {
            Some(token) => !token.starts_with('#'),
            None => false,
        }
    }

    // Gets the current token for error messages.
    fn describe(&self) -> String {
        match self.tokens.get(self.pos) {
            Some(token) => token.clone(),
            None => "end of input".to_string(),
        }
    }

    // Builds an error message with the token position.
    fn error(&self, message: &str) -> String {
        format!(
            "Syntax error at token {}: {} (found {})",
            self.pos + 1,
            message,
            self.describe()
        )
    }

    // Consumes an expected annotation or reports a syntax error.
    fn expect(&mut self, tag: &str) -> Result<(), String> {
        if !self.at(tag) {
            return Err(self.error(&format!("expected {}", tag)));
        }

        let count = self.peek_tag().unwrap().1;

        self.pos += count;
        self.leaf(tag, tag.to_string());

        Ok(())
    }

    // Parses a variable definition if one appears here.
    fn parse_optional_variable(&mut self) -> Result<(), String> {
        if self.at("#IHAZ") {
            self.parse_variable_define()?;
        }

        Ok(())
    }

    // Parses any comments that appear one after another.
    fn parse_comments(&mut self) -> Result<(), String> {
        while self.at("#OBTW") {
            self.parse_comment()?;
        }

        Ok(())
    }
}

impl SyntaxAnalyzer for Parser {
    // Checks the full program, starting with #HAI and ending with #KBYE.
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

    // Parses the head section and its optional title.
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

    // Parses the title text until #OIC.
    fn parse_title(&mut self) -> Result<(), String> {
        self.enter("title");

        self.expect("#GIMMEH TITLE")?;
        self.parse_optional_variable()?;
        self.parse_inner_text()?;
        self.expect("#OIC")?;

        self.leave();

        Ok(())
    }

    // Reads comment tokens until #TLDR.
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

    // Keeps parsing body elements until #KBYE.
    fn parse_body(&mut self) -> Result<(), String> {
        self.enter("body");

        while self.pos < self.tokens.len() && !self.at("#KBYE") {
            if self.at("#OBTW") {
                self.parse_comment()?;
            } else if self.at("#IHAZ") {
                self.parse_variable_define()?;
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

    // Parses one paragraph until its closing #MKAY.
    fn parse_paragraph(&mut self) -> Result<(), String> {
        self.enter("paragraph");

        self.expect("#MAEK PARAGRAF")?;
        self.parse_optional_variable()?;
        self.parse_inner_paragraph()?;
        self.expect("#MKAY")?;

        self.leave();

        Ok(())
    }

    // Allows text, variables, and other supported elements inside a paragraph.
    fn parse_inner_paragraph(&mut self) -> Result<(), String> {
        self.enter("inner_paragraph");

        while self.pos < self.tokens.len() && !self.at("#MKAY") {
            if self.at("#OBTW") {
                self.parse_comment()?;
            } else if self.at("#IHAZ") {
                self.parse_variable_define()?;
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

    // Reads plain text and variable uses until #OIC.
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

    // Reads a variable name and its value after #IHAZ and #ITIZ.
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

    // Reads the variable name requested by #GIMMEH VAR.
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

    // Parses the text inside a bold section.
    fn parse_bold(&mut self) -> Result<(), String> {
        self.enter("bold");

        self.expect("#GIMMEH BOLD")?;
        self.parse_optional_variable()?;
        self.parse_inner_text()?;
        self.expect("#OIC")?;

        self.leave();

        Ok(())
    }

    // Parses a list and its items until #MKAY.
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

    // Requires at least one item and parses any following items.
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

    // Parses the contents of one list item.
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

    // Reads the link address until #OIC.
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

    // Parses a newline command.
    fn parse_newline(&mut self) -> Result<(), String> {
        self.enter("newline");

        self.expect("#NEWLINE")?;

        self.leave();

        Ok(())
    }

    // Adds the next plain-text token to the tree.
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
