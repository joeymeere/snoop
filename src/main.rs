pub mod commands;
pub mod utils;
use anyhow::Error;
use clap::{Args, Parser, Subcommand};
use commands::{cfg, disassemble, functions, graph, instructions};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Disassemble a compiled program binary")]
    Disassemble(DisassembleArgs),
    #[command(about = "Generate a graphviz of a given program binary")]
    Graph(GraphArgs),
    #[command(about = "Inspect functions in the binary")]
    Functions(GenericArgs),
    #[command(about = "Inspect instructions in the binary")]
    Instructions(GenericArgs),
    #[command(about = "View the program's config")]
    Cfg(GenericArgs),
}

#[derive(Args)]
struct DisassembleArgs {
    path: Option<String>,
    #[arg(short = 'o', long = "output")]
    outfile: Option<String>,
}

#[derive(Args)]
struct GraphArgs {
    path: Option<String>,
    #[arg(short = 'o', long = "output")]
    outfile: Option<String>,
}

#[derive(Args)]
struct GenericArgs {
    path: Option<String>,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Disassemble(args) => disassemble(args.path.clone(), args.outfile.clone()),
        Commands::Graph(args) => graph(args.path.clone()),
        Commands::Functions(args) => functions(args.path.clone()),
        Commands::Instructions(args) => instructions(args.path.clone()),
        Commands::Cfg(args) => cfg(args.path.clone()),
    }
}
