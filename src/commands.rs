use clap::{Parser, Subcommand};

pub async fn figure() -> anyhow::Result<String> {
    let cli = Cli::parse();

    let result: anyhow::Result<String> = match cli.command {
        Some(Commands::Markdown) => Ok(clap_markdown::help_markdown::<Cli>()),
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
    /// [STABLE] print markdown doc of qwit to std out
    Markdown,
}
