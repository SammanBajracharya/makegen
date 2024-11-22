mod args;
mod compiler;
mod generate_content;

use args::Args;
use std::fs::write;
use clap::Parser;
use generate_content::generate_makefile;

fn main() {
    let args = Args::parse();

    let makefile_content = generate_makefile(&args);
    let makefile_path = format!("{}/makefile", args.dir.trim_end_matches('/'));

    if let Err(e) = write(&makefile_path, makefile_content) {
        eprintln!("Error writting Makefile: {}", e);
    } else if args.verbose {
        println!("Makefile generated at {}", makefile_path);
    }
}
