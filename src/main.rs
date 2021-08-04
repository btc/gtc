/*
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
use clap::{AppSettings, ArgSettings, Clap};
use git2::Repository;

mod checkout_default_branch;
mod cleanup_branches;
mod create_branch;
mod default_branch_name;
mod grasp;
mod repository;
mod switch;
mod test;
mod update_pulls;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    let opts: Opts = Opts::parse();

    let cwd = std::env::current_dir().context("unable to obtain PWD")?;
    let repo = Repository::discover(cwd).context("failed to open repo")?;
    match opts.subcmd {
        SubCommand::UpdatePulls => {
            update_pulls::update_pulls(opts, &repo)?;
        }
        SubCommand::DefaultBranchName => {
            let name = default_branch_name::default_branch_name(&repo)
                .context("couldn't figure out name of default branch")?;
            println!("{}", name);
        }
        SubCommand::CreateBranch => {
            let name = create_branch::create_branch_in_sequence(&repo)?;
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
        SubCommand::Grasp => {
            grasp::grasp(&repo)?;
        }
    }
    Ok(())
}

/// gtc is a git powertool
#[derive(Clap)]
#[clap(version = "1.0", author = "btc <btc@no.reply.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,

    #[clap(long, env = "GITHUB_TOKEN", setting = ArgSettings::HideEnvValues)]
    token: String,
}

#[derive(Clap)]
enum SubCommand {
    UpdatePulls,
    Grasp,
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