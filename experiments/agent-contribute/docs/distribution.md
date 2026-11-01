# Distribution and compatibility

The implementation is an extractable Python distribution. The CLI/library owns
GitHub reads, discovery, contract parsing, compatibility checks and verification.
Its installed protocol instructions own the workflow. The Skill is a small adapter
that asks the installed CLI for that guidance and uses the existing agent's tools.
It contains no task-selection algorithms, repository names, validation commands or
approval rules specific to any project.

## Install once

Install a reviewed, pinned release of `agent-contrib` into a virtual environment or
an equivalent isolated application installer. Put its executable on PATH. Export
the bundled Skill into the host's supported skill directory once using
`agent-contrib skill --output PATH/agent-contribute/SKILL.md`. The directory must
already exist. Every later repository request uses the same installation.

`agent-contrib instructions` returns the installed workflow and compatibility
metadata. `agent-contrib --version` identifies the CLI release. Hosts that do not
load skills can read the exported adapter explicitly; implementing new harness
adapters does not require changing the protocol or integrating a model provider.
This POC does not automatically configure every host's skill search path.

For the POC, installation uses this directory in the LightCraft checkout. After
extraction, the same directory is the standalone repository root: pyproject.toml,
the package, bundled schemas/instructions/adapter, tests, documentation and license
texts are self-contained. The `tasks/` and `pilot/` directories are examples and
evidence, not imported dependencies. No Python code imports a LightCraft module or
locates the parent Cargo workspace. Tests use synthetic GitHub repositories and
temporary Git checkouts; live network access is an explicit integration exercise.

## Version and update policy

- CLI/library releases use semantic versions, starting at `0.1.0`. During `0.x`,
  a minor release may break compatibility; patch releases preserve supported
  contract semantics. At `1.0`, breaking public interfaces require a major version.
- Contract and evidence schema versions are separate from package versions. v0.1
  supports exactly schema string `0.1`. Unknown versions fail closed. Do not treat
  a newer JSON object as compatible merely because its fields look familiar.
- The installed release bundles its own schemas and instructions. Verification
  never downloads a replacement schema or executes an update offered by a task.
- Updates are explicit: inspect release notes and source, choose a pinned version
  (or immutable source commit), install it, run its tests and re-export the adapter
  only after comparing changes. There is no automatic update or mutation of an
  installed Skill. Existing exported files are not silently overwritten.
- A future migration command must produce a new contract file and require review;
  changing `schema_version` by hand is not a migration. Hash-bound evidence must
  be regenerated when a reviewed contract changes.

Repository requirements are discovered at runtime from the requested GitHub
repository's pinned default branch. The repository may change its policy between
discovery and submission; verification refreshes the relevant source data and
approval. This is separate from updating the installed tool itself.

## POC boundary

The installed CLI and portable skill support manual installation in existing
harnesses. A public package release, signed release artifacts, managed cross-host
installation, migration tooling and protected-CI attestations are future work.
The POC demonstrates extraction by installing a copy outside LightCraft and running
the package and tests there; see the pilot report for the actual result.
