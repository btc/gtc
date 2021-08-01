use anyhow::anyhow;
use git2::Repository;

use crate::default_branch_name::default_branch_name;

pub fn checkout_default_branch(repo: &Repository) -> anyhow::Result<()> {
    let name = default_branch_name(&repo)?;
    let refname = repo
        .resolve_reference_from_short_name(&name)?
        .name()
        .map(String::from)
        .ok_or(anyhow!("failed to resolve reference"))?;
    repo.set_head(&refname)?;
    Ok(())
}
