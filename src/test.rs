use anyhow::{Context, Result};
use git2::{Repository, RepositoryInitOptions};
use std::fs::File;
use std::path::Path;
use tempfile::TempDir;

#[allow(dead_code)]
pub fn repo_init() -> (TempDir, Repository) {
    let td = TempDir::new().unwrap();
    let mut opts = RepositoryInitOptions::new();
    opts.initial_head("main");
    let repo = Repository::init_opts(td.path(), &opts).unwrap();
    {
        let mut config = repo.config().unwrap();
        config.set_str("user.name", "name").unwrap();
        config.set_str("user.email", "email").unwrap();
        let mut index = repo.index().unwrap();
        let id = index.write_tree().unwrap();

        let tree = repo.find_tree(id).unwrap();
        let sig = repo.signature().unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "initial", &tree, &[])
            .unwrap();
    }
    (td, repo)
}

#[allow(dead_code)]
pub(crate) fn commit_a_file(repo: &Repository, filename: &str) -> Result<()> {

    let mut index = repo.index()?;
    let root = repo
        .path()
        .parent()
        .context("failed to get parent of repo index path")?;
    File::create(&root.join(filename))?;
    index.add_path(Path::new(filename))?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let sig = repo.signature()?;
    let head_id = repo.refname_to_id("HEAD")?;
    let parent = repo.find_commit(head_id)?;
    let _ = repo.commit(Some("HEAD"), &sig, &sig, "commit", &tree, &[&parent])?;
    Ok(())
}