//! Unused variable detection rule

use crate::diagnostics::{Diagnostic, DiagnosticCode};
use full_moon::ast;
use full_moon::visitors::Visitor;
use std::collections::HashMap;
use std::path::PathBuf;

/// Visitor that detects unused local variables
pub struct UnusedVariableVisitor {
    file: PathBuf,
    /// Scope stack: each scope has (variable_name, line, column, used)
    scopes: Vec<HashMap<String, (usize, usize, bool)>>,
    diagnostics: Vec<Diagnostic>,
}

impl UnusedVariableVisitor {
    pub fn new(file: PathBuf) -> Self {
        Self {
            file,
            scopes: vec![HashMap::new()],
            diagnostics: Vec::new(),
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            for (name, (line, col, used)) in scope {
                // Ignore variables starting with _ (intentionally unused) and "self"
                if !used && !name.starts_with('_') && name != "self" {
                    self.diagnostics.push(Diagnostic::warning(
                        DiagnosticCode::UnusedVariable,
                        format!("Unused variable: {}", name),
                        self.file.clone(),
                        line,
                        col,
                    ));
                }
            }
        }
    }

    fn declare(&mut self, name: &str, line: usize, col: usize) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), (line, col, false));
        }
    }

    fn use_var(&mut self, name: &str) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some((_, _, used)) = scope.get_mut(name) {
                *used = true;
                return;
            }
        }
    }

    /// Mark a prefix expression's base variable as used
    fn mark_prefix_as_used(&mut self, prefix: &ast::Prefix) {
        match prefix {
            ast::Prefix::Name(name) => {
                self.use_var(name.token().to_string().trim());
            }
            ast::Prefix::Expression(expr) => {
                // For expressions like (foo).bar, recurse into the expression
                // This is rare but possible
                if let ast::Expression::Var(var) = expr.as_ref() {
                    if let ast::Var::Name(name) = var {
                        self.use_var(name.token().to_string().trim());
                    }
                }
            }
            _ => {}
        }
    }

    pub fn finish(mut self) -> Vec<Diagnostic> {
        while !self.scopes.is_empty() {
            self.pop_scope();
        }
        self.diagnostics
    }
}

impl Visitor for UnusedVariableVisitor {
    fn visit_local_assignment(&mut self, node: &ast::LocalAssignment) {
        for name in node.names() {
            let token = name.token();
            let line = token.start_position().line();
            let col = token.start_position().character();
            self.declare(name.token().to_string().trim(), line, col);
        }
    }

    fn visit_var(&mut self, node: &ast::Var) {
        match node {
            ast::Var::Name(name) => {
                self.use_var(name.token().to_string().trim());
            }
            ast::Var::Expression(var_expr) => {
                // Handle expressions like player._killstreak
                // The prefix contains the variable being accessed
                self.mark_prefix_as_used(var_expr.prefix());
            }
            _ => {}
        }
    }

    fn visit_function_call(&mut self, node: &ast::FunctionCall) {
        self.mark_prefix_as_used(node.prefix());
    }

    fn visit_function_body(&mut self, node: &ast::FunctionBody) {
        self.push_scope();
        for param in node.parameters().iter() {
            if let ast::Parameter::Name(name) = param {
                let token = name.token();
                let line = token.start_position().line();
                let col = token.start_position().character();
                self.declare(name.token().to_string().trim(), line, col);
            }
        }
    }

    fn visit_function_body_end(&mut self, _node: &ast::FunctionBody) {
        self.pop_scope();
    }

    fn visit_do(&mut self, _node: &ast::Do) {
        self.push_scope();
    }

    fn visit_do_end(&mut self, _node: &ast::Do) {
        self.pop_scope();
    }

    fn visit_generic_for(&mut self, node: &ast::GenericFor) {
        self.push_scope();
        for name in node.names() {
            let token = name.token();
            let line = token.start_position().line();
            let col = token.start_position().character();
            self.declare(name.token().to_string().trim(), line, col);
        }
    }

    fn visit_generic_for_end(&mut self, _node: &ast::GenericFor) {
        self.pop_scope();
    }

    fn visit_numeric_for(&mut self, node: &ast::NumericFor) {
        self.push_scope();
        let name = node.index_variable();
        let token = name.token();
        let line = token.start_position().line();
        let col = token.start_position().character();
        self.declare(name.token().to_string().trim(), line, col);
    }

    fn visit_numeric_for_end(&mut self, _node: &ast::NumericFor) {
        self.pop_scope();
    }
}
