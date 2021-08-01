use anyhow::anyhow;
use git2::Repository;

pub fn switch(repo: &Repository, branch: &str) -> anyhow::Result<()> {
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
