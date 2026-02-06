use crate::config::CometConfig;
use crate::flags::{FlagCategory, FlagRegistry};
use clap::{ArgEnum, Parser, Subcommand};

#[derive(Copy, Clone, Debug, ArgEnum)]
enum CategoryCli {
    Legal,
    Performance,
    Security,
    WebApi,
}

impl From<CategoryCli> for FlagCategory {
    fn from(c: CategoryCli) -> Self {
        match c {
            CategoryCli::Legal => FlagCategory::Legal,
            CategoryCli::Performance => FlagCategory::Performance,
            CategoryCli::Security => FlagCategory::Security,
            CategoryCli::WebApi => FlagCategory::WebApi,
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "comet-flags")]
#[command(about = "Comet Browser experimental flags controller")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    List {
        #[arg(long)]
        category: Option<CategoryCli>,
    },
    Enable {
        id: String,
    },
    Disable {
        id: String,
    },
}

pub fn run_cli(registry: &mut FlagRegistry, config: &mut CometConfig) {
    config.apply_to_registry(registry);
    let cli = Cli::parse();

    match cli.command {
        Commands::List { category } => {
            let flags = registry.all();
            for f in flags {
                if let Some(cat) = category {
                    if f.category != FlagCategory::from(cat) {
                        continue;
                    }
                }
                println!(
                    "[{}] {} ({}) - {}",
                    if f.enabled { "x" } else { " " },
                    f.title,
                    f.id,
                    f.description
                );
            }
        }
        Commands::Enable { id } => {
            if let Some(flag) = registry.find_mut(&id) {
                flag.enabled = true;
                config.sync_from_registry(registry);
                println!("Enabled flag: {} ({})", flag.title, flag.id);
            } else {
                eprintln!("Flag not found: {}", id);
            }
        }
        Commands::Disable { id } => {
            if let Some(flag) = registry.find_mut(&id) {
                flag.enabled = false;
                config.sync_from_registry(registry);
                println!("Disabled flag: {} ({})", flag.title, flag.id);
            } else {
                eprintln!("Flag not found: {}", id);
            }
        }
    }
}
