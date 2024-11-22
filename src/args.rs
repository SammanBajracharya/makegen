use clap::Parser;
use crate::compiler::Compiler;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, required=true, num_args=1..)]
    pub sources: Vec<String>,

    #[arg(short, long, default_value="output")]
    pub output: String,

    #[arg(short, long, default_value="gcc")]
    pub compiler: Compiler,

    #[arg(short, long, default_value="")]
    pub flags: String,

    #[arg(short, long, default_value="./")]
    pub dir: String,

    #[arg(short, long)]
    pub verbose: bool,
}
