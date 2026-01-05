//! SuperBLT output generator
//! Generates mod.txt, main.xml and other SuperBLT-specific files

use pkg::manifest::OdrillManifest;
use std::path::Path;

/// Generate SuperBLT mod files in dist/
pub fn generate_superblt_files(
    manifest: &OdrillManifest,
    dist_dir: &Path,
    project_dir: &Path,
) -> anyhow::Result<()> {
    // Generate mod.txt
    let mod_txt = generate_mod_txt(manifest);
    std::fs::write(dist_dir.join("mod.txt"), mod_txt)?;

    // Generate main.xml
    let main_xml = generate_main_xml(manifest);
    std::fs::write(dist_dir.join("main.xml"), main_xml)?;

    // Implement generic asset copying
    for asset_path in &manifest.assets {
        let src = project_dir.join(asset_path);
        let dist = dist_dir.join(asset_path);

        if src.is_dir() {
            // Recursive copy
            copy_dir_recursive(&src, &dist)?;
        } else if src.exists() {
            // Ensure parent dir exists
            if let Some(parent) = dist.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(&src, &dist)?;
        } else {
            eprintln!("Warning: Asset not found: {}", src.display());
        }
    }

    // Copy loc/ folder if it exists (Backwards compatibility / auto-detection)
    // Only if not explicitly in assets?
    // Let's keep it for now.
    let loc_src = project_dir.join("loc");
    if loc_src.exists() && loc_src.is_dir() {
        let loc_dist = dist_dir.join("loc");
        // Check if already created by assets
        if !loc_dist.exists() {
            copy_dir_recursive(&loc_src, &loc_dist)?;
        }
    }

    // Copy localization.lua if it exists
    let loc_lua_src = project_dir.join("src/localization.lua");
    if loc_lua_src.exists() {
        std::fs::copy(&loc_lua_src, dist_dir.join("localization.lua"))?;
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dist: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(dist)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name().unwrap();
        let target = dist.join(filename);
        if path.is_dir() {
            copy_dir_recursive(&path, &target)?;
        } else {
            std::fs::copy(&path, &target)?;
        }
    }
    Ok(())
}

fn generate_mod_txt(manifest: &OdrillManifest) -> String {
    let pkg = &manifest.package;

    // Generate hooks JSON
    let hooks_json: Vec<String> = manifest
        .hooks
        .iter()
        .map(|h| {
            format!(
                r#"        {{
            "hook_id": "{}",
            "script_path": "{}"
        }}"#,
                h.id,
                h.output.display()
            )
        })
        .collect();

    // No localization in mod.txt - it goes in main.xml
    format!(
        r#"{{
    "name": "{}",
    "description": "{}",
    "author": "{}",
    "version": "{}",
    "color": "0 0 0",
    "hooks": [
{}
    ]
}}"#,
        pkg.name,
        pkg.description.as_deref().unwrap_or(""),
        pkg.authors.join(", "),
        pkg.version,
        hooks_json.join(",\n")
    )
}

fn generate_main_xml(manifest: &OdrillManifest) -> String {
    let pkg = &manifest.package;

    // Generate localization XML if present
    let loc_xml = if !manifest.localization.is_empty() {
        let loc_items: Vec<String> = manifest
            .localization
            .iter()
            .map(|l| {
                format!(
                    r#"        <Localization language="english" file="{}"/>"#,
                    l.default
                )
            })
            .collect();

        let directory = manifest
            .localization
            .first()
            .map(|l| l.directory.as_str())
            .unwrap_or("loc");

        let default_file = manifest
            .localization
            .first()
            .map(|l| l.default.as_str())
            .unwrap_or("english.txt");

        format!(
            r#"    <Localization directory="{}" default="{}">
{}
    </Localization>"#,
            directory,
            default_file,
            loc_items.join("\n")
        )
    } else {
        String::new()
    };

    // main.xml is for SuperBLT XML format
    // Hooks are loaded via mod.txt JSON, not main.xml
    // main.xml only contains localization and metadata
    format!(
        r#"<mod name="{}" author="{}">
{}
    <priority>100</priority>
</mod>"#,
        pkg.name,
        pkg.authors.join(", "),
        loc_xml
    )
}
