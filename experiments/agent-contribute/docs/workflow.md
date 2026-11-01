# Contributor workflow

The portable entry point is [the thin adapter](../agent_contribute/adapters/agent-contribute/SKILL.md).
Install the CLI following [README](../README.md), then install that skill directory
using `agent-contrib skill --output PATH/agent-contribute/SKILL.md` and the host
agent's skill mechanism. `agent-contrib instructions` supplies the authoritative
versioned protocol guidance; the adapter contains no repository rules. It uses the ordinary
[Agent Skills format](https://agentskills.io/specification): a directory with a
frontmatter-bearing SKILL.md. Hosts without a skill loader can be told to read it.
The same JSON contract can be read by Codex, Claude Code, OpenHands or a Paseo
worker. No provider adapter, account sharing or model API is required.

## From a request to a reviewable change

1. Resolve the source and allowed target repositories from the user's request and
   verified Git remotes. Run discovery and inspect several candidates. Retain the
   snapshot timestamps and completeness flags. Read the repository's policies,
   specification index, agent files and relevant code at the captured base commit.
2. Check issue references, subject matter, checklist identifiers and touched files
   against open PRs. Automatic overlap detection is a screening aid; inspect likely
   matches. Claimed/duplicated tasks are excluded. GitHub's
   [issues API also returns PRs](https://docs.github.com/en/rest/issues/issues#list-repository-issues),
   and [pagination matters](https://docs.github.com/en/rest/using-the-rest-api/using-pagination-in-the-rest-api).
   A partial scan cannot establish that no duplicate exists.
3. Prepare a draft contract, then fill the objective, acceptance criteria, code
   locations, prohibitions, prerequisites, validation and definition of done from
   the issue **and inspected code**. Explicitly label assumptions and partial scope.
   Review the contract before setting its status to reviewed. This is scope review,
   not a maintainer's authorization to contribute upstream.
4. Verify permission. Maintainer-approved tasks require the repository's explicit
   policy and matching approval signal. Candidates stay pending for upstream.
   Separately record the human's permission to experiment in a specified fork.
   The CLI never creates a claim/approval comment or an issue in a fork.
5. Create a branch/worktree from the agreed base. Inspect build scripts and
   dependencies before tests: a build command runs project code. Use normal,
   unprivileged development tools, no production secrets, and isolated caches and
   test data. Implement acceptance behavior and the regression test together.
6. Run the repository-required gate plus the focused regression. Record real output,
   skipped checks and unavailable requirements. A test runner's exit code alone
   cannot prove its assertions ran. LightCraft writes skip details in its CI report.
7. Request review in a fresh agent context with the contract, base/head and diff.
   Prefer another model family. The reviewer checks the actual implementation and
   runs tests where useful. Resolve significant findings before calling it complete.
8. After the implementation commit, keep the evidence file outside the checked
   worktree. Record the clean HEAD/tree/base and canonical task digest as specified
   by the CLI. Fill the evidence schema with test results, criteria evidence,
   independent review identity and revision, and the human sign-off state. Re-run
   checks/review when the relevant code or contract changes.
9. Run verify and read individual gate failures. A valid contract is not a verified
   implementation. A fork implementation is not approved for upstream submission.
   Generate the PR body using the repository template and the recorded evidence;
   include missing checks, overlap/dependencies and any still-conceptual stages.
10. Use existing human authorization for the specific target. Before publishing,
    query `gh pr list --repo OWNER/FORK --head BRANCH --state all --json number,url,state`.
    Reuse an open PR; investigate a closed one before creating another. Inspect
    fork/upstream ahead-behind counts, merge-base and intended PR diff. Without an
    authorized target, stop at a local contract. Compare recorded authority with
    the actual user request, not a contract self-assertion. Push to the
    verified fork remote, then create a draft with explicit `--repo OWNER/FORK`,
    `--base main`, `--head BRANCH`, and `--body-file PATH`. The CLI itself performs
    no GitHub mutations. Never auto-merge.

## Maintainer integration

A maintainer can opt a repository in by placing a reviewed `.agent-contrib.json`
on its default branch and choosing dedicated approval labels (see README for the
exact policy schema). That policy explicitly delegates the label decision to
accounts the repository permits to label issues. Projects wanting approval by only
specific named maintainers need a stronger actor-verification integration; v0 does
not silently infer it from general triage labels or comments.

Maintainers still define acceptance and required CI. Store curated task contracts
in the repository if useful, link them from issues, and review proposed changes
through existing GitHub branch protection and PR review. No hosted coordinator or
external database is needed. Candidate proposals can be reviewed through whatever
channel the project permits; this prototype never sends unsolicited proposals.

The CLI verifies contributor-reported evidence for consistency. It cannot prove
that an adversarial contributor actually ran a command or that two reviewer names
represent independent people. Protected CI attestations and protected human review
would be needed for that stronger assurance. Contract prose is a scope agreement,
not executable authority, and does not override the user's instructions.

## LightCraft pilot boundaries

The POC branch changes only this experiment. The optional product pilot runs on a
separate branch and is never merged into the POC. The task is a scoped part of
#3492; #3457 was declined because #3463 already addresses it. No upstream approval
has been manufactured and no upstream PR will be submitted. Fork issues are
disabled, so task contracts and reports are versioned here as files.

Root AGENTS.md and CLAUDE.md differ about some validation/lint guidance. This
experiment uses the stricter full `cargo xtask ci` gate and records failures; it
does not edit either instruction file. The ignored `plan/` directory is absent.
The committed architecture, roadmap and relevant crate sources supply the
available context. An unavailable full CI stage stays incomplete even when the
experiment's own tests pass.
