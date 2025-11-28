mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sword")]
#[command(about = "Sword Framework CLI - Generate backend projects with Axum and SeaORM", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Sword project
    New {
        /// Name of the project
        name: Option<String>,

        /// Output directory (defaults to parent directory)
        #[arg(short, long, default_value = "..")]
        out_dir: String,

        /// Initialize in the specified directory instead of creating a subdirectory
        #[arg(long)]
        init: bool,

        /// Disable interactive prompts
        #[arg(long)]
        no_interactive: bool,

        /// Application port (default: 3000)
        #[arg(short, long)]
        port: Option<u16>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            name,
            out_dir,
            init,
            no_interactive,
            port,
        } => {
            commands::new::execute(name, &out_dir, init, no_interactive, port)?;
        }
    }

    Ok(())
}
