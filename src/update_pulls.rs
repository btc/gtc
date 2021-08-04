use crate::Opts;
use anyhow::anyhow;
use anyhow::{Context, Result};
use git2::Repository;
use github_rs::client::{Executor, Github};
use itertools::Itertools;
use serde::Deserialize;


pub fn update_pulls(opts: Opts, repo: &Repository) -> Result<()> {
    let client = Github::new(opts.token).map_err(to_anyhow)?;
    let remote = repo.find_remote("origin")?;
    let url = remote.url().context("url is missing")?;
    println!("{}", url);
    let (owner, repository) = parse_github_ssh_url(url).context(anyhow!("unrecognized URL"))?;

    let (_, _, pulls) = client
        .get()
        .repos()
        .owner(&owner)
        .repo(&repository)
        .pulls()
        .execute::<Vec<PullResponse>>()
        .map_err(to_anyhow)?;

    for pull in pulls.ok_or(anyhow!("hmm"))? {
        println!("{:?}", pull.base);
        println!("{:?}", pull.head);
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct PullResponse {
    pub number: i32,
    pub title: String,
    pub locked: bool,
    pub draft: bool,
    pub base: BranchResponse,
    pub head: BranchResponse,
}

#[derive(Debug, Deserialize)]
struct BranchResponse {
    pub label: String,
    pub r#ref: String,
    pub sha: String,
}

pub fn to_anyhow<E: ToString>(e: E) -> anyhow::Error {
    anyhow!(e.to_string())
}

fn parse_github_ssh_url<S: AsRef<str>>(url: S) -> Result<(String, String)> {
    let (owner, repo) = url
        .as_ref()
        .strip_prefix("git@github.com:")
        .context("unrecognized prefix")?
        .strip_suffix(".git")
        .context("unrecognized suffix")?
        .splitn(2, "/")
        .tuples()
        .next()
        .context("malformed url")?;
    Ok((owner.into(), repo.to_string()))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse() -> Result<()> {
        let test_cases = vec![
            ("git@github.com:btc/gtc.git", ("btc", "gtc")),
            ("git@github.com:btc/noop.git", ("btc", "noop")),
        ];
        for (input, (expected_owner, expected_repo)) in test_cases {
            let (owner, repo) = parse_github_ssh_url(input)?;
            assert_eq!(expected_owner, owner);
            assert_eq!(expected_repo, repo);
        }
        Ok(())
    }
}