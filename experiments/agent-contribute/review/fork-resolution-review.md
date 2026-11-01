# Fork resolution: design/attack review (pre-implementation; builder worktree had no changes at inspection)
Grounding (live): nkovanen/lightcraft id 1441563016 fork=true has_issues=false parent=storytold/lightcraft (id 1398185212),
source=storytold/lightcraft. Fork has open draft PRs #1 (experiment/agent-contribute-v0) and #2 (pilot branch, head 150d4057).
PR2 lint fix b11d21de: only `unrelated.calibration = before.calibration;` (clone_on_copy). Semantically identical; Calibration
is Copy (clone was a lint). No concern; the PR head on the fork (150d4057) will need the push to be refreshed.

D1 (High) Own fork PR blocks verify. Scanning the proposed target's open PRs as blocking overlap means the contributor's own
  deliverable (fork draft PR #2 for #3492) is an "overlap" with its own contract, so `execution_ready` (and fork_completion_ready)
  can never be true once the draft exists; workflow step "reuse an existing PR" contradicts it. Require explicit semantics:
  source-repo PR overlaps block; target-repo PR overlaps are reported as `target_prs` with reuse/blocking classification: a target
  PR whose head repo == target and head.sha == evidence.git_head (or an ancestor of HEAD per `git merge-base --is-ancestor`) is
  the contributor's own work and is reported as `existing_pr`, not an overlap; any other target PR with explicit full reference
  still blocks. Test: fork PR with `storytold/lightcraft#N` full ref and head == evidence head must not block; different head blocks.
D2 (High) Verify must re-establish fork relationship, not trust the task: for mode fork-experiment fetch `repos/{target}` and
  require fork==true and parent.full_name == task.source.repository (case-insensitive, canonical full_name equality, repo id
  preferred) else fail closed. Today any other repo name passes "different from source" (my earlier M4). Also unresolved if the
  fork was deleted/renamed/detached.
D3 (Medium) Source/target confusion in bare references. Bare `#N` in a fork PR is relative to the fork (issues disabled: its own
  namespace) and must never match upstream issue #N; in upstream PRs bare `#N` is upstream. index_prs/duplicate_prs currently
  keep one unqualified `local` set for all PRs: it must be keyed by the PR's base repo. Full `owner/repo#N`/URL refs resolve to
  that repo and match only actual source. Tests: fork PR "Fixes #3492" bare -> no match; fork PR `storytold/lightcraft#3492` -> match;
  upstream PR bare `#3492` -> match; reference to `nkovanen/lightcraft#3492` -> no match (fork's own namespace).
D4 (Medium) Metadata validation (fail closed, no fallback): has_issues must be a real bool (reject "false"/0/null), fork a bool,
  parent a dict with full_name passing parse_repository and an integer id; fork==true but parent missing/inaccessible (deleted,
  private, DMCA) -> error, never resolve to the fork itself; has_issues true -> unchanged (source = requested); has_issues false
  and fork false -> error (issues unavailable), not fork. Chain: use exactly one hop; if parent is itself a fork with issues
  disabled report unresolved (do not follow `source` silently). Require parent canonical name to match what the parent endpoint
  returns (rename/transfer) and parent has_issues true, else error. Do not probe issue 404s to infer has_issues.
D5 (Medium) Approval scope. With a disabled-issue fork, approval comes from the parent's policy and means "upstream contribution
  eligible" only. Output must label it so and prepare must not set execution mode/authorization: proposed fork stays in
  `execution.target_repository` with mode `pending` and blank authorized_by (verify already errors on pending mode; keep a
  regression). Also issue-enabled forks: a fork's own `.agent-contrib.json` makes `approved` and, with source==target==fork,
  `upstream-contribution` verifies as upstream_submission_ready. Record `source_is_fork` from the metadata already fetched and
  refuse upstream-contribution mode (and upstream_submission_ready) when the source repo is a fork.
D6 (Medium) Completeness/pagination: the combined overlap result needs both scans complete; target-scan failure/incomplete ->
  duplicate_scan `unknown` with an error naming which repository failed, never silently using the source scan only. Reuse the
  already fetched repository metadata (no second inconsistent GET), and record per-repo pulls_scan in provenance with page counts.
D7 (Medium) Compatibility: task schema 0.1 is closed (additionalProperties false). Adding requested/proposed fields to
  `task.source` while keeping schema "0.1" makes 0.2.0 documents that 0.1.0 rejects and, if the new fields are required, 0.1
  documents that 0.2.0 rejects. Keep task.source/execution shape unchanged (source = actual parent, execution.target_repository =
  proposed fork) and put requested_repository/issue_source/proposed_target/provenance only in the inspect/discover output
  wrapper; if a contract field is unavoidable, bump the schema string and version policy (docs say unknown versions fail closed,
  no silent migration). Verify output should echo `requested/proposed` from the task only.
D8 (Low) Provenance: record numeric repo ids (requested, parent) and fetched_at/default branch commit of the source; verify
  compares source id to catch a renamed/transferred source whose old name now belongs to another repository.
D9 (Low) Zero-context flow: `discover fork` ranks parent's issues but `inspect/prepare fork#N` must print the resolved source in
  the first lines and `prepare` output must warn the number belongs to the parent; a fork-with-issues-enabled and
  `fork#N` must keep reading the fork's issue (unchanged). Instructions text must say fork authorization != upstream approval.
No PR2 finding. Wait for the stable source for code review.

# Code review of frozen patch (stable.patch sha256 2788f11f75c4c766613dca60a393997a1e333036a62f616b206617de771df173, base adf6d9d8)
61 tests pass (experiment dir). Skill SKILL.md sha unchanged (aeab9c59...). Repro: scratchpad r11.py (imports builder package).
R1 (High, regression) `discover` crashes on any open issue with a null body (GitHub returns body:null for empty descriptions,
  common in a 3.5k-issue repo). discover() now calls duplicate_prs(source, item, target_index, ...) and ranking(item, duplicates)
  on the RAW list item instead of the normalized issue that inspect_from_snapshot/issue_data used before; duplicate_prs does
  `issue['title'] + '\n' + issue['body']` -> TypeError, not a ProtocolError, so cli.main prints a traceback (TypeError is not in
  its handled exceptions). Repro: FakeGitHub issues list with body=None -> "discover null body -> TypeError can only concatenate
  str (not 'NoneType') to str". Fix: use `issue_data(item)` once and reuse it for duplicates and ranking (or reuse
  inspected['ranking'] with the combined duplicates), plus a regression test with body None (also covers ordinary repos);
  consider catching unexpected exceptions per issue as ProtocolError. The live `inspect 3492` check could not catch this.
R2 (Low) prepare now prefills execution.target_repository with the requested repo even for ordinary repos (target == source,
  mode pending); previously ''. Harmless to gates but "ordinary repos unchanged" is not literally true; prefill only when the
  proposed target differs from the issue source.
R3 (Low) verify's `requested_repository` is derived from execution.target_repository, not the original request; label it
  `proposed_target` only or omit to avoid misleading provenance.
Checked, no finding: resolution is one hop with bool/shape validation and canonical-name checks, parent cannot equal requested,
  non-fork + no tracker errors, issue-enabled fork unchanged; target PR bare refs are not matched to the source number (only full
  refs); source-side bare refs match; incomplete target scan makes duplicate_scan unknown and complete False; verify requires
  target.fork and parent == source for fork-experiment, fails closed on metadata errors, and keeps target PR overlaps (own-PR
  limitation documented); prepare for an unresolved fork exits 1 with JSON, no traceback; task schema unchanged.

# Follow-up freeze (review-followup.patch sha256 b65d4e7e9ff8a8fe2c46d8d97930d598f653bab1f3d0b8ce8db51a152f384f53)
core.py sha256 6794b0e5c44d5517a5963fa0d117e683ba0a5b855eca17652ea2ef483de7463d; 65 tests pass; Skill unchanged (aeab9c59...).
R1 fixed: discover normalizes with issue_data(item) before duplicates/ranking; my r11.py repro (null body, discover via core and
  CLI for ordinary and fork requests) now completes; a null-assignees shape yields complete=False without crashing.
R2 fixed (ordinary and enabled-tracker fork drafts leave execution.target_repository blank; resolved disabled-tracker fork
  proposes it). R3 fixed (verify requested_repository null with explicit provenance). Inspect ranking is recomputed after
  target-only overlaps (new regression test). No new defects found in this increment.
Final readiness: no open significant findings in requested scope. Documented limits remain: own open target PR blocks verify,
  one-hop resolution only, proposed-target PR scans are conservative, approval never authorizes execution.
