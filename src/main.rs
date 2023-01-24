#![allow(unused_variables)]

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Default
    #)]
    add: String,
}

fn main() {
print!("##### Welcome cli-host_file #####");
println!("
     _~^~^~_
  \\) /  o o  \\ (/
    '_   -   _'
    / '-----' \\
");
    let cli = Cli::parse();
}
