use crate::default_branch_name::default_branch_name;
use anyhow::{anyhow, Context};
use anyhow::Result;
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository, StatusOptions};
use std::env;

pub fn grasp(repo: &Repository) -> Result<()> {
    if !is_clean(&repo)? {
        return Err(anyhow!("repo must be clean"));
    }

    let default = default_branch_name(&repo)?;
    switch(&repo, &default)?;

    fetch(&repo, "origin", &default)
        .context("failed to fetch updates from origin default branch")?;

    rebase_current_branch_upstream(&repo)
        .context("failed to rebase the default branch")?;

    /* TODO
    rebase default on origin/default
    checkout -
    rebase on default
     */
    Ok(())
}

fn rebase_current_branch_upstream(repo: &Repository) -> Result<()> {
    let mut opts = Default::default();
    let mut rebase = repo.rebase(None, None, None, Some(&mut opts))?;
    loop {
        let maybe = rebase.next();
        if maybe.is_none() {
            rebase.finish(None)?;
            return Ok(());
        }
        let _op = maybe.unwrap()?;
        rebase.finish(None)?;
    }
}

fn fetch(repo: &Repository, remote: &str, branch: &str) -> Result<()> {
    let mut remote_callbacks = RemoteCallbacks::default();
    remote_callbacks.credentials(|_, username_from_url, _| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )
    });

    let mut fetch_options = FetchOptions::default();
    fetch_options.remote_callbacks(remote_callbacks);

    let refspecs = &[branch];
    repo.find_remote(remote)?
        .fetch(refspecs, Some(&mut fetch_options), None)?;
    Ok(())
}

fn switch(repo: &Repository, branch: &str) -> Result<()> {
    let reference = repo
        .resolve_reference_from_short_name(branch)?
        .name()
        .ok_or(anyhow!("failed to resolve reference"))?
        .to_string();
    let treeish = repo.revparse_single(reference.as_str())?;
    repo.checkout_tree(&treeish, None)?;
    repo.set_head(&reference)?;
    Ok(())
}

fn is_clean(repo: &Repository) -> Result<bool> {
    let mut opts = StatusOptions::new();
    opts.include_ignored(false);
    let statuses = repo.statuses(Some(&mut opts))?;
    let is_clean = statuses.iter().len() == 0;
    Ok(is_clean)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::create_branch::create_branch;

    #[test]
    fn test_grasp() -> Result<()> {
        let (td, repo) = crate::test::repo_init();
        let name = create_branch(&repo)?;
        // TODO
        Ok(())
    }
}