use anyhow::anyhow;
use anyhow::Context;
use git2::BranchType::Local;
use git2::Repository;

pub fn default_branch_name(repo: &Repository) -> anyhow::Result<String> {
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