mod commandline;
mod xmlrepolist;
mod commitlog;
mod password;
mod http;
mod git;

use clap::Parser;
use std::error::Error;
use password::read_user_password;

use crate::commandline::Args;
use crate::commitlog::process_repos;
use crate::xmlrepolist::parse_repo_list;
use crate::http::get_repo_list;
use crate::git::open_or_create_git_repo;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let repo = open_or_create_git_repo(&args)?;

    let svn_password = read_user_password(args.svn_use_password)?;

    let urls = if args.svn_repo_names.is_some() {
        // get the repo list from the command line
        args.svn_repo_names.clone().unwrap()
    }
    else
    {
        // try to decode the repo list from the XML index
        let repo_list = get_repo_list(
            &args.svn_url, 
            &args.svn_username, 
            args.svn_use_password,
            &svn_password
        )?;
        parse_repo_list(repo_list)?
    };

    let statistics = process_repos(
        &args, 
        &repo,
        &svn_password, 
        urls,
    )?;

    println!("Processed repositories:");
    for stat in statistics {
        println!("{}\t{}\t{}", stat.repo_id, &stat.repo_name, stat.commit_count);
    }

    Ok(())
}
