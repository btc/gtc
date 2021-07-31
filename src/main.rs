/*
 * TODO: grasp
 * TODO: create new branch
 * TODO: create new PR
 * TODO: access GitHub API in Rust
 * TODO: update PR chain
 * TODO: rebase PR on main if dependent PR is merged
 *
 * TODO: sync chain
 * TODO: if curr.approved then curr.merge, delete curr.remote -> GOTO curr = next
 *
 * TODO: test different repos flavor, pancake, this one, etc.
 */

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use clap::{AppSettings, Clap};
use git2::Repository;

mod create_branch;
mod default_branch_name;

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::DefaultBranchName => {
            let cwd = std::env::current_dir().context("unable to obtain PWD")?;
            let repo = Repository::discover(cwd).context("failed to open repo")?;
            let name = default_branch_name::default_branch_name(&repo)
                .context("failed to obtain name of default branch")?;
            println!("{}", name);
        }
        SubCommand::CreateBranch => {
            let cwd = std::env::current_dir().context("unable to obtain PWD")?;
            let repo = Repository::discover(cwd).context("failed to open repo")?;
            create_branch::create_branch(&repo)?;
        }
        _ => {}
    }
    Ok(())
}

/// gtc is a git powertool
#[derive(Clap)]
#[clap(version = "1.0", author = "btc <btc@no.reply.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets the path to the git repo
    #[clap(short, long, default_value = ".")]
    path: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    DefaultBranchName,
    CreateBranch,
}