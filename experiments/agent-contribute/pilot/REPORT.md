# POC execution report — 2026-11-01

The protocol implementation and portable adapter are working. The independent
implementation agent consumed the real task contract and produced a tested
Calibration reset change on a separate branch. The final product review is accepted; full LightCraft validation is
incomplete as recorded below; this is not an upstream-approved task.

## Architecture and distribution

The versioned Python CLI/library owns discovery, parsing, compatibility checks,
GitHub reads and passive evidence verification. The 135-word Skill only locates
that CLI and delegates to its installed instructions. It contains no repository
names, test commands or model-provider integration. Repository rules are read
from a pinned GitHub default-branch snapshot and refreshed at verification.

CLI release **0.1.0** supports task/evidence schema **0.1** exactly. Unknown
versions fail closed. Updates and adapter replacement require explicit action.
The complete experiment was copied outside the LightCraft checkout, installed
into a temporary virtual environment, and exercised from an unrelated directory.
Neither the Cargo workspace nor LightCraft imports are needed. See
[distribution and update policy](../docs/distribution.md).

## Live discovery and selection

A real `agent-contrib discover storytold/lightcraft` completed successfully:
651 open issues, 14 repository-issue API pages (including filtered-out PRs),
eight open-PR pages and one label page. Pagination completed without errors.
All 651 tasks remained candidates: upstream had no agent opt-in policy. The
[bounded discovery record](discovery-summary.json) includes timestamps, source
revision, rankings, exclusions and the raw response hash.

