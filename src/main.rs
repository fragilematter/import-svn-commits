use crate::commandline::Args;
use clap::Parser;
use xmlrepolist::{get_repo_list, parse_repo_list};
use std::error::Error;

mod commandline;
mod xmlrepolist;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let urls = if args.svn_repo_names.is_some() {
        // get the repo list from the command line
        args.svn_repo_names.unwrap()
    }
    else
    {
        // try to decode the repo list from the XML index
        let repo_list = get_repo_list(args.svn_url, args.svn_username, args.svn_password)?;
        parse_repo_list(repo_list)?
    };

    dbg!(urls);

    Ok(())
}
