
use std::collections::HashMap;
use crate::syntaxanalyzer::Node;

/// Checks variable definitions, uses, and scopes.
pub struct SemanticAnalyzer {
    scopes: Vec<HashMap<String, String>>,
}

impl SemanticAnalyzer {
    /// Starts the analyzer with no variable scopes.
    pub fn new() -> Self {
        SemanticAnalyzer {
            scopes: Vec::new(),
        }
    }

    // Adds a new scope for variables defined inside this section.
    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    // Removes the current scope when leaving a section.
    fn leave_scope(&mut self) {
        self.scopes.pop();
    }

    // Saves a variable, but rejects a duplicate name in the same scope.
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

    // Searches from the newest scope outward so local variables take priority.
    fn lookup_variable(&self, name: &str) -> Option<String> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }

        None
    }

    // Checks which syntax tree nodes create their own variable scope.
    fn is_scope(kind: &str) -> bool {
        matches!(
            kind,
            "lolcode" | "head" | "title" | "paragraph" | "bold" | "list" | "inner_list"
        )
    }

    // Finds the variable name stored in a node.
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

    // Joins the text tokens that make up a variable's value.
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

    // Opens and closes a scope around nodes that need one.
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

    // Checks definitions and replaces variable uses with their saved values.
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

    /// Clears old scopes and checks the whole syntax tree.
    pub fn analyze(&mut self, tree: &mut Node) -> Result<(), String> {
        self.scopes.clear();
        self.resolve_node(tree)
    }
}
