
use std::collections::HashMap;
use crate::syntaxanalyzer::Node;

/// Checks variables and scopes.
pub struct SemanticAnalyzer {
    scopes: Vec<HashMap<String, String>>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            scopes: Vec::new(),
        }
    }

    // Starts a new scope.
    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn leave_scope(&mut self) {
        self.scopes.pop();
    }

    // Saves a variable in the current scope.
    fn define_variable(&mut self, name: String, value: String) -> Result<(), String> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name) {
                return Err(format!(
                    "Semantic error: variable '{}' already defined in this scope",
                    name
                ));
            }

            scope.insert(name, value);
            Ok(())
        } else {
            Err("Semantic error: no active scope".to_string())
        }
    }

    // Looks for a variable.
    fn lookup_variable(&self, name: &str) -> Option<String> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }

        None
    }

    fn is_scope(kind: &str) -> bool {
        matches!(
            kind,
            "lolcode" | "head" | "title" | "paragraph" | "bold" | "list" | "inner_list"
        )
    }

    fn get_variable_name(node: &Node) -> Result<String, String> {
        for child in &node.children {
            if child.kind == "variable_name" {
                if let Some(name) = &child.token {
                    return Ok(name.clone());
                }
            }
        }

        Err("Semantic error: missing variable name".to_string())
    }

    fn get_variable_value(node: &Node) -> Result<String, String> {
        let mut values = Vec::new();

        for child in &node.children {
            if child.kind == "text" {
                if let Some(value) = &child.token {
                    values.push(value.clone());
                }
            }
        }

        if values.is_empty() {
            return Err("Semantic error: missing variable value".to_string());
        }

        Ok(values.join(" "))
    }

    fn resolve_node(&mut self, node: &mut Node) -> Result<(), String> {
        let new_scope = Self::is_scope(&node.kind);

        if new_scope {
            self.enter_scope();
        }

        let result = self.resolve_contents(node);

        if new_scope {
            self.leave_scope();
        }

        result
    }

    // Checks variable definitions and uses.
    fn resolve_contents(&mut self, node: &mut Node) -> Result<(), String> {
        if node.kind == "variable_define" {
            let name = Self::get_variable_name(node)?;
            let value = Self::get_variable_value(node)?;
            self.define_variable(name, value)?;
            return Ok(());
        }

        if node.kind == "variable_use" {
            let name = Self::get_variable_name(node)?;

            match self.lookup_variable(&name) {
                Some(value) => {
                    node.kind = "resolved_variable".to_string();
                    node.token = Some(value);
                    node.children.clear();
                    return Ok(());
                }
                None => {
                    return Err(format!(
                        "Semantic error: variable '{}' used before definition or outside its scope",
                        name
                    ));
                }
            }
        }

        for child in &mut node.children {
            self.resolve_node(child)?;
        }

        Ok(())
    }

    /// Checks the syntax tree.
    pub fn analyze(&mut self, tree: &mut Node) -> Result<(), String> {
        self.scopes.clear();
        self.resolve_node(tree)
    }
}
