use crate::commandline::Args;
use clap::Parser;
use xmlrepolist::{get_repo_list, parse_repo_list};

mod commandline;
mod xmlrepolist;

fn main() {
    let args = Args::parse();

    //dbg!(args);
    //println!("Hello, world! {}", args.svn_url);
    let repo_list = get_repo_list(args.svn_url);

    if repo_list.is_some() {
        parse_repo_list(repo_list.unwrap());
    }
}
