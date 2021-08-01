use crate::switch::switch;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use git2::BranchType::Local;
use git2::Repository;
use primes::{PrimeSet, Sieve};

// returns the name of the created branch
pub fn create_branch(repo: &Repository) -> anyhow::Result<String> {
    let mut branches = Vec::new();
    for branch in repo.branches(Some(Local))? {
        let branch_name = branch?
            .0
            .name()?
            .ok_or(anyhow!("failed to unpack branch name"))?
            .to_string();
        branches.push(branch_name);
    }

    // TODO let prefix = "btc";

    let next_branch = next(branches.iter())?;

    let target_commit = repo.head()?.peel_to_commit()?;
    let created = repo.branch(&next_branch, &target_commit, false)?;

    let name = created
        .name()
        .context("failed to obtain name of newly created branch")?
        .ok_or(anyhow!("failed to unwrap created branch name"))?;

    switch(&repo, name)?;
    Ok(name.to_string())
}

fn next<T: AsRef<str>>(branches: impl Iterator<Item = T>) -> Result<String> {
    let mut seq = Vec::<u64>::new();
    &branches.for_each(|branch| {
        if let Ok(number) = parse_sequence_number(branch) {
            &seq.push(number);
        }
    });
    let m = seq.iter().max().unwrap_or(&0);
    let next_num = next_number_in_sequence(*m);
    let branch = BranchName {
        prefix: "btc".into(),
        seq_no: next_num,
    };
    Ok(branch.to_string())
}

fn parse_sequence_number<S: AsRef<str>>(branch_name: S) -> Result<u64> {
    let s = branch_name
        .as_ref()
        .strip_prefix("btc/")
        .ok_or(anyhow!("name doesn't begin with prefix"))?
        .trim_start_matches("0")
        .splitn(2, "-")
        .take(1)
        .collect::<String>();
    Ok(s.parse::<u64>()?.into())
}

fn next_number_in_sequence(last: u64) -> u64 {
    return Sieve::new().find(last + 1).1;
}

struct BranchName {
    seq_no: u64,
    prefix: String,
}

impl BranchName {
    fn to_string(&self) -> String {
        // TODO labels
        format!("{}/{:0>4}", self.prefix, self.seq_no)
    }
}

#[cfg(test)]
mod test {
    use crate::create_branch::*;

    #[test]
    fn test_parse_sequence_number() {
        let cases = vec![
            ("btc/1000", Some(1000)),
            ("btc/0041", Some(41)),
            ("btc/1001", Some(1001)),
            ("main", None),
            ("btc/0001-foobar", Some(1)),
        ];
        for (name, expected) in cases {
            let got = parse_sequence_number(name);
            assert_eq!(expected, got.ok());
        }
    }

    #[test]
    fn test_format_branch_name() {
        let branch = BranchName {
            seq_no: 1,
            prefix: "btc".into(),
        };
        assert_eq!("btc/0001", branch.to_string());
    }

    #[test]
    fn test_generate_next() {
        let cases = vec![(["btc/0003", "btc/0001"], "btc/0005")];
        for (test_case, expected) in cases {
            let arr = std::array::IntoIter::new(test_case);
            let got = next(arr);
            assert!(got.is_ok());
            assert_eq!(expected.to_string(), got.unwrap());
        }
    }
}
