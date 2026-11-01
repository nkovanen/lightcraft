# Pilot: reset Calibration from its section header

Task ID: `AC-PILOT-3492`. Protocol: `0.1`.

Source: <https://github.com/storytold/lightcraft/issues/3492> (open, checked
2026-11-01). This is a deliberately small **part** of that multi-part request.
It does not close the issue. The paired JSON file is the canonical machine-readable
contract; this Markdown explains it. Scope review is by the coordinating agent and
does not represent upstream approval. The pilot report records the contract digest.

## Authority and target

Upstream maintainer approval is **pending**. No `.agent-contrib.json` opt-in or
agent-approval label was found. User `nkovanen` explicitly authorized this pilot
in their fork during the project mission, including an isolated implementation
branch and a draft PR if feasible. The only permitted publication target is
`nkovanen/lightcraft`, base `main`. Never publish to `storytold/lightcraft`.

## Objective and acceptance

Make the existing Calibration section reset discoverable in the Edit panel's
section-header context menu. Reuse `develop.resetSection` and existing localized
`Reset Section` text. Prefer adding the action in the shared top-level section
menu if that avoids a special case; all exposed section identifiers must be valid.

- AC1: with a photo active and changed Calibration controls, right-clicking the
  Calibration header offers Reset Section and invoking it restores that section's
  controls to their existing defaults.
- AC2: resetting Calibration preserves unrelated settings (for example exposure)
  and uses the existing undoable command path. Undo restores the prior calibration.
- AC3: normal collapse/expand, the preview eye and Solo Mode continue to work.
  The reset must not accidentally toggle a section or reset all edits.
- AC4: a regression test drives the real UI/menu path and fails without the new
  action. Cover another section if the implementation uses the shared menu.
- AC5: required validation and a headless screenshot of the new menu are recorded.
  If build/platform requirements prevent these, report the stage incomplete.

## Starting points

- `crates/ui-egui/src/panels/edit/sliders.rs`: `section`, existing context menu.
- `crates/ui-egui/src/panels/edit.rs`: Calibration section uses `section`.
- `crates/engine/src/cmd/develop.rs`: `develop.resetSection`.
- `crates/develop/src/lib.rs`: `reset_section`.
- `crates/ui-egui/src/tests/panels.rs`, `slider_reset.rs`: UI test conventions.
- `docs/parity-checklist.md`: KEY-SLIDER-RESET and LRC-DEV-CALIB.

## Constraints

Read root AGENTS.md, CLAUDE.md, README, architecture and PR template before editing.
Their local-only `plan/` references are absent; use committed docs and record this.
Use the stricter full `cargo xtask ci` gate where the agent files differ. Product
code remains pure Rust; no panics, unsafe, unchecked input indexing, dependencies,
CI/AGENTS changes, assets, catalog/render/process version changes or broad refactor.
Use `tr()` and existing translations. Update applicable parity/what's-new/gap text
honestly in this pilot branch, with one task ID per commit.

At base `07301cbd1e1cab5c152228891d9c9ed87e263003`, `section()` exposes only Solo
Mode in its context menu. Calibration uses that helper and has a `Section::Calibration`
variant; `reset_section` resets its control defaults. Camera-profile state belongs
to a separate section and must remain unchanged when Calibration is reset.

Do not add Point Color, Color Grading or Defringe preview/reset controls in this
pilot. Their settings grouping needs separate decisions. Do not change the meaning
of defaults, the photo selection, stored settings or process versions.

## Validation and evidence

Inspect before invoking: `cargo test -p lightcraft-ui-egui --lib section_reset`
(name the new tests to match and require a nonzero executed test count),
`cargo xtask ci`, and `git diff --check`. Record the screenshot path and SHA-256
in AC5 evidence; a missing screenshot or incomplete required CI keeps the task incomplete.
Use a separate `CARGO_TARGET_DIR=target/agent-pilot`; keep memory/thread use bounded.
Record each command, stdout/stderr, exit status, exact git base/head and any skips.
Use headless UI test rendering or the existing control channel on a private port,
never OS-level synthetic input or a personal photo library. Independent review and
human sign-off are required before a ready-to-submit claim.

## Feasibility and duplicate work

Medium difficulty, low-to-medium ambiguity after this scope decision. Existing
command/localization/section helper reduce implementation effort. Main dependencies
are the Rust/UI build and ability to exercise a headless menu. Initial likelihood
is moderate, not a calibrated probability.

No open PR explicitly references #3492 or `develop.resetSection` in searches on
2026-11-01. File lists checked for [#1554](https://github.com/storytold/lightcraft/pull/1554)
and [#3155](https://github.com/storytold/lightcraft/pull/3155): both change Edit panel
code; #3155 also changes `panels/edit/sliders.rs`. Inspect their diffs and record
possible merge conflicts. Neither describes this reset action. This is a targeted
overlap scan, not a guarantee about every file in every open PR.
Refresh the open-PR scan before publication. #3457 was rejected because it is
labelled duplicate and [#3463](https://github.com/storytold/lightcraft/pull/3463)
already fixes it. See the pilot selection record for other rejected candidates.
