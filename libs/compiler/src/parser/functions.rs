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

    // Scan from current line to handle one-liners
    let end_line = find_function_end(lines, start);
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
    let mut depth = 0;
    let mut j = start;

    while j < lines.len() {
        let inner = lines[j].trim();
        depth += count_block_opens(inner);
        depth -= count_block_closes(inner);

        j += 1;
        if depth <= 0 {
            break;
        }
    }

    j
}
// ... (count_block_opens/closes unchanged)

// ... (count_block_opens/closes unchanged)

fn count_block_opens(line: &str) -> i32 {
    let openers = ["function", "local function", "if", "for", "while", "do"];

    for &opener in &openers {
        if line == opener
            || line.starts_with(&format!("{} ", opener))
            || line.starts_with(&format!("{}\t", opener))
        {
            return 1;
        }
    }
    0
}

fn count_block_closes(line: &str) -> i32 {
    if line == "end" || line.ends_with(" end") {
        1
    } else {
        0
    }
}

/// Build a symbol table from functions and tables: name -> FunctionDef
pub fn build_symbol_table(source: &str, function_pattern: &Regex) -> HashMap<String, FunctionDef> {
    let mut symbols: HashMap<String, FunctionDef> = extract_functions(source, function_pattern)
        .into_iter()
        .map(|f| (f.name.clone(), f))
        .collect();

    // Also extract local tables (for constants like PALETTE, ENEMIES, etc.)
    let table_pattern = Regex::new(r#"^local\s+([A-Z_][A-Z0-9_]*)\s*=\s*\{"#).unwrap();
    let lines: Vec<&str> = source.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        if let Some(cap) = table_pattern.captures(line) {
            let name = cap[1].to_string();
            if symbols.contains_key(&name) {
                continue;
            }

            // Find closing brace
            let start = i;
            let mut depth = 0;
            let mut end = i;

            while end < lines.len() {
                let l = lines[end];
                depth += l.matches('{').count() as i32;
                depth -= l.matches('}').count() as i32;

                end += 1;
                if depth <= 0 {
                    break;
                }
            }

            let content = lines[start..end].join("\n");

            symbols.insert(
                name.clone(),
                FunctionDef {
                    name,
                    start_line: start + 1,
                    end_line: end,
                    content,
                    is_local: true,
                },
            );
        }
    }

    symbols
}
