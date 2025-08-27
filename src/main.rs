use anyhow::Result;
use clap::Parser;

use aui_next_generator::{Cli, get_project_config, generate_project, print_success_message};

fn main() -> Result<()> {
    let args = Cli::parse();
    let config = get_project_config(args)?;
    
    generate_project(&config)?;
    print_success_message(&config);

    Ok(())
}