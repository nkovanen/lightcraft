# Agent Contribution Protocol experiment

A contributor-side Python 3.10+ CLI using only the standard library, Git, and
GitHub's `gh` CLI. GitHub remains the source of truth. This is isolated tooling,
not part of LightCraft's Rust product. It never writes to GitHub, pushes, opens
PRs, invokes a model, or runs validation commands.

Run from this directory without installation:

```console
python3 -m agent_contribute discover owner/repo
python3 -m agent_contribute inspect 'owner/repo#123'
python3 -m agent_contribute prepare 'owner/repo#123' --output task.json
python3 -m agent_contribute verify task.json --evidence /outside/checkout/evidence.json --repo-dir /path/to/checkout
python3 -m unittest discover -v
```

Authenticate `gh` for github.com separately (tested with gh 2.45.0 and Python 3.12.3 on Linux). Optional installation with
`python3 -m pip install .` provides the same commands as `agent-contrib`.
There are no Python runtime dependencies; installation uses setuptools.
All paths are local. `prepare` creates a new file exclusively and refuses to
replace existing files, including symlinks. Keep evidence outside the checkout
so writing it does not invalidate the clean-worktree check.

## Commands and exit status

| Command | JSON result |
| --- | --- |
| `discover owner/repo` | Open issues ranked by deterministic difficulty, ambiguity, dependency and likelihood heuristics, with explanations, exclusions, PR overlap warnings, repository instructions and completeness. |
| `inspect owner/repo#N` | One issue, immutable repository policy/instruction snapshot, approval provenance, open PR overlaps, errors and rankings. |
| `prepare owner/repo#N --output FILE` | Saves a structurally valid **draft**, with blank objective, acceptance and validation. Returns the contract and output path. |
| `verify FILE [--evidence FILE] [--repo-dir DIR]` | Strict schema validation, live GitHub refresh, execution authorization, passive evidence, independent review and human signoff gates. |

JSON goes to stdout, including expected errors. `--help` is ordinary text.
Exit 0 means a complete read/draft creation, or final verification ready for the
explicitly selected target. Exit 1 means incomplete reads, input/API errors or
verification not ready; exit 2 means invalid command usage. Saving a draft does
not mean the task is approved or executable. No offline readiness mode exists:
failed live refresh leaves execution and submission gates false.

## Portable adapter and installed workflow

```console
agent-contrib --version
agent-contrib instructions
agent-contrib skill --output /chosen/skills/agent-contribute/SKILL.md
```

Create the destination directory first; export refuses to overwrite a file.
The canonical adapter is packaged once at
[`adapters/agent-contribute/SKILL.md`](agent_contribute/adapters/agent-contribute/SKILL.md).
The installed `instructions` command returns versioned, agent-independent
workflow guidance from [`instructions.md`](agent_contribute/instructions.md),
CLI version and supported schema versions. It works from any directory.

The workflow and trust boundaries live in that packaged resource. In particular,
ordinary issues are candidates, fork execution and upstream eligibility are
separate, evidence is self-attested and no command executes remote instructions.
Task and evidence schemas live in the package. The CLI version is `0.1.0`;
only exact schema version `0.1` is supported. Updates are explicit; no updater or
schema migration runs automatically. This POC is not on a package registry.

Run tests with `python3 -m unittest discover -v` from this directory. The package
can be extracted independently; root product code, dependencies and CI do not
participate in this experiment.

## Pilot and extraction evidence

The contributor workflow is documented in [docs/workflow.md](docs/workflow.md);
[docs/distribution.md](docs/distribution.md) describes standalone packaging, semantic
versioning and controlled updates. The canonical real task is
[tasks/lightcraft-3492.json](tasks/lightcraft-3492.json), with a
[readable brief](tasks/lightcraft-3492.md).
[Live discovery and selection](pilot/discovery-summary.json) records the completed
GitHub scan, duplicate rejection and candidate-only approval state. The pilot
execution and validation report is in [pilot/REPORT.md](pilot/REPORT.md).

## Repository opt-in

A maintainer can add this policy to the repository's default branch and apply a
dedicated label to explicitly approved issues:

```json
{
  "schema_version": "0.1",
  "agent_approved_labels": ["agent-approved"],
  "label_authority": "repository-label-permission"
}
```

Save it as `.agent-contrib.json`. This deliberately delegates labeling authority
to accounts the repository permits to label issues. Ordinary `bug`, `help wanted`
and `good first issue` labels never authorize an agent contribution. Without this
policy, discovered work remains a candidate. The tool does not install the policy
or create labels. See [the workflow](docs/workflow.md#maintainer-integration) for
the maintainer's remaining responsibilities.

Latest follow-up: [final-commit CI, fresh-agent trial and unrelated-repository results](pilot/validation-2026-11-01/REPORT.md). The bare-request fork trial required one parent-discovery hint; PR #2 full CI failed at clippy. Both remain drafts.
