#![allow(unused_variables)]
use clap::{Args, Parser, Subcommand};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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
    Delete {
        /// Name row to delete
        ip: String,
        name: String,
    },
}

#[derive(Args)]
struct Add {
    ip: String,
    name: String,
}

const FILEPATH: &str = "/etc/hosts";

fn main() {
    about();
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Commands::Add(payload) => {
            if !check_lines(payload) {
                add_line(payload);
            }
        }
        Commands::Delete { name, ip } => {
            println!("It's live {} {}", name, ip)
        }
    }
}

fn check_lines(payload: Add) -> bool {
    // Open host file
    let file = match File::open(FILEPATH) {
        Ok(file) => file,
        Err(e) => {
            println!("Impossible d'ouvrir le fichier hosts: {}", e);
            return false;
        }
    };

    let reader = BufReader::new(file);

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.contains(&payload.name) {
            println!(
                "Une entrée existe déja pour : {} à la ligne {}",
                payload.name, line_number
            );
            return true;
        }
    }
    return false;
}

fn add_line(payload: Add) {
    let file = match File::open(FILEPATH) {
        Ok(file) => file,
        Err(e) => {
            println!("Impossible d'ouvrir le fichier hosts: {}", e);
            return;
        }
    };
}
