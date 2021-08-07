/*
 * gtc update-pulls
 *      TODO scoped to current user.
 *              determined how?  github API?
 *
 * pull status:
 *
 *      #     Title     Status          Recommended Action
 *      4     Feature   behind [branch] rebase on [branch name] ([PR #5])
 *      5     Feature   behind main     rebase on [branch name]
 *      7     Refactor  behind base     rebase on [branch name] ([PR #5])
 *
 *      gtc update-pulls
 *          for each open |pr| in PRs
 *              state = model.compute_state(pr)
 *              if state.up_to_date
 *                  continue
 *              if state.base.has_new_commits
 *                  rebase on state.base.sha
 *              if state.base.is_merged
 *                  rebase on new.main
 *                      but what if new updates are on main?
 *
 *      State
 *          .base
 *              either default
 *              or the PR's stipulated base branch
 *
 *              ...
 *              or some other feature branch?
 *                  (its defined to be a feature branch if
 *                      branch has a tag that matches the format (we filter on this)
 *                      branch is the head of an open PR (filter on this)
 *                      branch is the fewest commits away from current PR commit (filter on this when there exist multiple matches)
 *
 *      Base
 *              has_new_commits
 *                  if the common ancestor commit is not equal to the base branch's tip commit
 *
 *              is merged
 *                  if there is a PR for that branch and it is merged in
 *                      Q: what about branch re-use?
 */