[Issue #3457](https://github.com/storytold/lightcraft/issues/3457) remained open
but carried a duplicate label, and [PR #3463](https://github.com/storytold/lightcraft/pull/3463)
explicitly addressed it. Several alternatives also had open fixes. Code review
identified a small, testable subset of
[issue #3492](https://github.com/storytold/lightcraft/issues/3492): expose the
existing undoable Calibration reset in its header menu. This does not resolve
the whole issue or add Point Color, Color Grading or Defringe actions.

[PR #1554](https://github.com/storytold/lightcraft/pull/1554) and
[PR #3155](https://github.com/storytold/lightcraft/pull/3155) overlap Edit-panel
files. Their descriptions and paginated changed-file lists were inspected;
neither described this reset action. Their overlap remains a future merge risk.
This was targeted inspection, not a review of every open PR patch.

The canonical [JSON contract](../tasks/lightcraft-3492.json) is reviewed for scope,
with upstream approval still `candidate`. The initiating user's explicit grant
authorizes experiments and draft PRs in `nkovanen/lightcraft` only. The contract
records that grant separately from maintainer approval and final-code sign-off.

## Independent agent handoff

A Codex worker in a separate Paseo worktree received the adapter, task brief and
worktree location rather than implementation instructions. It read the code,
wrote regressions, implemented the menu and exercised the actual UI.

The initial handoff exposed incomplete packaging while the CLI was still being
built. Its first machine-readable preflight later failed closed on an outdated
issue digest after a pre-release hash-algorithm change. The coordinator compared
the unchanged issue content and regenerated/reviewed the canonical digest. The
worker then read the final installed thin adapter, ran `agent-contrib instructions`
and verified the refreshed JSON contract using the installed executable.

[Recorded preflight](handoff-preflight.json): schema valid, complete live scan,
no detected duplicate, and `execution_ready: true`. Evidence, fork completion and
upstream submission remained false because final evidence/review/sign-off had
not been supplied. Exit 1 in this state is intentional, not implementation failure.
The contract digest is
`f454e02ed384511ffd715c3413dfb1839f9a06c80298f7836d7af2936cd871be`.

The first real UI regression run executed three tests and all failed without the
menu implementation. A later run passed all three after correcting the screenshot
fixture. Independent review then caught the overly broad shared-header behavior:
Effects/Optics would have reset only some visible controls, and Optics would have
violated the contract's Defringe prohibition. A new negative regression failed against the shared-header implementation.
The narrowed Calibration-only patch then passed all three tests, and the
independent reviewer accepted the actual diff. The screenshot was regenerated
and visually inspected by the pilot, reviewer and coordinator. Its SHA-256 is
`ed5fdd52fe389fd85f1c562bff5af42ca3f0e030e1c1bce5744153f4bb35badd`.
The reviewed pre-commit diff hash is
`684d1f95b190175aa0b7e6222db3dfd9d252f49bb8afbc87368e06e05c6776bc`.
The product gate was interrupted while queued; no completion-ready claim is made.
[Product execution evidence](product-execution.json) and the
[focused test log](product-focused-tests.log) retain the observed results.

## Executed protocol checks

| Check | Observed result |
| --- | --- |
| Experiment unittest suite | 45 passed; [log](protocol-tests.log) |
| Extracted standalone unittest suite | 45 passed; [log](standalone-tests.log) |
| Wheel build and isolated installation | Passed; console entry point works outside checkout |
| Packaged workflow and Skill export | Passed from unrelated working directory |
| Unknown schema and existing adapter file | Rejected; existing file preserved |
| Python syntax compilation | Passed |
| Live discovery, inspect, prepare and verify | Executed against real upstream data |
| Independent protocol review | No remaining significant findings after repeated attack/fix/review rounds |
| Full root `cargo xtask ci` | Interrupted during GPU unit tests; no final result |
| Repository layering and asset checks | Both separately passed after interruption |

[Standalone installation evidence](standalone-install.json) records release,
compatibility checks and hashes. One initial root-directory test invocation
failed because its subprocess could not import the uninstalled module; the
documented invocation from the experiment directory passed all 45 tests, as did
the extracted copy. No failure is counted as a successful check.

## Adversarial review

A separate Claude Sonnet reviewer inspected actual code and ran attack repros.
The final accepted `core.py` SHA-256 is
`2190f63070f04abfe526a932dfa902f403f22f372c99ba731bffeac85dd8a595`.
Findings led to regressions and fixes for Git-config command execution, empty
implementations, scope/remote mismatches, rename and intermediate-base scope
bypasses, PR regex denial of service, stale/zero-test evidence, FIFO reads,
artifact containment, Git replacement refs, submodule masking and leading-whitespace
filename scope bypasses. NUL-delimited Git output preserves exact path bytes;
real-Git regressions and independent attacks cover unusual filenames.

The final reviewer rechecked upstream compare direction, rejected contributor-only
bases and failed API responses, verified gitlink rejection in HEAD/base/index,
and confirmed that hostile nested Git configuration was not executed. The
reviewer also checked final documentation and standalone hashes/logs.

## Limits and maintainer integration

- Evidence is self-attested. Hash/revision checks detect inconsistency, not forged
  test runs or forged reviewer identities. Protected CI/review attestations remain
  future work. Verification never executes validation commands.
- Approval requires a repository-controlled policy and dedicated label. Its
  documented authority includes accounts allowed to label issues. Actor-specific
  maintainer approval needs a stronger integration.
- Ranking is conservative heuristic screening. Agents still inspect requirements,
  code, recent merges, timeline links and ownership. Topic overlaps can block
  valid tasks; this POC has no manual override. API failures and partial scans
  cannot establish permission or absence of duplicates.
- Rebase-only evidence is supported; submodule evidence is rejected. The CLI
  performs no model execution, GitHub writes, automatic update, PR submission
  or merge. The host agent uses its existing tools and actual user authority.
- Installation/export is working. A package-registry release, signed artifacts,
  automatic host-specific skill installation and schema migrations are future
  work. Claude Code, Codex, OpenHands, Paseo and other hosts can consume the
  portable files, but every host's installer has not been integration-tested.
- Upstream must opt in and decide acceptance/CI requirements. No upstream PR or
  policy change was made. Final human approval is still required for completion.

## Full repository validation limitation

The full `cargo xtask ci` attempt completed formatting, workspace clippy and the
HEIF stage, then ran workspace test binaries through the GPU tests. The session
was interrupted before the gate exited, and no final `target/ci/report.json` was
produced. Forty-two internal skip records existed in the workspace-test stage
and four in HEIF, including unavailable corpora, models/fonts and system data.
They are not counted as exercised assertions. The remaining workspace tests,
doctests, parity and wasm checks have no verified full-gate result.

After the interruption, `cargo xtask layers` and `cargo xtask assets` each passed
separately. The product branch's full CI was still waiting for the shared slot
when its worker/session closed; no product full-gate pass is claimed. The
[validation record](repository-validation.json) preserves completed test-binary
summaries, skip counts, commands and raw-log hash. Both fork drafts intentionally
leave full CI and final human sign-off outstanding, as permitted by the original
POC request. No repository gate or policy was weakened.
