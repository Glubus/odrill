//! odrill init command - Initialize a new odrill project

use colored::Colorize;
use std::process::Command;

// Templates embedded at compile time
const TPL_ODRILL_TOML: &str = include_str!("../../../../templates/odrill/odrill.toml");
const TPL_GITIGNORE: &str = include_str!("../../../../templates/odrill/.gitignore");
const TPL_README: &str = include_str!("../../../../templates/odrill/README.md");
const TPL_CONSTS: &str = include_str!("../../../../templates/odrill/src/consts.lua");
const TPL_UTILS: &str = include_str!("../../../../templates/odrill/src/shared/utils.lua");
const TPL_MENUMANAGER: &str =
    include_str!("../../../../templates/odrill/src/hooks/menumanager/mod.lua");
const TPL_CHATMANAGER: &str =
    include_str!("../../../../templates/odrill/src/hooks/chatmanager/mod.lua");
const TPL_LOC_ENGLISH: &str = include_str!("../../../../templates/odrill/loc/english.json");
const TPL_LOC_FRENCH: &str = include_str!("../../../../templates/odrill/loc/french.json");

fn render(template: &str, vars: &[(&str, &str)]) -> String {
    let mut result = template.to_string();
    for (key, value) in vars {
        result = result.replace(&format!("{{{{{}}}}}", key), value);
    }
    result
}

pub fn run(name: &str, init_git: bool, _template: Option<&str>) -> anyhow::Result<()> {
    let project_dir = if name == "." {
        std::env::current_dir()?
    } else {
        let dir = std::env::current_dir()?.join(name);
        std::fs::create_dir_all(&dir)?;
        dir
    };

    let project_name = project_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("my-mod")
        .to_string();

    println!(
        "{} {}",
        "Creating odrill project:".cyan().bold(),
        project_name
    );

    let vars: &[(&str, &str)] = &[("name", &project_name)];

    // Create directory structure
    let dirs = [
        "src/hooks/menumanager",
        "src/hooks/chatmanager",
        "src/shared",
        "src/libs",
        "loc",
        "dist",
    ];
    for dir in dirs {
        std::fs::create_dir_all(project_dir.join(dir))?;
    }
    println!("  {} project structure", "mkdir".green());

    // Create project files
    std::fs::write(
        project_dir.join("odrill.toml"),
        render(TPL_ODRILL_TOML, vars),
    )?;
    std::fs::write(project_dir.join(".gitignore"), TPL_GITIGNORE)?;
    std::fs::write(project_dir.join("README.md"), render(TPL_README, vars))?;
    println!("  {} odrill.toml, .gitignore, README.md", "create".green());

    // Source files
    std::fs::write(project_dir.join("src/consts.lua"), render(TPL_CONSTS, vars))?;
    std::fs::write(project_dir.join("src/shared/utils.lua"), TPL_UTILS)?;
    std::fs::write(
        project_dir.join("src/hooks/menumanager/mod.lua"),
        render(TPL_MENUMANAGER, vars),
    )?;
    std::fs::write(
        project_dir.join("src/hooks/chatmanager/mod.lua"),
        render(TPL_CHATMANAGER, vars),
    )?;
    println!(
        "  {} src/consts.lua, src/shared/utils.lua",
        "create".green()
    );
    println!(
        "  {} src/hooks/menumanager/mod.lua, src/hooks/chatmanager/mod.lua",
        "create".green()
    );

    // Localization files
    std::fs::write(
        project_dir.join("loc/english.json"),
        render(TPL_LOC_ENGLISH, vars),
    )?;
    std::fs::write(
        project_dir.join("loc/french.json"),
        render(TPL_LOC_FRENCH, vars),
    )?;
    println!("  {} loc/english.json, loc/french.json", "create".green());

    // Initialize git
    if init_git {
        println!("\n{}", "Initializing git...".cyan());
        match Command::new("git")
            .args(["init"])
            .current_dir(&project_dir)
            .output()
        {
            Ok(o) if o.status.success() => println!("  {} git", "init".green()),
            _ => println!("  {} git", "skip".yellow()),
        }
    }

    println!("\n{}", "Project created!".green().bold());
    println!("\nThe project shows 'Hello World!' in chat when you join a game.");
    println!("Run {} to bundle and test in-game.", "odrill build".cyan());

    Ok(())
}
