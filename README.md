# Import SVN Commits

A tool to import a summary of commits from private SVN repositories to a 
git repository.
Can be used to show your programming activity on a public Git server, 
without disclosing details about the actual work performed.
The SVN Server needs to be accessible over HTTP and DAV.
Only tested with VisualSVN Server.

Inspired by alexandear/import-gitlab-commits.

## Getting Started

1. Ensure that rust is installed. This tool has been built on rustc 1.78.
2. Clone this repo and cd to the folder.
3. Build it:

```shell
cargo build
```

4. Run it, see the help:

```shell
./target/debug/import-svn-commits --help
```
5. Here's an example invocation:

```shell
mkdir ./test 
./target/debug/import-svn-commits -s https://demo-server.visualsvn.com/ -c xhmikosr -c XhmikosR -d ./test -n you -e you@example.com tortoisesvn
```

For a private server that requires authentication, the user and the "uses 
password" flag will probably be necessary.

Multiple committer names can be specified by repeating the `-c` parameter.
If no SVN Repo Names are specified, the tool will try to retrieve the list of 
all the repositories on the server. This only works if the server is configured
to allow it (`SVNListParentPath On` for mod_dav_svn).

A typical invocation for all the repos on a private server can be:

```shell
import-svn-commits -s https://yourserver.lan/ -u your_username -p -c your_username -c your_old_username -d ./svn_to_git -n you -e you@example.com
```


## Pushing to a git server

Once processing is finished, the local repository can be pushed to a public
remote.

For GitHub and similar platforms, the New Repo landing page has all the 
instructions under the *push an existing repository from the command line*
heading. Switch to the path that was specified for the --destination-repo 
parameter and run:

```shell
git remote add origin git@github.com:you/private_contributions.git
git branch -M main
git push -u origin main
```
