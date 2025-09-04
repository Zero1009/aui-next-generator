use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::config::{ProjectConfig, DIRECTORIES};
use crate::files::*;
use crate::validation::install_dependencies_with_pnpm;

pub fn generate_project(config: &ProjectConfig) -> Result<()> {
    let project_path = Path::new(&config.name);

    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists!", config.name);
    }

    println!("\n🏗️  Creating project: {}", config.name.yellow());

    fs::create_dir(project_path)?;
    create_directories(project_path)?;
    create_files(project_path, config)?;

    // Install dependencies as the final step
    if config.install_deps {
        println!("\n📦 Installing dependencies...");
        install_dependencies_with_pnpm(project_path)?;
    }

    Ok(())
}

fn create_directories(project_path: &Path) -> Result<()> {
    println!("{}", "📁 Creating directory structure...".blue());

    for dir in DIRECTORIES {
        let dir_path = project_path.join(dir);
        fs::create_dir_all(&dir_path)?;
        println!("   Created: {}", dir.green());
    }

    Ok(())
}

fn create_files(project_path: &Path, config: &ProjectConfig) -> Result<()> {
    println!("{}", "📝 Creating project files...".blue());

    create_package_json(project_path, config)?;
    create_tsconfig(project_path)?;
    create_postcss_config(project_path)?;
    create_next_config(project_path)?;
    create_eslint_config(project_path)?;
    create_gitignore(project_path)?;
    create_npmrc(project_path)?;
    create_app_layout(project_path, &config.name, config)?;
    create_app_page(project_path, &config.name)?;
    create_globals_css(project_path)?;
    create_button_component(project_path)?;

    if config.use_react_query {
        create_query_provider(project_path)?;
        create_api_client(project_path)?;
        create_example_hooks(project_path)?;
    }

    create_readme(project_path, &config.name)?;

    Ok(())
}
