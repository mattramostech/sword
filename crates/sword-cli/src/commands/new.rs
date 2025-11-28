use dialoguer::Input;
use include_dir::{include_dir, Dir, DirEntry};
use std::fs;
use std::path::{Path, PathBuf};

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates/base");

/// Version of the sword library that generated projects will use.
/// Update this constant when publishing a new version of the sword crate.
const SWORD_VERSION: &str = "0.1.0";

pub fn execute(
    name: Option<String>,
    out_dir: &str,
    init: bool,
    no_interactive: bool,
    port: Option<u16>,
) -> anyhow::Result<()> {
    let project_name = match name {
        Some(n) => n,
        None => {
            if no_interactive {
                anyhow::bail!("Project name is required when using --no-interactive");
            }
            Input::<String>::new()
                .with_prompt("Project name")
                .interact_text()?
        }
    };

    // Validate project name
    if project_name.is_empty() {
        anyhow::bail!("Project name cannot be empty");
    }

    let port = match port {
        Some(p) => p,
        None => {
            if no_interactive {
                3000
            } else {
                Input::<u16>::new()
                    .with_prompt("Application port")
                    .default(3000)
                    .interact_text()?
            }
        }
    };

    let out_path = PathBuf::from(out_dir);
    let project_path = if init {
        out_path
    } else {
        out_path.join(&project_name)
    };

    if !init && project_path.exists() {
        anyhow::bail!("Directory '{}' already exists", project_path.display());
    }

    println!("Creating project '{}'...", project_name);

    generate_project(&project_path, &project_name, port, SWORD_VERSION)?;

    println!("\n✓ Project '{}' created successfully!", project_name);
    println!("\nNext steps:");
    println!("  cd {}", project_name);
    println!("  docker compose up -d db");
    println!("  cargo run");

    Ok(())
}

fn generate_project(
    project_path: &Path,
    project_name: &str,
    port: u16,
    sword_version: &str,
) -> anyhow::Result<()> {
    fs::create_dir_all(project_path)?;
    extract_recursive(
        &TEMPLATES_DIR,
        project_path,
        project_name,
        port,
        sword_version,
    )?;
    Ok(())
}

fn extract_recursive(
    dir: &Dir,
    base_path: &Path,
    project_name: &str,
    port: u16,
    sword_version: &str,
) -> anyhow::Result<()> {
    for entry in dir.entries() {
        match entry {
            DirEntry::File(file) => {
                let mut path = base_path.join(file.path());

                // Rename env.template to .env
                if path.file_name().is_some_and(|n| n == "env.template") {
                    path.set_file_name(".env");
                }

                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }

                // Try to read as UTF-8 to replace placeholders
                if let Some(content_str) = file.contents_utf8() {
                    let new_content = content_str
                        .replace("{{PROJECT_NAME}}", project_name)
                        .replace("{{APP_PORT}}", &port.to_string())
                        .replace("{{SWORD_VERSION}}", sword_version);
                    fs::write(path, new_content)?;
                } else {
                    // Binary file, just copy
                    fs::write(path, file.contents())?;
                }
            }
            DirEntry::Dir(subdir) => {
                let path = base_path.join(subdir.path());
                fs::create_dir_all(&path)?;
                extract_recursive(subdir, base_path, project_name, port, sword_version)?;
            }
        }
    }
    Ok(())
}
