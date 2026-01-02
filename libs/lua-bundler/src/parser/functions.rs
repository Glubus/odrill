//! Function definition extraction from Lua source

use regex::Regex;
use std::collections::HashMap;

/// Represents a function definition in Lua
#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
    pub is_local: bool,
}

/// Extract function definitions from Lua source
pub fn extract_functions(source: &str, pattern: &Regex) -> Vec<FunctionDef> {
    let lines: Vec<&str> = source.lines().collect();
    let mut functions = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        if let Some(func) = try_extract_function(&lines, i, pattern) {
            i = func.end_line;
            functions.push(func);
        } else {
            i += 1;
        }
    }

    functions
}

fn try_extract_function(lines: &[&str], start: usize, pattern: &Regex) -> Option<FunctionDef> {
    let line = lines[start];
    let cap = pattern.captures(line)?;

    let is_local = cap.get(1).is_some();
    let name = cap[2].to_string();
    let start_line = start + 1;

    let end_line = find_function_end(lines, start + 1);
    let content = lines[start..end_line].join("\n");

    Some(FunctionDef {
        name,
        start_line,
        end_line,
        content,
        is_local,
    })
}

fn find_function_end(lines: &[&str], start: usize) -> usize {
    let mut depth = 1;
    let mut j = start;

    while j < lines.len() && depth > 0 {
        let inner = lines[j].trim();
        depth += count_block_opens(inner);
        depth -= count_block_closes(inner);
        j += 1;
    }

    j
}

fn count_block_opens(line: &str) -> i32 {
    let openers = [
        "function ",
        "local function ",
        "if ",
        "for ",
        "while ",
        "do",
    ];
    let is_opener = openers.iter().any(|o| line.starts_with(o));
    if is_opener && !line.ends_with("end") {
        1
    } else {
        0
    }
}

fn count_block_closes(line: &str) -> i32 {
    if line == "end" || line.ends_with(" end") {
        1
    } else {
        0
    }
}

/// Build a symbol table from functions: name -> FunctionDef
pub fn build_symbol_table(source: &str, pattern: &Regex) -> HashMap<String, FunctionDef> {
    extract_functions(source, pattern)
        .into_iter()
        .map(|f| (f.name.clone(), f))
        .collect()
}
