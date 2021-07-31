use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
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

fn parse_sequence_number<S: AsRef<str>>(branch_name: S) -> Result<i32> {
    let s = branch_name.as_ref()
        .strip_prefix("btc/")
        .ok_or(anyhow!("name doesn't begin with prefix"))?
        .trim_start_matches("0")
        .splitn(2, "-").take(1)
        .collect::<String>();
    Ok(s.parse::<i32>()?.into())
}

fn next_number_in_sequence(last: i32) -> i32 {
    last + 1 // TODO compute primes
}

mod test {
    use crate::create_branch::*;

    #[test]
    fn test_parse_sequence_number() {
        let cases = vec![
            ("btc/1000", Some(1000)),
            ("btc/0041", Some(41)),
            ("btc/1001", Some(1001)),
            ("main", None),
            ("btc/0001-foobar", Some(1))];
        for (name, expected) in cases {
            let got = parse_sequence_number(name);
            assert_eq!(expected, got.ok());
        }
    }
}