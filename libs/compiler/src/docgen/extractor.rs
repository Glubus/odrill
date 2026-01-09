//! Documentation comment extractor
//!
//! Extracts LDoc-style documentation comments from Lua source files.

use std::path::Path;

/// A parsed documentation comment
#[derive(Debug, Clone)]
pub struct DocComment {
    /// Main description text
    pub description: String,
    /// @param annotations
    pub params: Vec<ParamDoc>,
    /// @return annotations
    pub returns: Vec<ReturnDoc>,
    /// @example annotations
    pub examples: Vec<String>,
    /// @see references
    pub see: Vec<String>,
    /// Line number where doc starts
    pub line: usize,
}

/// Parameter documentation
#[derive(Debug, Clone)]
pub struct ParamDoc {
    pub name: String,
    pub type_hint: Option<String>,
    pub description: String,
}

/// Return value documentation
#[derive(Debug, Clone)]
pub struct ReturnDoc {
    pub type_hint: Option<String>,
    pub description: String,
}

/// A documented item (function, table, variable)
#[derive(Debug, Clone)]
pub struct DocItem {
    /// Item name
    pub name: String,
    /// Item kind (function, table, variable)
    pub kind: DocItemKind,
    /// Associated documentation
    pub doc: DocComment,
    /// Source file
    pub file: String,
    /// Line number of definition
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DocItemKind {
    Function,
    LocalFunction,
    Table,
    Variable,
    Module,
}

impl DocItemKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            DocItemKind::Function => "function",
            DocItemKind::LocalFunction => "local function",
            DocItemKind::Table => "table",
            DocItemKind::Variable => "variable",
            DocItemKind::Module => "module",
        }
    }
}

/// Extract documentation from a Lua source file
pub fn extract_docs(path: &Path, source: &str) -> Vec<DocItem> {
    let mut items = Vec::new();
    let lines: Vec<&str> = source.lines().collect();

    let mut current_doc: Option<(usize, Vec<String>)> = None;

    for (idx, line) in lines.iter().enumerate() {
        let line_num = idx + 1;
        let trimmed = line.trim();

        // Check for doc comment start (--- or ---)
        if trimmed.starts_with("---") {
            let content = trimmed.trim_start_matches('-').trim();
            if let Some((_, ref mut doc_lines)) = current_doc {
                doc_lines.push(content.to_string());
            } else {
                current_doc = Some((line_num, vec![content.to_string()]));
            }
        } else if let Some((start_line, doc_lines)) = current_doc.take() {
            // Check if this line defines something
            if let Some(item) = parse_definition(trimmed, line_num, path, &doc_lines, start_line) {
                items.push(item);
            }
            // If not a definition, the doc comment is orphaned (ignored)
        }
    }

    items
}

/// Parse a Lua definition line and associate it with documentation
fn parse_definition(
    line: &str,
    line_num: usize,
    path: &Path,
    doc_lines: &[String],
    doc_start_line: usize,
) -> Option<DocItem> {
    let file_name = path.file_name()?.to_string_lossy().to_string();

    // Parse function definitions
    if line.starts_with("function ") {
        let name = extract_function_name(line)?;
        return Some(DocItem {
            name,
            kind: DocItemKind::Function,
            doc: parse_doc_comment(doc_lines, doc_start_line),
            file: file_name,
            line: line_num,
        });
    }

    if line.starts_with("local function ") {
        let name = extract_local_function_name(line)?;
        return Some(DocItem {
            name,
            kind: DocItemKind::LocalFunction,
            doc: parse_doc_comment(doc_lines, doc_start_line),
            file: file_name,
            line: line_num,
        });
    }

    // Parse variable/table definitions with = function
    if line.contains(" = function") || line.contains("= function") {
        let name = line
            .split('=')
            .next()?
            .trim()
            .trim_start_matches("local ")
            .to_string();
        return Some(DocItem {
            name,
            kind: DocItemKind::Function,
            doc: parse_doc_comment(doc_lines, doc_start_line),
            file: file_name,
            line: line_num,
        });
    }

    // Parse module definitions (M = {} or ModuleName = {})
    if line.contains(" = {") && !line.contains("function") {
        let name = line
            .split('=')
            .next()?
            .trim()
            .trim_start_matches("local ")
            .to_string();
        return Some(DocItem {
            name,
            kind: DocItemKind::Table,
            doc: parse_doc_comment(doc_lines, doc_start_line),
            file: file_name,
            line: line_num,
        });
    }

    None
}

