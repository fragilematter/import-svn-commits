use std::error::Error;
use git2::Repository;
use url::Url;
use secrecy::SecretString;
use roxmltree::Document;

use crate::commandline::Args;
use crate::http::report_commit_log;
use crate::xmlrepolist::get_last_version;
use crate::git::git_commit;

pub struct CommitSummary {
    pub repo_id: u32,
    pub repo_name: String,
    pub commit_count: u32,
}

pub fn process_repos(
    args: &Args,
    repo: &Repository,
    password: &Option<SecretString>,
    repo_list: Vec<String>,
) -> Result<Vec<CommitSummary>, Box<dyn Error>> {
    let mut commit_statistics: Vec<CommitSummary> = Vec::new();
    let mut repo_number: u32 = 0;

    for repo_name in repo_list.iter() {
        repo_number += 1;

        let repo_url = args.svn_url.join(repo_name).unwrap();
        let commits = process_repo(
            &args,
            &repo,
            &repo_url, 
            password,
            repo_number,
        )?;
        
        commit_statistics.push(CommitSummary {
            repo_id: repo_number,
            repo_name: repo_name.to_string(),
            commit_count: commits,
        });
    }

    Ok(commit_statistics)
}

fn process_repo(
    args: &Args,
    repo: &Repository,
    repo_url: &Url, 
    password: &Option<SecretString>,
    repo_number: u32,
) -> Result<u32, Box<dyn Error>> {
    const REVISION_BATCH_SIZE: u32 = 1000;

    let last_revision: u32 = get_commit_log_tail(
        repo_url, 
        &args.svn_username, 
        args.svn_use_password, 
        password, 
        0,
        1,
    )?.parse()?;

    println!("Processing {}, found head rev {}", &repo_url, last_revision);

    let mut processed_rev_count: u32 = 0;

    for start_revision in (0..last_revision).step_by(REVISION_BATCH_SIZE.try_into()?) {
        processed_rev_count += process_commit_log(
            &args,
            &repo,
            repo_url, 
            password,
            repo_number,
            start_revision, 
            REVISION_BATCH_SIZE)?;
    }

    Ok(processed_rev_count)
}

fn get_commit_log_tail(
    repo_url: &Url, 
    username: &Option<String>,
    use_password: bool,
    password: &Option<SecretString>,
    end_revision: u32,
    limit: u32,
) -> Result<String, Box<dyn Error>> {
    let last_commit_xml = report_commit_log(
        repo_url, 
        username, 
        use_password, 
        password, 
        format!("<S:log-report xmlns:S=\"svn:\"><S:end-revision>{end_revision}</S:end-revision><S:limit>{limit}</S:limit><S:encode-binary-props /><S:path></S:path></S:log-report>")
    )?;

    let last_commit = get_last_version(last_commit_xml)?;

    Ok(last_commit)
}

fn process_commit_log(
    args: &Args,
    repo: &Repository,
    repo_url: &Url, 
    password: &Option<SecretString>,
    repo_number: u32,
    start_revision: u32,
    limit: u32,
) -> Result<u32, Box<dyn Error>> {
    let commit_log = report_commit_log(
        repo_url, 
        &args.svn_username, 
        args.svn_use_password, 
        password, 
        format!("<S:log-report xmlns:S=\"svn:\"><S:start-revision>{start_revision}</S:start-revision><S:limit>{limit}</S:limit><S:encode-binary-props /><S:path></S:path></S:log-report>")
    )?;

    let mut processed_rev_count = 0;
    let doc = Document::parse(&commit_log)?;
    let committer_names: Vec<String> = if args.svn_committers.is_some() {
        args.svn_committers.clone().unwrap()
    } else {
        vec![args.svn_username.clone().expect("SVN Committers and SVN Username are both empty!")]
    };

    for log_item in doc.descendants().filter(|n| n.tag_name().name() == "log-item") {
        // Extract <D:creator-displayname> value
        if let Some(creator_displayname) = log_item
            .descendants()
            .find(|n| n.tag_name().name() == "creator-displayname")
            .and_then(|n| n.text())
        {
            // Check if the creator is in the provided Vec<String>
            if committer_names.contains(&creator_displayname.to_string()) {
                // Extract <D:version-name> and <S:date> values
                if let (Some(version_name), Some(date)) = (
                    log_item
                        .descendants()
                        .find(|n| n.tag_name().name() == "version-name")
                        .and_then(|n| n.text()),
                    log_item
                        .descendants()
                        .find(|n| n.tag_name().name() == "date")
                        .and_then(|n| n.text()),
                ) {
                    git_commit(&args, &repo, repo_number, version_name, date)?;
                    processed_rev_count += 1;
                }
            }
        }
    }

    Ok(processed_rev_count)
}
