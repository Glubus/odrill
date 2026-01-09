//! Undefined variable detection rule

use crate::diagnostics::{Diagnostic, DiagnosticCode};
use crate::globals::KnownGlobals;
use full_moon::ast;
use full_moon::visitors::Visitor;
use std::collections::HashSet;
use std::path::PathBuf;

/// Visitor that detects undefined variables
pub struct UndefinedVariableVisitor<'a> {
    file: PathBuf,
    known_globals: &'a KnownGlobals,
    defined: HashSet<String>,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> UndefinedVariableVisitor<'a> {
    pub fn new(file: PathBuf, known_globals: &'a KnownGlobals, imports: Vec<String>) -> Self {
        let mut defined = HashSet::new();
        for import in imports {
            defined.insert(import);
        }

        Self {
            file,
            known_globals,
            defined,
            diagnostics: Vec::new(),
        }
    }

    pub fn finish(self) -> Vec<Diagnostic> {
        self.diagnostics
    }
}

impl Visitor for UndefinedVariableVisitor<'_> {
    fn visit_local_assignment(&mut self, node: &ast::LocalAssignment) {
        for name in node.names() {
            self.defined
                .insert(name.token().to_string().trim().to_string());
        }
    }

    fn visit_local_function(&mut self, node: &ast::LocalFunction) {
        self.defined
            .insert(node.name().token().to_string().trim().to_string());
    }

    fn visit_function_declaration(&mut self, node: &ast::FunctionDeclaration) {
        let name = node.name();
        if let Some(first) = name.names().first() {
            self.defined
                .insert(first.value().to_string().trim().to_string());
        }
    }

    fn visit_function_body(&mut self, node: &ast::FunctionBody) {
        // Track function parameters as defined variables
        for param in node.parameters().iter() {
            if let ast::Parameter::Name(name) = param {
                self.defined
                    .insert(name.token().to_string().trim().to_string());
            }
        }
    }

    fn visit_generic_for(&mut self, node: &ast::GenericFor) {
        for name in node.names() {
            self.defined
                .insert(name.token().to_string().trim().to_string());
        }
    }

    fn visit_numeric_for(&mut self, node: &ast::NumericFor) {
        let name = node.index_variable();
        self.defined
            .insert(name.token().to_string().trim().to_string());
    }

    fn visit_var(&mut self, node: &ast::Var) {
        if let ast::Var::Name(name) = node {
            let var_name = name.token().to_string().trim().to_string();
            if !self.defined.contains(&var_name) && !self.known_globals.contains(&var_name) {
                let token = name.token();
                let line = token.start_position().line();
                let col = token.start_position().character();
                self.diagnostics.push(Diagnostic::warning(
                    DiagnosticCode::UndefinedVariable,
                    format!("Undefined variable: {}", var_name),
                    self.file.clone(),
                    line,
                    col,
                ));
            }
        }
    }
}
