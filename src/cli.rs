use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use dialoguer::{Confirm, Input};

use crate::config::ProjectConfig;
use crate::validation::{check_node_version, check_and_install_pnpm};

#[derive(Parser)]
#[command(name = "aui-next-gen")]
#[command(about = "Generate Next.js projects with Tailwind & ESLint using pnpm")]
pub struct Cli {
    /// Project name (optional - will prompt if not provided)
    pub name: Option<String>,

    /// Skip dependency installation
    #[arg(long)]
    pub skip_install: bool,
}

pub fn get_project_config(args: Cli) -> Result<ProjectConfig> {
    println!("{}", "🚀 AUI Next.js Generator".bold().blue());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".blue());

    let project_name = match args.name {
        Some(name) => {
            println!("{}", name.green());
            name
        }
        None => {
            let name: String = Input::new()
                .with_prompt("📝 Enter project name")
                .interact()?;

            if name.trim().is_empty() {
                anyhow::bail!("Project name cannot be empty");
            }
            name
        }
    };

    let (install_deps, use_turbo) = if args.skip_install {
        (false, false)
    } else {
        check_node_version()?;
        check_and_install_pnpm()?;

        let install = Confirm::new()
            .with_prompt("📦 Install project dependencies")
            .default(true)
            .interact()?;

        let turbo = if install {
            Confirm::new()
                .with_prompt("🚀 Use Turbopack for faster development")
                .default(true)
                .interact()?
        } else {
            false
        };

        (install, turbo)
    };

    Ok(ProjectConfig::new(project_name, install_deps, use_turbo))
}

pub fn print_success_message(config: &ProjectConfig) {
    println!("\n{}", "🎉 Project created successfully!".green().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".green());
    println!("Next steps:");
    println!("   cd {}", config.name.blue());
    if !config.install_deps {
        println!("   pnpm install");
    }
    println!("   pnpm dev");
    println!("\n🌐 Then open http://localhost:3000");
}