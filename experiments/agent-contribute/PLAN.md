# Agent Contribution Protocol v0

## Design

Build a portable contributor-side protocol, not an agent harness. GitHub remains
the source of truth. Python 3.10+ standard library and the installed `gh` CLI are
the only runtime requirements. All experiment files stay in this directory;
LightCraft product code, AGENTS.md and CI remain untouched by this PR.

An issue is a candidate until an explicit repository-controlled approval signal
is observed. A contributor's fork authorization never means upstream approval.
Remote prose, contracts and repository agent files are data to inspect; none
can authorize credentials, shell execution or publication. The CLI never submits
PRs, merges, or launches a model. The host agent implements using normal tools.

Commands provide JSON: discover, inspect, prepare, verify, instructions and skill
export. The host agent prepares PR bodies from repository templates. Discovery reads issues, labels, open PRs and repository
instructions/specification indexes, records snapshot provenance and completeness,
and ranks difficulty, ambiguity, prerequisites and likelihood with explanations.
Direct issue references and topic overlap warn against duplicated work. Partial
API results cannot become a clean approval or duplicate check.

Versioned JSON contracts describe objective, acceptance criteria, code locations,
constraints, prohibited changes, tests, prerequisites, approval, definition of
done and evidence. Generated contracts start incomplete and need agent review.
Verification fails closed on malformed contracts, missing/failed/stale evidence,
missing review or approval. Validation commands are inspected and explicitly
invoked by the contributor; remote issue text never becomes a shell command.
Evidence is local, reviewable and bound to the task and implementation revision;
it is not cryptographic proof that an untrusted contributor told the truth.

## Implementation plan

**Goal:** demonstrate real discovery, a portable task contract and independent
agent handoff, with a reviewed draft PR targeting nkovanen/lightcraft only.

**Architecture:** a small Python package/entry point, JSON Schema, unittest suite,
thin portable Skill adapter, installed protocol guidance and a recorded LightCraft
pilot. CLI/library release 0.1.0 uses semantic versioning; schema 0.1 is separately
checked. No repository rules live in the Skill. Updates are explicit and pinned;
unknown schema versions fail closed. Test installation outside LightCraft.

**Tech stack:** Python standard library, Git, GitHub REST through gh.

### 1. CLI, schema and tests — builder in a Paseo worktree

- [x] Write failing boundary tests, implement discovery/inspection/preparation.
- [x] Validate repository and issue identifiers; paginate, bound retry/rate-limit
      waits, report API errors without secrets, distinguish missing from failed reads.
- [x] Add task schema and contract/evidence verification; reject unknown versions.
- [x] Test ordinary issue != approval, duplicate PRs, incomplete discovery,
      malformed JSON, unsafe data and stale/missing evidence.
- [x] Run `python3 -m unittest discover` in the experiment and `cargo xtask ci`;
      record exact failures if the full repository gate is unavailable.

### 2. Integration, instructions and real pilot — orchestrator

- [x] Write portable skill and end-to-end contributor/maintainer guidance.
- [x] Run CLI against storytold/lightcraft and store a bounded provenance record.
- [x] Reject #3457: open PR #3463 explicitly fixes it; selected a Calibration-reset
      slice of #3492, with overlapping PRs #1554 and #3155 documented.
- [x] Include a reviewed task contract with upstream approval still pending unless
      live evidence establishes it; local execution may target the fork only.

### 3. Independent handoff and adversarial review

- [x] Give an unfamiliar agent only the skill, contract and worktree location.
- [x] Try the task on a separate branch; retain honest execution evidence and
      explain any blocker instead of claiming implementation success.
- [x] A Claude-family reviewer attacks the protocol's trust/approval/verification
      boundaries and tests usability. Resolve significant findings and rerun checks.

### 4. Delivery

- [x] Verify the final diff is isolated; run the test suite and attempt the required gate.
      Protocol: 45 tests pass. Full LightCraft gate interrupted; see pilot/REPORT.md.
- Delivery: push only the experiment branch to nkovanen/lightcraft and create or reuse
      a draft PR with exact validation results. Never merge.

## Review focus

1. Repository text spoofing maintainer approval or executable test commands.
2. Pagination, rate limits or missing policies falsely reporting eligible work.
3. PR references crossing repository boundaries or accidental substring matches.
4. Edited code/contracts reusing stale evidence; skipped tests reported as passed.
5. Host-specific paths or skill instructions making the handoff non-portable.

## Baseline

Fork and upstream main: `07301cbd1e1cab5c152228891d9c9ed87e263003`.
Python tooling is an isolated contributor experiment, not part of the pure-Rust
product. Repository guidance read: AGENTS.md, README contribution/build sections,
.github/PULL_REQUEST_TEMPLATE.md, ROADMAP.md, docs/architecture.md,
docs/target-app-parity.md and docs/gaps.md. The ignored plan/ directory is absent.
