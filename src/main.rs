use std::io::{self, BufRead};

use clap::{command, Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use urlencoding::{decode, encode};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Percent Encode
    Encode {
        /// Raw content to be percent encoded. If not provided, reads from stdin.
        decoded: Option<String>,
    },
    /// Percent Decode
    Decode {
        /// Encoded content to be percent decoded. If not provided, reads from stdin.
        encoded: Option<String>,
    },
    /// Generate shell completion.
    Completion { shell: Shell },
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn read_from_stdin() -> String {
    let mut handle = io::stdin().lock();
    let mut buffer = String::new();
    handle
        .read_line(&mut buffer)
        .expect("Failed to read line from stdin");
    // remove tailing newline
    buffer.pop();
    buffer
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { decoded } => {
            let decoded = decoded.unwrap_or_else(read_from_stdin);
            let encoded = encode(&decoded);
            print!("{encoded}");
        }
        Commands::Decode { encoded } => {
            let encoded = encoded.unwrap_or_else(read_from_stdin);
            match decode(&encoded) {
                Ok(decoded) => print!("{decoded}"),
                Err(e) => eprintln!("Failed to decode input: {e}"),
            }
        }
        Commands::Completion { shell } => {
            let mut cmd = Cli::command();
            print_completions(shell, &mut cmd);
        }
    }
}
