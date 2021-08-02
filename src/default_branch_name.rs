use crate::repository::remote_callbacks;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use git2::BranchType::Local;
use git2::Direction::Fetch;
use git2::Repository;

pub fn default_branch_name(repo: &Repository) -> Result<String> {
    let found_master = repo.find_branch("master", Local).is_ok();
    let found_main = repo.find_branch("main", Local).is_ok();

    if found_main && !found_master {
        return Ok("main".to_string());
    }
    if found_master && !found_main {
        return Ok("master".to_string());
    }

    // TODO(btc): if found both, choose the one with more commits?

    let mut remote = repo
        .find_remote("origin")
        .context("error getting info about remote `origin`")?;

    let cb = remote_callbacks();
    remote.connect_auth(Fetch, Some(cb), None)?;

    let remote_default_branch = remote
        .default_branch()?
        .as_str()
        .ok_or(anyhow!("unable to obtain remote default branch name"))?
        .strip_prefix("refs/heads/")
        .context("malformed reference name. expected a prefix of refs/heads/")?
        .to_string();

    Ok(remote_default_branch)
}

#[cfg(test)]
mod test {
    use crate::create_branch::create_branch_here;

    use super::*;

    #[test]
    fn test_lookup_fails_if_both_branches_exist_and_no_origin() -> Result<()> {
        let (_td, repo) = crate::test::repo_init();
        assert_eq!("main", default_branch_name(&repo)?);

        create_branch_here(&repo, "master")?;
        assert!(default_branch_name(&repo).is_err());

        Ok(())
    }
}
