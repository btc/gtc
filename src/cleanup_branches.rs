use git2::Repository;

pub fn cleanup_branches(repo: Repository, dry_run: bool) -> anyhow::Result<()> {
    // ensuyre on defualt branch
    // delete special branches which point to this same commit
    if dry_run {
        return Ok(());
    }
    for _branch in repo.branches(None) {}
    Ok(())
}
