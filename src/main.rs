mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "spl-token")]
#[command(about = "CLI took to create, mint and transfer SPL tokens in Solana", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        symbol: String,
        #[arg(short, long)]
        decimals: u64,
    },
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
        Commands::Create {
            name,
            symbol,
            decimals,
        } => commands::create::create(name, symbol, decimals),
        Commands::Mint {
            token,
            amount,
            recipient,
        } => commands::mint::mint(token, amount, recipient),
        Commands::Transfer {
            token,
            amount,
            recipient,
        } => commands::transfer::transfer(token, amount, recipient),
    }
}
