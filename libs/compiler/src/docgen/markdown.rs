//! Markdown documentation generator

use crate::docgen::extractor::DocItem;

/// Generate Markdown documentation
pub fn generate_markdown(items: &[DocItem], title: &str) -> String {
    let mut md = String::new();

    // Header
    md.push_str(&format!("# {}\n\n", title));

    // Table of contents
    md.push_str("## Table of Contents\n\n");
    for item in items {
        md.push_str(&format!("- [{}](#{})\n", item.name, slug(&item.name)));
    }
    md.push_str("\n---\n\n");

    // Items
    for item in items {
        // Header with kind badge
        md.push_str(&format!("## {} `{}`\n\n", item.name, item.kind.as_str()));

        // Description
        if !item.doc.description.is_empty() {
            md.push_str(&format!("{}\n\n", item.doc.description));
        }

        // Meta info
        md.push_str(&format!(
            "_Defined in `{}` at line {}_\n\n",
            item.file, item.line
        ));

        // Parameters
        if !item.doc.params.is_empty() {
            md.push_str("### Parameters\n\n");
            md.push_str("| Name | Type | Description |\n");
            md.push_str("|------|------|-------------|\n");
            for param in &item.doc.params {
                let type_str = param.type_hint.as_deref().unwrap_or("any");
                md.push_str(&format!(
                    "| `{}` | `{}` | {} |\n",
                    param.name, type_str, param.description
                ));
            }
            md.push('\n');
        }

        // Returns
        if !item.doc.returns.is_empty() {
            md.push_str("### Returns\n\n");
            for ret in &item.doc.returns {
                let type_str = ret.type_hint.as_deref().unwrap_or("any");
                md.push_str(&format!("- `{}` - {}\n", type_str, ret.description));
            }
            md.push('\n');
        }

        // Examples
        if !item.doc.examples.is_empty() {
            md.push_str("### Example\n\n");
            md.push_str("```lua\n");
            md.push_str(&item.doc.examples.join("\n"));
            md.push_str("\n```\n\n");
        }

        // See also
        if !item.doc.see.is_empty() {
            md.push_str("### See Also\n\n");
            for see_ref in &item.doc.see {
                md.push_str(&format!("- {}\n", see_ref));
            }
            md.push('\n');
        }

        md.push_str("---\n\n");
    }

    md
}

/// Convert a name to a URL-safe slug
fn slug(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect()
}
