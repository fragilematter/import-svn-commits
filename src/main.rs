use crate::commandline::Args;
use clap::Parser;

mod commandline;

fn main() {
    let args = Args::parse();

    println!("Hello, world! {}", args.svn_url);
}
