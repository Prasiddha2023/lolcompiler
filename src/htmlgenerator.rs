use crate::syntaxanalyzer::Node;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct HtmlGenerator;

impl HtmlGenerator {
    pub fn new() -> Self {
        HtmlGenerator
    }

    fn escape_html(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    }

    fn render_children(&self, node: &Node) -> String {
        let mut result = String::new();
        let mut previous_text = false;

        for child in &node.children {
            let text = self.render(child);

            if text.is_empty() {
                continue;
            }

            let current_text = matches!(
                child.kind.as_str(),
                "text" | "resolved_variable" | "bold" | "link"
            );

            if previous_text && current_text {
                result.push(' ');
            }

            result.push_str(&text);
            previous_text = current_text;
        }

        result
    }

    fn render(&self, node: &Node) -> String {
        match node.kind.as_str() {
            "lolcode" => format!("<html>\n{}\n</html>\n", self.render_children(node)),
            "head" => format!("<head>\n{}\n</head>\n", self.render_children(node)),
            "title" => format!("<title>{}</title>\n", self.render_children(node)),
            "paragraph" => format!("<p>{}</p>\n", self.render_children(node)),
            "bold" => format!("<b>{}</b>", self.render_children(node)),
            "list" => format!("<ul>\n{}\n</ul>\n", self.render_children(node)),
            "inner_list" => format!("<li>{}</li>\n", self.render_children(node)),
            "link" => {
                let address = self.render_children(node);
                format!("<a href=\"{}\">{}</a>", address, address)
            }
            "newline" => "<br>\n".to_string(),
            "comment" => {
                let content = node.children.iter()
                    .filter(|child| child.kind == "comment_text")
                    .filter_map(|child| child.token.as_ref())
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(" ");
                format!("<!-- {} -->\n", content.replace("--", "- -"))
            }
            "variable_define" => String::new(),
            "text" | "resolved_variable" => {
                match &node.token {
                    Some(token) => Self::escape_html(token),
                    None => String::new(),
                }
            }
            "variable_name" | "comment_text" => String::new(),
            kind if kind.starts_with('#') => String::new(),
            _ => self.render_children(node),
        }
    }

    pub fn generate(&self, tree: &Node, filename: &str) -> Result<PathBuf, String> {
        let input_path = Path::new(filename);
        let output_path = input_path.with_extension("html");
        let html = self.render(tree);

        fs::write(&output_path, html)
            .map_err(|e| format!("Error writing HTML: {}", e))?;

        Ok(output_path)
    }

    pub fn open_browser(&self, path: &Path) -> Result<(), String> {
        let full_path = path.canonicalize()
            .map_err(|e| format!("Error finding HTML file: {}", e))?;

        #[cfg(target_os = "windows")]
        let result = Command::new("cmd")
            .args(["/C", "start", ""])
            .arg(&full_path)
            .spawn();

        #[cfg(target_os = "macos")]
        let result = Command::new("open")
            .arg(&full_path)
            .spawn();

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        let result = Command::new("xdg-open")
            .arg(&full_path)
            .spawn();

        result
            .map(|_| ())
            .map_err(|e| format!("Error opening browser: {}", e))
    }
}
