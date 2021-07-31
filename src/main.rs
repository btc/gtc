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
 */

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use clap::{AppSettings, Clap};
use git2::BranchType::Local;
use git2::Repository;

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        DefaultBranchName => {
            let cwd = std::env::current_dir().context("unable to obtain PWD")?;
            let repo = Repository::open(cwd).context("failed to open repo")?;
            let name =
                name_of_default_branch(&repo).context("failed to obtain name of default branch")?;
            println!("{}", name);
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
}

fn name_of_default_branch(repo: &Repository) -> Result<String> {
    let found_master = repo.find_branch("master", Local).is_ok();
    let found_main = repo.find_branch("main", Local).is_ok();

    if found_main && !found_master {
        return Ok("main".to_string());
    }
    if found_master && !found_main {
        return Ok("master".to_string());
    }

    // TODO(btc): if found both, choose the one with more commits?

    let remote_default_branch = repo
        .find_remote("origin")
        .context("error getting info about remote `origin`")?
        .default_branch()?
        .as_str()
        .ok_or(anyhow!("unable to obtain remote default branch name"))?
        .to_string();
    Ok(remote_default_branch)
}

// TODO test different repos flavor, pancake, this one, etc.