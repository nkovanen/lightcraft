# Final-commit validation and fresh-agent experiment

Both fork PRs remain drafts. This round tests the existing release and product
patch; it does not add a feature or claim upstream approval.

## Revision-bound checks

| Check | Revision | Actual result |
| --- | --- | --- |
| PR #1 protocol suite | `0e449b199f5868c7217a6b9e66d93cc5fd18c833` | Exit 0; 45 tests passed |
| PR #2 `cargo xtask ci` | `150d4057a2f0a717caec6f916dff76596253ba44` | Exit 1; failed at clippy |

The PR #2 checkout was clean before and after the run. Formatting passed.
Clippy rejected `before.calibration.clone()` in
`crates/ui-egui/src/tests/slider_reset.rs:189` because Calibration implements Copy.
The clippy subprocess exited 101; the outer `cargo xtask ci` exited 1.

The repository produced an actual `target/ci/report.json` and `summary.md`; both
were captured along with the complete log and invocation metadata. HEIF,
workspace tests, parity, layers, assets and wasm did not run after the failure.
The report's internal `skips` array is empty, **not evidence of zero skipped
assertions**: its summary says skipped tests are unknown because the test stage
did not run. Earlier pilot test passes do not change this full-gate failure.

Environment: Cargo/Rust 1.95.0, four build jobs, two Rust test threads and two
Rayon threads, with a dedicated `target/agent-validation-final` cache. The tested
commit was not edited to conceal the result.

## Fresh-agent setup and exact-prompt baseline

A new Codex/Paseo agent started in an empty, non-Git directory. Its only workspace
file was the exported generic Skill. CLI 0.1.0 was already installed on PATH and
the existing user gh authentication was available. No issue number, task brief,
checkout or prepared worktree was supplied. Normal host skills and Paseo caller
identity/title metadata remained available; no parent conversation was passed.

The explicit prompt was exactly:

> Go help nkovanen/lightcraft.

The agent automatically found the Skill, read `agent-contrib instructions`, ran
discovery, inspected repository metadata and listed the existing PRs. It found
issues disabled, zero fork issues, no approval policy, and drafts #1 and #2. It
stopped without code or GitHub writes and asked for a source issue and scoped
execution authorization. It did **not** independently rank candidates or produce
a specification. Therefore the zero-coaching test fails the requested autonomous
discovery/specification criterion, while respecting the approval boundary.

## Interventions

| Event | What was supplied | Treatment |
| --- | --- | --- |
| Installation precondition | Released CLI on PATH and unchanged generic Skill in the otherwise empty workspace | Recorded setup; no repository task information |
| Excluded launch | Coordinator accidentally typed `nkovananen/lightcraft` and immediately cancelled that agent | Invalid trial; no outcome used |
| Valid baseline | Exact prompt above, new agent and new empty directory | No coaching before the agent stopped |
| I1 | Permission to inspect the fork's public parent read-only, independently rank/select/check overlaps and prepare a local candidate spec; explicitly no implementation or GitHub writes | Assisted diagnostic follow-up, separate from the failed unassisted test |

No issue number or task brief was supplied in I1. Its exact message and the
setup/agent identity are recorded in the accompanying JSON. The assisted result is recorded below.

## Maintainer-approval limitation

Version 0.1's `label_authority: repository-label-permission` intentionally
delegates the approval signal to **every account the repository allows to apply
issue labels, including triagers**. It does not authenticate that the actor is a
maintainer or verify a named maintainer applied the label. The repository opts in
through a policy on its default branch and dedicated approval labels; generic
triage labels do not count. Projects requiring named maintainers or protected
review attestation need an additional integration before using this signal.

Fork experimentation authorization and final human sign-off remain separate.
Neither a public issue nor this testing request supplies upstream approval.
This limit is also present in the packaged instructions and contributor workflow.

## Unrelated repository

The exact installed CLI and Skill, unchanged byte for byte, were exercised against
`pallets/click`. Live discovery completed with 75 issues; all remained candidates
because no opt-in policy was present. Inspection and `prepare` for a candidate
returned exit 0. The generated draft passed schema validation. `verify` returned
exit 1 with execution, fork completion and upstream submission false, correctly
reporting missing scope review and authorization/evidence.

This demonstrates portable read-only discovery, inspection, draft specification
generation and verification without editing the Skill or CLI. It does not claim
an implementation, validated behavioral contract, or PR against that repository.
No external issue, comment, PR or policy was created.

## Assisted follow-up result

After I1, the agent independently scanned 651 parent issues (14 issue-list and
eight open-PR pages), identified 117 without detected exclusions/overlap, and
ranked #3479, #3517 and #2254. It selected a bounded #3479 proposal: opt-in
24-hour capture-time presentation with English UI, initially through ui.json.
The candidate spelling and edge behavior are explicitly proposed design choices
awaiting review, not accepted maintainer requirements.

The agent inspected selected source files by immutable commit, issue timelines,
the 50 most recently updated closed PRs, and topic/issue searches. It rejected
#2085 as already implemented, avoided the existing #3492 fork pilot, and examined
adjacent PR #3146's patch and shared files. These checks are bounded observations,
not a proof of exclusive ownership. Its ranking report records scope, ambiguity,
external dependencies and relative implementation risk.

The self-authored specification contains six proposed behavior criteria,
concrete scope paths, validation argv, prerequisites, unresolved questions and
a definition of done. It remains a draft. The coordinator independently reran
verification: schema and live scan valid, execution/fork completion/upstream
submission all false, exit 1 as expected. No issue number, brief or prepared
checkout was supplied by the coordinator. No implementation or GitHub write
was authorized or observed.

The generated contract still needs semantic review before execution. In
particular, it classifies the composite full CI command as a `check`; reviewers
should make its required test-count expectations machine-checkable before using
its evidence gates. Schema validity alone does not approve acceptance criteria.
The original generated artifact is preserved without correction.

## Outcome

The exact-prompt baseline did not achieve autonomous task discovery or drafting
on the issues-disabled fork. One explicit parent-discovery hint was needed.
After that hint, candidate ranking, overlap checks, independent specification
preparation and the upstream approval boundary worked in this trial.
Unrelated-repository read-only CLI operation also worked with unchanged code,
schemas, installed guidance and Skill. These results do not establish autonomous
implementation readiness or a maintainer-verified approval system.

## Evidence files

- [Actual CI exit metadata](ci-result.json), [repository-generated JSON report](ci-report.json),
  [repository-generated summary](ci-summary.md), and [clippy error](ci-error.log).
- [Protocol test metadata](protocol-result.json) and [45-test log](protocol-tests.log).
- [Setup and exact interventions](fresh-agent-setup.json),
  [agent's candidate ranking](lightcraft-candidates.txt),
  [unaltered agent-generated specification](lightcraft-3479.task.json),
  [agent verification](lightcraft-3479.verify.json) and
  [independent coordinator re-verification](fresh-agent-coordinator-verify.json).
- [Unrelated-repository summary](unrelated-summary.json),
  [generated draft](unrelated-task.json) and [blocked verification](unrelated-verify.json).

The complete compiler log, raw GitHub command outputs and curated agent timeline
are retained in the local `validation-runs/final-heads` run directory outside
the LightCraft checkout. Hashes in the records bind the relevant raw outputs.
The protocol package and Skill were unchanged throughout these trials.
