//! Use directive types and parsing

use regex::Regex;

/// Represents a selective import: `-- use module::symbol`
#[derive(Debug, Clone, PartialEq)]
pub struct UseDirective {
    pub module_path: String,
    pub symbols: Vec<String>, // Empty = import all (*)
    pub line: usize,
    pub full_match: String,
}

/// Extract use directives from source
pub fn extract_uses(
    source: &str,
    use_single: &Regex,
    use_multi: &Regex,
    use_all: &Regex,
) -> Vec<UseDirective> {
    let mut uses = Vec::new();

    for (line_num, line) in source.lines().enumerate() {
        let trimmed = line.trim();

        if let Some(dir) = try_parse_use_all(trimmed, line_num, use_all) {
            uses.push(dir);
            continue;
        }

        if let Some(dir) = try_parse_use_multi(trimmed, line_num, use_multi) {
            uses.push(dir);
            continue;
        }

        if let Some(dir) = try_parse_use_single(trimmed, line_num, use_single) {
            uses.push(dir);
        }
    }

    uses
}

fn try_parse_use_all(line: &str, line_num: usize, pattern: &Regex) -> Option<UseDirective> {
    pattern.captures(line).map(|cap| UseDirective {
        module_path: cap[1].to_string(),
        symbols: vec![],
        line: line_num + 1,
        full_match: line.to_string(),
    })
}

fn try_parse_use_multi(line: &str, line_num: usize, pattern: &Regex) -> Option<UseDirective> {
    pattern.captures(line).map(|cap| {
        let symbols: Vec<String> = cap[2]
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        UseDirective {
            module_path: cap[1].to_string(),
            symbols,
            line: line_num + 1,
            full_match: line.to_string(),
        }
    })
}

fn try_parse_use_single(line: &str, line_num: usize, pattern: &Regex) -> Option<UseDirective> {
    pattern.captures(line).map(|cap| UseDirective {
        module_path: cap[1].to_string(),
        symbols: vec![cap[2].to_string()],
        line: line_num + 1,
        full_match: line.to_string(),
    })
}
