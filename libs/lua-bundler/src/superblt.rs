//! SuperBLT output generator
//! Generates mod.txt, main.xml and other SuperBLT-specific files

use crate::config::BundleConfig;
use std::path::Path;

/// Generate SuperBLT mod files in dist/
pub fn generate_superblt_files(
    config: &BundleConfig,
    dist_dir: &Path,
    project_dir: &Path,
) -> anyhow::Result<()> {
    // Generate mod.txt
    let mod_txt = generate_mod_txt(config);
    std::fs::write(dist_dir.join("mod.txt"), mod_txt)?;

    // Generate main.xml
    let main_xml = generate_main_xml(config);
    std::fs::write(dist_dir.join("main.xml"), main_xml)?;

    // Copy loc/ folder if it exists
    let loc_src = project_dir.join("loc");
    if loc_src.exists() && loc_src.is_dir() {
        let loc_dist = dist_dir.join("loc");
        std::fs::create_dir_all(&loc_dist)?;
        for entry in std::fs::read_dir(&loc_src)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let filename = path.file_name().unwrap();
                std::fs::copy(&path, loc_dist.join(filename))?;
            }
        }
    }

    // Copy localization.lua if it exists
    let loc_lua_src = project_dir.join("src/localization.lua");
    if loc_lua_src.exists() {
        std::fs::copy(&loc_lua_src, dist_dir.join("localization.lua"))?;
    }

    Ok(())
}

fn generate_mod_txt(config: &BundleConfig) -> String {
    let pkg = &config.package;

    // Generate hooks JSON
    let hooks_json: Vec<String> = config
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

    format!(
        r#"{{
    "name": "{}",
    "description": "{}",
    "author": "{}",
    "version": "{}",
    "color": "0 0 0",
    "localization": "loc/",
    "hooks": [
{}
    ]
}}"#,
        pkg.name,
        pkg.description,
        pkg.author,
        pkg.version,
        hooks_json.join(",\n")
    )
}

fn generate_main_xml(config: &BundleConfig) -> String {
    let pkg = &config.package;

    format!(
        r#"<mod name="{}" author="{}">
    <Hooks directory="lua" />
</mod>"#,
        pkg.name, pkg.author
    )
}
