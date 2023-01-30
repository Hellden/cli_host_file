#![allow(unused_variables)]
use clap::{Args, Parser, Subcommand};

fn about() {
    print!("##### Welcome cli-host_file #####");
    println!(
        "
      _~^~^~_
  \\) /  o o  \\ (/
    '_   -   _'
    / '-----' \\
"
    );
    println!("Author : Hellden\n\n");
}

#[derive(Parser)]
#[command(version, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Add row in host file
    Add(Add),
    Delete{
        /// Name row to delete
        name:String,
        ip:String
    }
}

#[derive(Args)]
struct Add {
    name: String,
    ip: String
}


fn main() {
    about();
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Commands::Add(payload) => {
            println!("'myapp add' was used, name is: {} {}", payload.name, payload.ip)
        },
        Commands::Delete { name, ip } => {
            println!("It's live {} {}", name, ip)
        }
    }
}