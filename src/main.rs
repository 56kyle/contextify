use std::path::{PathBuf};
use clap::{Parser, Subcommand};
use crate::summary::create_directory_summary;

mod summary;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>
}


#[derive(Subcommand)]
pub enum Commands {
    /// Summarizes the structure of the provided path
    #[command(arg_required_else_help = true)]
    Structure {
        /// Root path that will be used
        #[arg(required = true)]
        path: PathBuf,

        /// Output path for generated structure summary
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,

        /// Whether paths are searched recursively
        #[arg(short, long, default_value_t = 8)]
        depth: usize,
    },

    /// Summarizes the content of the provided Paths
    #[command(arg_required_else_help = true)]
    Content {
        /// Paths that will be summarized
        path: Vec<PathBuf>,

        /// Output path for generated content summary
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,

        /// Whether paths are searched recursively
        #[arg(short, long)]
        depth: usize,
    }
}

fn main() {
    println!("Hello, world!");
    let args = Cli::parse();
    match args.command {
        Some(Commands::Structure{
            path,
            output,
            depth
        }) => {
            let output_path: PathBuf = output.unwrap_or(
                std::env::current_dir()
                    .unwrap()
                    .join("structure.txt")
            );
            let _ = create_directory_summary(path, output_path, depth);
        },
        Some(Commands::Content {
            ..
        }) => {
        },
        _ => {}
    }
}
