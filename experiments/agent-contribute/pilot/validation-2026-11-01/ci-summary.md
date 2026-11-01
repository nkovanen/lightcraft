<!-- lightcraft ci report, schema 1 -->
**`cargo xtask ci`: fail** at 150d4057a2 (clean)

| | |
|---|---|
| Machine | linux x86_64 (Ubuntu 24.04.5 LTS, kernel 6.17.0-1033-oem), 8 CPUs, 31 GB RAM (16.4 GB available) |
| Rust | rustc 1.95.0 (59807616e 2026-04-14) |
| GPU for the pipeline | probe failed: not probed: the test step did not run |
| Data | corpus: 0 of 451 pinned raws, 0 images, 0 PngSuite files; no CRAFT_FONTS_DIR |

| Step | Result | Time |
|---|---|---|
| fmt | ok | 3.1 s |
| clippy | fail | 2m 53s |
| heif | not run |  |
| test | not run |  |
| parity | not run |  |
| layers | not run |  |
| assets | not run |  |
| wasm | not run |  |

Skipped tests: unknown, the test step did not run.
