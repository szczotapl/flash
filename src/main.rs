use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use git2::Repository;
use dirs::home_dir;
use colored::*;

fn clone_repo(repo_url: &str, clone_dir: &Path) -> Result<(), git2::Error> {
    println!("{} Cloning {} into {}", ">>".green(), repo_url, clone_dir.display());
    Repository::clone(repo_url, clone_dir)?;
    Ok(())
}

fn run_config_script(package_dir: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    let config_file = package_dir.join("config.flash");
    if !config_file.exists() {
        return Err(format!("Config file not found: {}", config_file.display()).into());
    }

    let config_content = fs::read_to_string(config_file)?;
    let mut exec_command = None;
    let mut package_name = None;
    let mut package_desc = None;
    let mut clear = false;

    for line in config_content.lines() {
        if line.starts_with("exec=") {
            exec_command = Some(line[5..].to_string());
        } else if line.starts_with("name=") {
            package_name = Some(line[5..].to_string());
        } else if line.starts_with("desc=") {
            package_desc = Some(line[5..].to_string());
        } else if line.starts_with("clear=true") {
            clear = true;
        }
    }

    if let Some(name) = package_name {
        println!("{} Installing package: {}", ">>".green(), name);
    }
    if let Some(desc) = package_desc {
        println!("{} Description: {}", ">>".green(), desc);
    }

    print!("{} Confirm installation? [y/N]: ", ">>".yellow());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let confirm = input.trim().eq_ignore_ascii_case("y");

    if !confirm {
        println!("{} Installation aborted.", ">>".yellow());
        return Ok(false);
    }

    if let Some(command) = exec_command {
        println!("{} Running install command: {}", ">>".green(), command);
        let status = Command::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(package_dir)
            .status()?;

        if !status.success() {
            return Err(format!("Install command failed with status: {}", status).into());
        }
    } else {
        return Err("No exec command found in config file.".into());
    }

    Ok(clear)
}


fn list_packages() {
    println!("{} Installed packages:", ">>".yellow());
    let package_dir = home_dir().expect("Could not get home directory").join(".flash/packages");
    if let Ok(entries) = fs::read_dir(&package_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("  {}", entry.file_name().to_string_lossy());
            }
        }
    }
}

fn remove_package(package_name: &str) {
    let package_dir = home_dir().expect("Could not get home directory").join(".flash/packages").join(package_name);
    if package_dir.exists() {
        println!("{} Removing package {}...", ">>".yellow(), package_name);
        fs::remove_dir_all(&package_dir).expect("Failed to remove package directory");
        println!("{} Package {} removed.", ">>".green(), package_name);
    } else {
        println!("{} Package {} is not installed.", ">>".yellow(), package_name);
    }
}

fn update_package(package_name: &str) {
    let package_dir = home_dir().expect("Could not get home directory").join(".flash/packages").join(package_name);
    if package_dir.exists() {
        println!("{} Updating package {}...", ">>".yellow(), package_name);
        let status = Command::new("git")
            .arg("pull")
            .current_dir(&package_dir)
            .status()
            .expect("Failed to execute git pull command");
        if status.success() {
            println!("{} Package {} updated successfully.", ">>".green(), package_name);
        } else {
            println!("{} Failed to update package {}.", ">>".red(), package_name);
        }
    } else {
        println!("{} Package {} is not installed.", ">>".yellow(), package_name);
    }
}

fn update_all_packages() {
    println!("{} Updating all packages...", ">>".yellow());
    let package_dir = home_dir().expect("Could not get home directory").join(".flash/packages");
    if let Ok(entries) = fs::read_dir(&package_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(package_name) = entry.file_name().to_str() {
                    update_package(package_name);
                }
            }
        }
    }
}


fn ask_and_remove(clone_dir: &Path) {
    println!("{} Do you want to remove the directory {}? [y/N]", ">>".yellow(), clone_dir.display());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    if input.trim().eq_ignore_ascii_case("y") {
        fs::remove_dir_all(clone_dir).expect("Failed to remove directory");
        println!("{} Directory {} removed.", ">>".green(), clone_dir.display());
    } else {
        println!("{} Directory {} not removed.", ">>".yellow(), clone_dir.display());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [options]", args[0]);
        eprintln!("Commands:");
        eprintln!("  -S <github_user>/<repo>: Clone and install a package from GitHub.");
        eprintln!("  -L: List installed packages.");
        eprintln!("  -R <package>: Remove a package.");
        eprintln!("  -U <package>: Update a package.");
        eprintln!("  -UA: Update all installed packages.");
        std::process::exit(1);
    }

    let command = &args[1];
    match command.as_str() {
        "-S" => {
            if args.len() != 3 {
                eprintln!("Usage: {} -S <github_user>/<repo>", args[0]);
                std::process::exit(1);
            }
            let github_user_repo = &args[2];
            let parts: Vec<&str> = github_user_repo.split('/').collect();
            if parts.len() != 2 {
                eprintln!("Invalid repository format. Use <github_user>/<repo>.");
                std::process::exit(1);
            }
            let github_user = parts[0];
            let repo = parts[1];
            let repo_url = format!("https://github.com/{}/{}", github_user, repo);
            let clone_dir = home_dir()
                .expect("Could not get home directory")
                .join(".flash/packages")
                .join(repo);

            if clone_dir.exists() {
                println!("{} Package {} already exists.", ">>".yellow(), repo);
                ask_and_remove(&clone_dir);
            }

            if let Err(e) = clone_repo(&repo_url, &clone_dir) {
                eprintln!("{} Failed to clone repository: {}", ">>".red(), e);
                ask_and_remove(&clone_dir);
                std::process::exit(1);
            }

            match run_config_script(&clone_dir) {
                Ok(clear) => {
                    if clear {
                        fs::remove_dir_all(&clone_dir).expect("Failed to remove directory");
                        println!("{} Directory {} removed.", ">>".green(), clone_dir.display());
                    }
                },
                Err(e) => {
                    eprintln!("{} Failed to run config script: {}", ">>".red(), e);
                    ask_and_remove(&clone_dir);
                    std::process::exit(1);
                }
            }

            println!("{} Package {} installed successfully.", ">>".green(), repo);
        },
        "-L" => {
            list_packages();
        },
        "-R" => {
            if args.len() != 3 {
                eprintln!("Usage: {} -R <package>", args[0]);
                std::process::exit(1);
            }
            let package_name = &args[2];
            remove_package(package_name);
        },
        "-U" => {
            if args.len() != 3 {
                eprintln!("Usage: {} -U <package>", args[0]);
                std::process::exit(1);
            }
            let package_name = &args[2];
            update_package(package_name);
        },
        "-UA" => {
            update_all_packages();
        },
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}
