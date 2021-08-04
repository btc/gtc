use std::process::Command;

use anyhow::Result;
use anyhow::{anyhow, Context};
use git2::{AnnotatedCommit, FetchOptions, Repository, Signature, StatusOptions};

use crate::default_branch_name::default_branch_name;
use crate::repository::remote_callbacks;
use crate::switch::switch;

pub fn grasp(repo: &Repository) -> Result<()> {
    // to avoid complexity, ensure repo is clean
    if !is_clean(&repo)? {
        return Err(anyhow!("repo must be clean"));
    }

    let default_branch = default_branch_name(&repo)?;

    // hold onto the name of the current branch so we can switch to it after updating the default branch
    let current_branch = current_branch_name(&repo)?;

    let remote = "origin"; // TODO: parameterize
    let has_remote = repo.find_remote(remote).is_ok();

    if has_remote {
        // apply remote updates to the default branch

        switch(&repo, &default_branch)?;
        let upstream = &format!("{}/{}", remote, default_branch);
        fetch(&repo, remote, &default_branch)
            .context(format!("failed to fetch updates from {} branch", upstream))?;
        rebase(&repo, upstream).context(format!(
            "failed to rebase '{}' on '{}'",
            default_branch, upstream
        ))?;
    }

    // apply default-branch updates to branch of interest

    switch(&repo, &current_branch)?;
    rebase(&repo, &default_branch)?;

    Ok(())
}

fn current_branch_name(repo: &Repository) -> Result<String> {
    let head = repo.head()?;
    return head.name().context("HEAD has no name").map(String::from);
}

fn rebase(repo: &Repository, upstream: &str) -> Result<()> {
    rebase_exec(repo, upstream)
}

fn rebase_exec(repo: &Repository, upstream: &str) -> Result<()> {
    let path = repo.path().parent().context("failed to locate git repo")?;

    let status = Command::new("git")
        .arg("rebase")
        .arg(upstream)
        .current_dir(path)
        .output()?
        .status;
    if !status.success() {
        return Err(anyhow!("failed to rebase"));
    }

    Ok(())
}

#[allow(dead_code)]
fn rebase_libgit(repo: &Repository, upstream: &str) -> Result<()> {
    let mut opts = Default::default();
    let sig = Signature::now("gtc", "gtc@example.com").unwrap();

    let upstream_commit = annotated_commit_from_shortname(&repo, upstream)?;

    let mut rebase = repo.rebase(None, Some(&upstream_commit), None, Some(&mut opts))?;
    loop {
        let maybe = rebase.next();
        if maybe.is_none() {
            rebase.finish(None).context("rebase failed on finish")?;
            return Ok(());
        }
        let _ = maybe.unwrap().context("failed rebase operation")?;

        let _ = rebase.commit(None, &sig, None)?;
    }
}

fn annotated_commit_from_shortname<'repo>(
    repo: &'repo Repository,
    shortname: &str,
) -> Result<AnnotatedCommit<'repo>> {
    let commit = repo
        .resolve_reference_from_short_name(shortname)?
        .name()
        .context("failed to resolve reference name")
        .and_then(|refname| Ok(repo.refname_to_id(refname)?))
        .and_then(|oid| Ok(repo.find_annotated_commit(oid)?))?;
    return Ok(commit);
}

fn fetch(repo: &Repository, remote: &str, branch: &str) -> Result<()> {
    let mut fetch_options = FetchOptions::default();
    fetch_options.remote_callbacks(remote_callbacks());

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
    use crate::create_branch::create_branch_in_sequence;
    use crate::switch::switch;
    use crate::test::commit_a_file;

    use super::*;

    #[test]
    fn test_grasp() -> Result<()> {
        let filename = "foo";

        let (_td, repo) = crate::test::repo_init();
        let default_branch = default_branch_name(&repo)?;

        let feature_branch = create_branch_in_sequence(&repo)?;
        switch(&repo, &default_branch)?;
        commit_a_file(&repo, filename)?;

        switch(&repo, &feature_branch)?;
        assert_eq!(false, file_exists(&repo, filename)?);

        grasp(&repo)?;
        switch(&repo, &feature_branch)?;
        assert!(file_exists(&repo, filename)?);

        Ok(())
    }

    fn file_exists(repo: &Repository, filename: &str) -> Result<bool> {
        let exists = repo
            .path()
            .parent()
            .context("failed to get repo path")
            .map(|p| p.join(filename))?
            .exists();
        Ok(exists)
    }
}