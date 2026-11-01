# Fork discovery and final-commit validation, 2026-11-01

The second fresh-context trial completed task discovery and draft preparation
without coaching. CLI 0.2.0 resolves the disabled-issue fork to its parent itself;
the agent ranks the candidates using source and discussion review. The historical
[0.1.0 trial](../validation-2026-11-01/REPORT.md), which needed one discovery hint,
is preserved unchanged. Neither trial grants implementation or publication rights.

## Revisions and protocol checks

- CLI source commit: `87f74cbe56d485beceb1a6efb2c479e25ce8c621`.
- Reviewed `core.py` SHA-256: `6794b0e5c44d5517a5963fa0d117e683ba0a5b855eca17652ea2ef483de7463d`.
- CLI/library release: **0.2.0**. Task/evidence schemas remain **0.1**, byte-identical.
- Skill SHA-256 remains `aeab9c59aa99d6a0487280aeed5538d492af2eccc4233646f02328f331bb3b1a`.
- [65 protocol tests passed on the committed revision](protocol-commit-tests.log),
  and [65 passed after extraction outside LightCraft](standalone-tests.log).
  Wheel build/install, console entry, packaged instructions and Skill export all
  exited 0. [Installation record](standalone-install.json) binds the artifact and
  installed resources to the source revision.
- Independent adversarial review caught a null-description discovery regression;
  it was fixed and re-attacked. [Final implementation review](../../review/fork-resolution-review.md)
  accepts the bounded change. No product code or CI configuration changed in PR #1.

Repository resolution returns `requested_repository`, `issue_source_repository`
and `proposed_target_repository`, with timestamped metadata-read provenance.
Only disabled-issue forks use their immediate parent's enabled tracker. Ordinary
repositories and forks with their own tracker retain their own source/policy.
Missing parent data, unreadable APIs, malformed responses and incomplete scans
fail closed. Source and proposed-target open PRs are both screened; bare issue
references belong to the PR's own repository.

A reference such as `nkovanen/lightcraft#3479` is a **CLI lookup reference** that
preserves the request context, not a GitHub issue URL. The real issue URL and
contract source remain `storytold/lightcraft`. The fork is proposed, never
implicitly authorized. A schema 0.1 contract does not store the original request;
`verify.requested_repository` therefore returns null with that provenance instead
of inventing it from the execution target.

## Fresh-agent trial

A new agent (`cc5f9a89-bb3d-4bfa-8990-26ee237eee81`, Codex GPT-6.1-Sol through Paseo)
started in an otherwise empty directory with the installed CLI and unchanged
Skill. No issue number, brief, prepared checkout or source files were supplied.
The complete explicit task prompt was:

> Go help nkovanen/lightcraft.

**Interventions: zero.** Installation and existing `gh` authentication were
preconditions. Paseo supplies normal caller identity/title metadata, but no parent
conversation was supplied. The agent resolved the parent through the CLI, scanned
**654 open issues** (14 issue pages, 8 source-PR pages and 1 fork-PR page), and read
pinned source instructions/code, issue discussions and timelines. All scans were
complete. [Setup and outcome](fresh-agent-setup.json), [discovery summary](fresh-discovery-summary.json),
[activity record](fresh-agent-activity.json), and [independent trial audit](../../review/trial-review.md)
record what actually happened.

The agent's [ranking](lightcraft-candidate-ranking.json) chose:

1. **#3479:** a saved, configuration-only preference for 24-hour capture-time
   presentation with an English interface; current behavior was checked in code.
2. **#2773:** UI cleanup, deferred because multiple requested changes need scope.
3. **#2000:** update indication, deferred because behavior needs definition.

It rejected #2085 and #1908 after finding merged fixes despite the issues staying
open, #2813 because the fix belongs to downstream AUR packaging, and #2254 because
it needs maintainer decisions. The CLI supplies conservative heuristic scores and
overlap screening; this final ranking is the agent's judgment, not a measured
success prediction from the CLI.

