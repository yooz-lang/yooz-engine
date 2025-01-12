use std::fs::File;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author="yooz-lang", version="1.0", about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Create{
        input: String
    },
    
}

fn main() {
    let args = Args::parse();
  
    match args.cmd {
        Commands::Create{input} => create_db(input),
    }
}

fn create_db(filename: String){
    let db = File::create(format!("{}.yooz",filename));
}