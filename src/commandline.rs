use clap::Parser;
use url::Url;
use email_address::EmailAddress;
use camino::Utf8PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// SVN Server URL
    #[arg(short = 'u', long)]
    pub svn_url: Url,

    /// SVN username
    #[arg(short, long)]
    pub svn_username: String,

    /// Path to destination git repo
    #[arg(short, long)]
    pub destination_repo: Utf8PathBuf,

    /// Committer name (Github name surname)
    #[arg(short = 'n', long)]
    pub committer_name: String,

    /// Committer email address
    #[arg(short = 'e', long)]
    pub committer_email: EmailAddress,

    /// Optional repositories relative to SVN Server URL
    #[command()]
    pub svn_repo_names: Option<Vec<String>>,
}
