use std::path::PathBuf;

use crate::probe;
use clap::{Parser, Subcommand};

use super::init;

pub async fn figure() -> anyhow::Result<String> {
    let cli = Cli::parse();

    let result: anyhow::Result<String> = match cli.command {
        Some(Commands::Markdown) => Ok(clap_markdown::help_markdown::<Cli>()),
        Some(Commands::Init {}) => init::example(),
        Some(Commands::Check { file, json }) => probe::all(file, json).await,
        Some(Commands::Generate { _ci, _web }) => Err(anyhow::anyhow!("NOT_IMPLENTED".to_string())),
        Some(Commands::ServerMode {}) => Err(anyhow::anyhow!("NOT_IMPLENTED".to_string())),
        None => Ok("try hy --help for information on how to use hay".to_string()),
    };

    result
}

/// hay cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about, name = "hy")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// [STABLE] print markdown doc of hay to std out
    Markdown,
    /// [STABLE] creates an example config
    Init {},
    /// [PREVIEW] checks all endpoints
    Check {
        file: Option<PathBuf>,
        #[arg(short, long, default_value_t = false)]
        json: bool,
    },
    /// [NOT IMPLEMENTED] generates ci actions to create that checks endpoints and updates website
    Generate {
        #[arg(short, long, default_value_t = true)]
        _ci: bool,
        #[arg(short, long, default_value_t = false)]
        _web: bool,
    },
    /// [NOT IMPLEMENTED] lets the ci run in server mode, continously checks endpoints
    ServerMode {},
}
