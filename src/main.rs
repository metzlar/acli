use clap::{Parser, Subcommand};

mod chatgpt;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}


#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Print an AI generated CLI command
    Do{
        /// Explain to the AI what you want to do, example: `acli do "list all .png files in the current dir"`
        text: String,
    },
    /// Ask AI to explain a CLI command
    Explain {
        /// The CLI command, example: `acli explain "find -name *.png -print"`
        text: String,
    },
}


#[tokio::main]
async fn main() {
    let args = Args::parse();

    let output = match args.cmd {
        Commands::Do { text } => chatgpt::r#do(&text).await,
        Commands::Explain { text } => chatgpt::explain(&text).await,
    }.unwrap();

    println!("{}", output);
}
