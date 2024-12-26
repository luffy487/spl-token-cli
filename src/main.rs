mod commands; // Declare the commands module
mod utils; // Declare the utils module

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "spl-token")]
#[command(about = "CLI tool to create, mint, and transfer SPL tokens on Solana", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create,
    Mint {
        #[arg(short, long)]
        token: String,
        #[arg(short, long)]
        amount: u64,
        #[arg(short, long)]
        recipient: String,
    },
    Transfer {
        #[arg(short, long)]
        token: String,
        #[arg(short, long)]
        amount: u64,
        #[arg(short, long)]
        recipient: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create => commands::create::create().await,
        Commands::Mint {
            token,
            amount,
            recipient,
        } => commands::mint::mint(token, amount, recipient).await,
        Commands::Transfer {
            token,
            amount,
            recipient,
        } => commands::transfer::transfer_tokens(token, amount, recipient).await,
    }
}
