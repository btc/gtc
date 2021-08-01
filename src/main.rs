/*
 * TODO: grasp
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

use anyhow::Context;
use anyhow::Result;
use clap::{AppSettings, Clap};
use git2::Repository;

mod checkout_default_branch;
mod cleanup_branches;
mod create_branch;
mod default_branch_name;

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let cwd = std::env::current_dir().context("unable to obtain PWD")?;
    let repo = Repository::discover(cwd).context("failed to open repo")?;
    match opts.subcmd {
        SubCommand::DefaultBranchName => {
            let name = default_branch_name::default_branch_name(&repo)
                .context("failed to obtain name of default branch")?;
            println!("{}", name);
        }
        SubCommand::CreateBranch => {
            let name = create_branch::create_branch(&repo)?;
            println!("{}", name);
        }
        SubCommand::CheckoutDefaultBranch => {
            checkout_default_branch::checkout_default_branch(&repo)?;
        }
        SubCommand::CleanupBranches { dry_run } => {
            if dry_run {
                println!("dry run...")
            }
            cleanup_branches::cleanup_branches(repo, dry_run)?;
        }
    }
    Ok(())
}

/// gtc is a git powertool
#[derive(Clap)]
#[clap(version = "1.0", author = "btc <btc@no.reply.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    DefaultBranchName,
    #[clap(visible_alias = "branch-random")]
    CreateBranch,
    #[clap(visible_alias = "m")]
    CheckoutDefaultBranch,
    CleanupBranches {
        #[clap(short)]
        dry_run: bool,
    },
}

#[derive(Clap)]
struct CleanupBranches {
    #[clap(short)]
    dry_run: bool,
}