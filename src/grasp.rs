use std::env;
use std::process::Command;

use anyhow::{anyhow, Context};
use anyhow::Result;
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository, StatusOptions};

use crate::default_branch_name::default_branch_name;
use crate::switch;

pub fn grasp(repo: &Repository) -> Result<()> {
    if !is_clean(&repo)? {
        return Err(anyhow!("repo must be clean"));
    }

    let default = default_branch_name(&repo)?;
    switch::switch(&repo, &default)?;

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

fn rebase_exec(repo: &Repository, base_branch: &str) -> Result<()> {
    let path = repo.path().parent().context("failed to locate git repo")?;

    let status = Command::new("git")
        .arg("rebase")
        .arg(base_branch)
        .current_dir(path)
        .output()?
        .status;
    if !status.success() {
        return Err(anyhow!("failed to rebase"));
    }

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

fn is_clean(repo: &Repository) -> Result<bool> {
    Ok(dirty_files(&repo)?.is_empty())
}

fn dirty_files(repo: &Repository) -> Result<Vec<String>> {
    let mut opts = StatusOptions::new();
    opts.include_ignored(false);
    let statuses = repo.statuses(Some(&mut opts))?;
    let mut vec = Vec::new();
    for s in statuses.iter() {
        let p = s.path().context("path is missing")?;
        vec.push(p.to_string());
    }
    Ok(vec)
}

#[cfg(test)]
mod test {
    use crate::create_branch::create_branch;
    use crate::switch;
    use crate::test::commit_a_file;

    use super::*;

    #[test]
    fn test_grasp() -> Result<()> {
        let (_td, repo) = crate::test::repo_init();
        let default = default_branch_name(&repo)?;

        let new_branch = create_branch(&repo)?;

        switch::switch(&repo, &default)?;
        commit_a_file(&repo, "foo")?;

        switch::switch(&repo, &new_branch)?;
        assert_eq!(
            Vec::<String>::new(),
            dirty_files(&repo)?,
            "committed files from previous branch are present after switch",
        );
        assert!(is_clean(&repo)?);

        assert!(is_clean(&repo)?);

        Ok(())
    }
}