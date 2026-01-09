//! HTML documentation generator

use crate::docgen::extractor::DocItem;

/// Generate HTML documentation
pub fn generate_html(items: &[DocItem], title: &str) -> String {
    let mut html = String::new();

    // HTML header with embedded CSS
    html.push_str(&format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - Documentation</title>
    <style>
        :root {{
            --bg: #0d1117;
            --bg-secondary: #161b22;
            --text: #c9d1d9;
            --text-muted: #8b949e;
            --accent: #58a6ff;
            --accent-secondary: #1f6feb;
            --border: #30363d;
            --code-bg: #1f2428;
            --success: #3fb950;
            --warning: #d29922;
        }}
        
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif;
            background: var(--bg);
            color: var(--text);
            line-height: 1.6;
        }}
        
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            display: grid;
            grid-template-columns: 280px 1fr;
            min-height: 100vh;
        }}
        
        /* Sidebar */
        .sidebar {{
            background: var(--bg-secondary);
            border-right: 1px solid var(--border);
            padding: 2rem 1rem;
            position: sticky;
            top: 0;
            height: 100vh;
            overflow-y: auto;
        }}
        
        .sidebar h1 {{
            font-size: 1.5rem;
            margin-bottom: 2rem;
            color: var(--accent);
        }}
        
        .sidebar nav a {{
            display: block;
            padding: 0.5rem 1rem;
            color: var(--text-muted);
            text-decoration: none;
            border-radius: 6px;
            font-size: 0.9rem;
            transition: all 0.2s;
        }}
        
        .sidebar nav a:hover {{
            background: var(--border);
            color: var(--text);
        }}
        
        .sidebar nav a .kind {{
            display: inline-block;
            font-size: 0.7rem;
            padding: 0.1rem 0.4rem;
            border-radius: 4px;
            margin-right: 0.5rem;
            background: var(--accent-secondary);
            color: white;
        }}
        
        /* Main content */
        .content {{
            padding: 3rem;
        }}
        
        .doc-item {{
            background: var(--bg-secondary);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 2rem;
            margin-bottom: 2rem;
        }}
        
        .doc-item h2 {{
            color: var(--accent);
            margin-bottom: 1rem;
            font-family: 'SFMono-Regular', Consolas, monospace;
        }}
        
        .doc-item .kind {{
            display: inline-block;
            font-size: 0.75rem;
            padding: 0.2rem 0.6rem;
            border-radius: 4px;
            background: var(--accent-secondary);
            color: white;
            margin-left: 0.5rem;
            vertical-align: middle;
        }}
        
        .doc-item .description {{
            margin: 1rem 0;
            color: var(--text);
        }}
        
        .doc-item .meta {{
            font-size: 0.85rem;
            color: var(--text-muted);
        }}
        
        .params, .returns {{
            margin-top: 1.5rem;
        }}
        
        .params h3, .returns h3 {{
            font-size: 0.9rem;
            color: var(--text-muted);
            margin-bottom: 0.5rem;
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }}
        
        .param, .return {{
            background: var(--code-bg);
            padding: 0.75rem 1rem;
            border-radius: 4px;
            margin-bottom: 0.5rem;
            font-size: 0.9rem;
        }}
        
        .param .name {{
            color: var(--accent);
            font-family: monospace;
        }}
        
        .param .type, .return .type {{
            color: var(--warning);
            font-family: monospace;
            font-size: 0.85rem;
        }}
        
        .example {{
            margin-top: 1.5rem;
        }}
        
        .example h3 {{
            font-size: 0.9rem;
            color: var(--text-muted);
            margin-bottom: 0.5rem;
        }}
        
        .example pre {{
            background: var(--code-bg);
            padding: 1rem;
            border-radius: 4px;
            overflow-x: auto;
            font-family: 'SFMono-Regular', Consolas, monospace;
            font-size: 0.85rem;
        }}
        
        @media (max-width: 768px) {{
            .container {{
                grid-template-columns: 1fr;
            }}
            .sidebar {{
                position: static;
                height: auto;
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <aside class="sidebar">
            <h1>{}</h1>
            <nav>
"#, title, title));

    // Navigation
    for item in items {
        html.push_str(&format!(
            "                <a href=\"#{}\">\
<span class=\"kind\">{}</span>{}</a>\n",
            item.name,
            item.kind.as_str(),
            item.name
        ));
    }

    html.push_str(
        r#"            </nav>
        </aside>
        <main class="content">
"#,
    );

    // Content
    for item in items {
        html.push_str(&format!(
            r#"            <section class="doc-item" id="{}">
                <h2>{}<span class="kind">{}</span></h2>
                <p class="description">{}</p>
                <p class="meta">Defined in <code>{}</code> at line {}</p>
"#,
            item.name,
            item.name,
            item.kind.as_str(),
            item.doc.description,
            item.file,
            item.line
        ));

        // Parameters
        if !item.doc.params.is_empty() {
            html.push_str(
                r#"                <div class="params">
                    <h3>Parameters</h3>
"#,
            );
            for param in &item.doc.params {
                let type_str = param.type_hint.as_deref().unwrap_or("any");
                html.push_str(&format!(
                    r#"                    <div class="param"><span class="name">{}</span> <span class="type">{}</span> - {}</div>
"#,
                    param.name, type_str, param.description
                ));
            }
            html.push_str("                </div>\n");
        }

        // Returns
        if !item.doc.returns.is_empty() {
            html.push_str(
                r#"                <div class="returns">
                    <h3>Returns</h3>
"#,
            );
            for ret in &item.doc.returns {
                let type_str = ret.type_hint.as_deref().unwrap_or("any");
                html.push_str(&format!(
                    r#"                    <div class="return"><span class="type">{}</span> - {}</div>
"#,
                    type_str, ret.description
                ));
            }
            html.push_str("                </div>\n");
        }

        // Examples
        if !item.doc.examples.is_empty() {
            html.push_str(
                r#"                <div class="example">
                    <h3>Example</h3>
                    <pre><code>"#,
            );
            html.push_str(&item.doc.examples.join("\n"));
            html.push_str("</code></pre>\n                </div>\n");
        }

        html.push_str("            </section>\n");
    }

    html.push_str(
        r#"        </main>
    </div>
</body>
</html>
"#,
    );

    html
}
