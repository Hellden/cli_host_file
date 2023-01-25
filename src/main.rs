#![allow(unused_variables)]
use clap::{Args, Command, Parser, Subcommand};

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
    /// Add line at host file
    Add(Add),
    /// Delete line specifie of host file
    Delete(Delete),
}

#[derive(Args)]
struct Add {
    /// Name add host
    name: String,
    /// Ip linked at name host
    ip: String,
}
#[derive(Args)]
struct Delete {
    /// Name host at delete
    name: String,
    /// Optional: Specifiy ip address at name host
    ip: Option<String>,
}

fn main() {
    about();
    // ###############################
    let cli = Cli::parse();

    match cli.commands {
        Commands::Add => {
            println!("adds")
        }
    }
}
