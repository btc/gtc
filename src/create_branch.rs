use anyhow::anyhow;
use anyhow::Context;
use git2::BranchType::Local;
use git2::Repository;

// returns the name of the created branch
pub fn create_branch(repo: &Repository) -> anyhow::Result<String> {
    for branch in repo.branches(Some(Local))? {
        let (b, _) = branch.context("odd error")?;
        let f = b
            .name()?
            .ok_or(anyhow!("failed to unpack branch name"))?
            .to_string();
    }
    // parse strings to int
    // figure out the next branch number
    // parse pattern
    // define tests
    Err(anyhow!("TODO"))
}