fn extract_function_name(line: &str) -> Option<String> {
    let after_func = line.strip_prefix("function ")?.trim();
    let name = after_func.split('(').next()?.trim();
    Some(name.to_string())
}

fn extract_local_function_name(line: &str) -> Option<String> {
    let after_func = line.strip_prefix("local function ")?.trim();
    let name = after_func.split('(').next()?.trim();
    Some(name.to_string())
}

/// Parse documentation comment lines into structured DocComment
fn parse_doc_comment(lines: &[String], start_line: usize) -> DocComment {
    let mut description = String::new();
    let mut params = Vec::new();
    let mut returns = Vec::new();
    let mut examples = Vec::new();
    let mut see = Vec::new();
    let mut in_example = false;

    for line in lines {
        let trimmed = line.trim();

        if trimmed.starts_with("@param") {
            in_example = false;
            if let Some(param) = parse_param(trimmed) {
                params.push(param);
            }
        } else if trimmed.starts_with("@return") {
            in_example = false;
            if let Some(ret) = parse_return(trimmed) {
                returns.push(ret);
            }
        } else if trimmed.starts_with("@example") {
            in_example = true;
        } else if trimmed.starts_with("@see") {
            in_example = false;
            let reference = trimmed.strip_prefix("@see").unwrap_or("").trim();
            if !reference.is_empty() {
                see.push(reference.to_string());
            }
        } else if in_example {
            examples.push(trimmed.to_string());
        } else if !trimmed.starts_with('@') {
            // Regular description line
            if !description.is_empty() {
                description.push(' ');
            }
            description.push_str(trimmed);
        }
    }

    DocComment {
        description,
        params,
        returns,
        examples,
        see,
        line: start_line,
    }
}

fn parse_param(line: &str) -> Option<ParamDoc> {
    // Format: @param name type Description
    // or: @param name Description
    let content = line.strip_prefix("@param")?.trim();
    let mut parts = content.splitn(3, ' ');

    let name = parts.next()?.to_string();
    let second = parts.next().unwrap_or("");
    let third = parts.next().unwrap_or("");

    // Check if second part looks like a type
    if second.starts_with(|c: char| c.is_lowercase()) && !second.contains(' ') && !third.is_empty()
    {
        // second is type, third is description
        Some(ParamDoc {
            name,
            type_hint: Some(second.to_string()),
            description: third.to_string(),
        })
    } else {
        // No type, second is start of description
        let desc = if third.is_empty() {
            second.to_string()
        } else {
            format!("{} {}", second, third)
        };
        Some(ParamDoc {
            name,
            type_hint: None,
            description: desc,
        })
    }
}

fn parse_return(line: &str) -> Option<ReturnDoc> {
    // Format: @return type Description
    // or: @return Description
    let content = line.strip_prefix("@return")?.trim();
    let mut parts = content.splitn(2, ' ');

    let first = parts.next().unwrap_or("");
    let second = parts.next().unwrap_or("");

    // Check if first part looks like a type
    if first.starts_with(|c: char| c.is_lowercase()) && !first.contains(' ') && !second.is_empty() {
        Some(ReturnDoc {
            type_hint: Some(first.to_string()),
            description: second.to_string(),
        })
    } else {
        Some(ReturnDoc {
            type_hint: None,
            description: content.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_extract_function_doc() {
        let source = r#"
--- Calculate the sum of two numbers
--- @param a number The first number
--- @param b number The second number
--- @return number The sum
function add(a, b)
    return a + b
end
"#;
        let items = extract_docs(&PathBuf::from("test.lua"), source);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "add");
        assert_eq!(items[0].doc.params.len(), 2);
    }
}
