use clap::Parser;
use input_source_manager::{self, InputSourceError};



#[derive(Parser, Debug)]
#[command(name = "macism-rust", version, about = "A Rust-based input source manager for macOS, mimicking macism.", long_about = None)]
struct Cli {
    /// The input source ID to set (positional argument)
    #[arg(last = true)]
    id: Option<String>,

    /// List all available input source IDs
    #[arg(
        short = 'l',
        long = "list",
        help = "List all available input source IDs"
    )]
    list: bool,

    /// List only palette input source IDs
    #[arg(
        short = 'p',
        long = "palette",
        help = "List only palette input source IDs"
    )]
    palette: bool,
}



fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), InputSourceError> {
    // Initialize the Swift bridge
    input_source_manager::initialize();

    let cli = Cli::parse();

    // Handle list/palette options
    if cli.list || cli.palette {
        let category = if cli.list && cli.palette {
            input_source_manager::InputSourceCategory::All
        } else if cli.list {
            input_source_manager::InputSourceCategory::Keyboard
        } else if cli.palette {
            input_source_manager::InputSourceCategory::Palette
        } else {
            unreachable!();
        };
        let ids = input_source_manager::get_available_ids(category)?;
        for id in ids {
            println!("{}", id);
        }
        return Ok(()); // Exit after listing
    }

    // Handle subcommands if no list/palette option is present
    // Handle positional argument for set or default get behavior
    if let Some(id) = cli.id {
        let new_id = input_source_manager::set_input_source(&id)?;
        println!("{}", new_id);
    } else {
        // Default behavior: get current input source if no command is specified
        let current_id = input_source_manager::get_current_input_source_id()?;
        println!("{}", current_id);
    }

    Ok(())
}
