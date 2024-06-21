use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// SVN Server URL
    #[arg(short, long)]
    pub svn_url: Url,
}
