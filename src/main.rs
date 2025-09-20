use clap::{Parser, Subcommand};
use input_source_manager::{self, InputSourceError};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(name = "macism-rust", about = "A Rust-based input source manager for macOS, mimicking macism.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Print version information
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version, help = "Print version information")]
    version: Option<bool>,

    /// List all available input source IDs
    #[arg(short = 'l', long = "list", help = "List all available input source IDs")]
    list: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {

    /// Get the current input source ID
    Get {},
    /// Set the input source to a specific ID
    Set {
        /// The ID of the input source to set
        id: String,
    },
}

fn main() -> Result<(), InputSourceError> {
    // Initialize the Swift bridge
    input_source_manager::initialize();

    let cli = Cli::parse();

    // Handle --version flag if present
    if cli.version.is_some() {
        println!("{}", VERSION);
        return Ok(());
    }

    if cli.list {
        let ids = input_source_manager::get_available_ids()?;
        for id in ids {
            println!("{}", id);
        }
    } else {
        match cli.command {
            Some(Commands::Get {}) => {
                let current_id = input_source_manager::get_current_input_source_id()?;
                println!("{}", current_id);
            }
            Some(Commands::Set { id }) => {
                let new_id = input_source_manager::set_input_source(&id)?;
                println!("{}", new_id);
            }
            None => {
                // Default behavior: get current input source if no command is specified
                let current_id = input_source_manager::get_current_input_source_id()?;
                println!("{}", current_id);
            }
        }
    }

    Ok(())
}
