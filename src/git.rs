use git2::{Repository, Signature, Time};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use std::error::Error;

use crate::Args;

pub fn open_or_create_git_repo(args: &Args) -> Result<Repository, Box<dyn Error>> {
    let repo_path = &args.destination_repo;

    match Repository::open(repo_path) {
        Ok(repo) => {
            println!("Repository exists at {}", repo_path);
            Ok(repo)
        },
        Err(_) => {
            println!("No repository found at {}. Initializing a new repository...", repo_path);
            match Repository::init(repo_path) {
                Ok(repo) => {
                    println!("Repository created at {}", repo_path);
                    create_initial_commit(&repo, &args)?;
                    Ok(repo)
                },
                Err(e) => Err(Box::new(e))
            }
        }
    }
}

// adapted from git2-rs examples/init.rs
// https://github.com/rust-lang/git2-rs/blob/master/examples/init.rs
fn create_initial_commit(repo: &Repository, args: &Args) -> Result<(), Box<dyn Error>> {
    // First use the config to initialize a commit signature for the user.
    let now = OffsetDateTime::now_utc();

    let sig = Signature::new(
        &args.committer_name,
        args.committer_email.as_str(),
        &Time::new(now.unix_timestamp(), now.offset().whole_seconds())
    )?;

    // Now let's create an empty tree for this commit
    let tree_id = {
        let mut index = repo.index()?;

        // Outside of this example, you could call index.add_path()
        // here to put actual files into the index. For our purposes, we'll
        // leave it empty for now.

        index.write_tree()?
    };

    let tree = repo.find_tree(tree_id)?;

    // Ready to create the initial commit.
    //
    // Normally creating a commit would involve looking up the current HEAD
    // commit and making that be the parent of the initial commit, but here this
    // is the first commit so there will be no parent.
    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit by fragilematter/import-svn-commits", &tree, &[])?;

    Ok(())
}

pub fn git_commit(
    args: &Args,
    repo: &Repository,
    repo_number: u32,
    revision: &str,
    date: &str,
) -> Result<(), Box<dyn Error>> {
    let commit_date = OffsetDateTime::parse(date, &Rfc3339)?;

    // Retrieve the current HEAD reference
    let head_ref = repo.head()?;
    let parent_commit = head_ref.peel_to_commit()?;

    // Retrieve the tree of the parent commit
    let tree = parent_commit.tree()?;
    let time = Time::new(commit_date.unix_timestamp(), commit_date.offset().whole_seconds());
    // Create a signature with a custom date for the commit
    let signature = Signature::new(
        &args.committer_name,
        args.committer_email.as_str(),
        &time
    )?;

    // Create the empty commit
    repo.commit(
        Some("HEAD"),          // Update HEAD to point to the new commit
        &signature,            // Author signature with custom date
        &signature,            // Committer signature with custom date
        &format!("project: {}, commit: {}", repo_number, revision), // Commit message
        &tree,                 // Tree object (same as parent)
        &[&parent_commit],     // Parent commits
    )?;

    Ok(())
}