The agent wrote its own [candidate task contract](lightcraft-3479-task.json): five
behavior criteria, exact source revision/code locations, path scope, proposed
validation, prohibitions and pending prerequisites. Canonical contract SHA-256:
`24b96eeed78f5ce2ed58f5e413dd4ede4c14dfe32a1d8fd762f32a1b74d3ea9c`.
Both the [agent's verification](lightcraft-3479-verification.json) and
[coordinator's independent refresh](fresh-coordinator-verify.json) report a valid
schema, complete live reads, no detected overlap and **all execution/completion/
upstream gates false**. Approval remains candidate; target remains the requested
fork; authorization fields remain blank. Verify exited 1 as expected. The agent
asked for permission to implement the concrete subset; no answer or follow-up was
given, and no implementation, tests on repository code or GitHub writes occurred.

This is an **uncoached fresh-context trial, not a fully blinded experiment**.
Public fork PR #1 already mentioned the previous #3479 candidate and the agent
read that body. The [provider tool-call chronology](fresh-candidate-chronology.json)
shows its first batch read of #3479 at 09:23:44 UTC and inspection at 09:25:12,
before reading PR #1 at 09:25:25; preparation followed at 09:25:55. Later influence
from that public evidence cannot be excluded. The raw session is hashed, but its
internal reasoning is not exported. The manual merged-PR check was bounded to the
latest 100 closed PRs, so it does not prove absence of every possible prior fix.

## Product PR #2

The only new product-branch change removes the unnecessary Calibration `.clone()`
at `crates/ui-egui/src/tests/slider_reset.rs:189`, commit
`b11d21de4422542534518dfa77c21914ed78bbd7`. Independent review found no behavior
change. On that clean commit, the [focused regressions](focused-tests.log) exited
0: **3 passed, 0 failed, 0 ignored**, with no recorded internal skips.
[Invocation and revision](focused-result.json) include the full-log hash.

Full `cargo xtask ci` ran from **09:13:10 to 09:44:56 UTC** on the same clean
commit and returned **exit 1** after about 31m 46s. Formatting, clippy and HEIF
passed. The UI library reported **1771 passed, 0 failed, 4 ignored**, including
the three Calibration regressions. UI integration tests reported **31 passed,
1 failed**: `dialog_role::settings_is_a_dialog_too` panicked because the installed
semibold fonts lack Georgian `ქ` (U+10E5). No `CRAFT_FONTS_DIR` was configured.

The repository's generated report records **175 distinct internal skips**:
corpus 137, fonts 28, models 8, GPU 1, system ICC profiles 1. These are separate
from framework `ignored` tests. **Parity, layers, assets and wasm did not run**
after the test failure; workspace doctests were not reached either. The full gate
is not green and no completion-ready claim is made.

- [Exact command/exit/revision and full-log hash](ci-result.json)
- [Unmodified generated CI report](ci-report.json) and [summary](ci-summary.md)
- [Raw HEIF skips](ci-skips-heif.jsonl) and [workspace skips](ci-skips-test.jsonl)
- [Failure excerpt](ci-failure-excerpt.log) and [condensed result](ci-condensed.json)
- [Minimal compiled-test reproduction](settings-dialog-repro.json) and [output](settings-dialog-repro.log)

The failure reproduces by running the compiled Settings-dialog test alone on this
head. Its test, font/theme/settings paths and dependency lockfile are unchanged
from the source base; the base itself was not executed for comparison. This new
font/test blocker was recorded without changing font behavior, weakening a check
or broadening the Calibration task. Full stdout is retained locally at
`validation-runs/fork-resolution/ci.log`, bound by the recorded SHA-256.

## Reuse and limits

The same installed CLI/Skill, without modification, completed read-only discovery
against unrelated `pallets/click` (**75 issues**) and prepared a draft for #373.
[The ordinary-repository result](unrelated-summary.json) preserves source==request
and a blank pending contract execution target. This demonstrates reusable discovery
and drafting, not implementation in that project or every agent harness.

Maintainer opt-in still delegates approval to **every account with issue-label
permission, including triagers**. v0 does not authenticate a particular maintainer
as label actor. Resolution and labels never supply fork authorization or permission
to publish upstream. Evidence remains self-attested. One-parent resolution,
conservative topic overlaps, and no exception for an already-open own PR are
intentional fail-closed limits; do not claim final protocol verification for an
existing matching draft. Both fork PRs remain drafts; no upstream publication,
merge or new product task is part of this iteration.

Session-owned compiled output and the temporary PATH symlink were removed after
recording results, as required by repository cache hygiene. The installed wheel,
trial files, raw logs and reports remain available; see [cleanup](cleanup.json).
