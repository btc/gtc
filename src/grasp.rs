use anyhow::anyhow;
use anyhow::Result;
use git2::RepositoryState::Clean;
use git2::{Repository, StatusOptions};

pub fn grasp(repo: Repository) -> Result<()> {
    if !is_clean(repo)? {
        return Err(anyhow!("repo must be clean"));
    }
    /* TODO
    checkout default
    fetch
    rebase default on origin/default
    checkout -
    rebase on default
     */
    Ok(())
}

fn is_clean(repo: Repository) -> Result<bool> {
    let mut opts = StatusOptions::new();
    opts.include_ignored(false);
    let statuses = repo.statuses(Some(&mut opts))?;
    let is_clean = statuses.iter().len() == 0;
    Ok(is_clean)
}