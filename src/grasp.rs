use crate::default_branch_name::default_branch_name;
use anyhow::anyhow;
use anyhow::Result;
use git2::{Repository, StatusOptions};

pub fn grasp(repo: Repository) -> Result<()> {
    if !is_clean(&repo)? {
        return Err(anyhow!("repo must be clean"));
    }

    let default = default_branch_name(&repo)?;
    switch(&repo, &default)?;

    fetch(&repo, "origin", &default)?;

    rebase_current_branch_upstream(&repo)?;

    /* TODO
    rebase default on origin/default
    checkout -
    rebase on default
     */
    Ok(())
}

fn rebase_current_branch_upstream(repo: &Repository) -> Result<()> {
    let mut rebase = repo.rebase(None, None, None, None)?;
    loop {
        let maybe = rebase.next();
        if maybe.is_none() {
            rebase.finish(None)?;
            return Ok(());
        }
        let op = maybe.unwrap()?;
        rebase.finish(None)?;
    }
}

fn fetch(repo: &Repository, remote: &str, branch: &str) -> Result<()> {
    let refspecs = &[branch];
    repo.find_remote(remote)?.fetch(refspecs, None, None)?;
    Ok(())
}

fn switch(repo: &Repository, branch: &str) -> Result<()> {
    let reference = repo
        .resolve_reference_from_short_name(branch)?
        .name()
        .ok_or(anyhow!("failed to resolve reference"))?
        .to_string();
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