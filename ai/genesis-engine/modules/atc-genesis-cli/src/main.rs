use std::{env, fs, path::Path};
fn usage() {
    println!(
        "Genesis Engine CLI

Commands:
  create <name>       Create a project skeleton
  build [debug|release]  Validate/build the workspace
  test                Run the engine test suite
  run                 Start the game runtime
  package             Prepare a distributable package
  doctor              Check local toolchain prerequisites"
    );
}
fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("create") => {
            let name = args.next().unwrap_or_else(|| "game".into());
            if let Err(e) = create_project(&name) {
                eprintln!("create failed: {e}");
                std::process::exit(1)
            }
            println!("created project {name}");
        }
        Some("build") => run_cargo("check", args.next().as_deref()),
        Some("test") => run_cargo("test", None),
        Some("run") => println!("runtime launch delegated to the configured game target"),
        Some("package") => println!("package command delegated to genesis-build"),
        Some("doctor") => {
            println!("rust: {}", rustc_version());
            println!("cargo: {}", command_version("cargo"));
            println!(
                "workspace: {}",
                if Path::new("Cargo.toml").exists() {
                    "ready"
                } else {
                    "missing"
                }
            );
        }
        _ => usage(),
    }
}
fn create_project(name: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("project name is empty".into());
    }
    let root = Path::new(name);
    if root.exists() {
        return Err("target already exists".into());
    }
    fs::create_dir_all(root.join("assets")).map_err(|e| e.to_string())?;
    fs::create_dir_all(root.join("src")).map_err(|e| e.to_string())?;
    fs::write(
        root.join("README.md"),
        format!(
            "# {name}

Genesis Engine project.
"
        ),
    )
    .map_err(|e| e.to_string())?;
    fs::write(
        root.join("src/main.rs"),
        "fn main() { println!(\"Genesis Engine game runtime\"); }
",
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
fn run_cargo(command: &str, profile: Option<&str>) {
    let mut c = std::process::Command::new("cargo");
    c.arg(command).arg("--workspace").arg("--all-targets");
    if profile == Some("release") {
        c.arg("--release");
    }
    match c.status() {
        Ok(s) if s.success() => println!("cargo {command}: PASS"),
        Ok(s) => {
            eprintln!("cargo {command}: failed ({s})");
            std::process::exit(s.code().unwrap_or(1))
        }
        Err(e) => {
            eprintln!("failed to start cargo: {e}");
            std::process::exit(1)
        }
    }
}
fn command_version(cmd: &str) -> String {
    std::process::Command::new(cmd)
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| "unavailable".into())
        .trim()
        .into()
}
fn rustc_version() -> String {
    command_version("rustc")
}
