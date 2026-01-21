# Simple Anchor App

A simple Anchor application for Solana with code coverage support using Surfpool.

## Prerequisites

- [Anchor](https://www.anchor-lang.com/)
- [Surfpool](https://github.com/txtx/surfpool) with register-tracing feature
- [sbpf-coverage](https://crates.io/crates/sbpf-coverage)

## Build

### Build Surfpool with register-tracing

```bash
git clone https://github.com/txtx/surfpool.git
cd surfpool
cargo build --features register-tracing --release
```

### Configure Cargo.toml for coverage

To generate accurate coverage reports, you need to disable optimizations, enable debug symbols, and disable LTO in your workspace `Cargo.toml`:

```toml
[profile.release]
overflow-checks = true
lto = "off"
codegen-units = 1
debug = true
opt-level = 0

[profile.release.build-override]
opt-level = 0
incremental = false
codegen-units = 1
debug = true
```

### Build the Anchor project

```bash
anchor build
rm target/deploy/simple_anchor_app.*
cargo build-sbf --tools-version v1.51 --arch v1 --debug
```

> **Why this complexity?** Anchor is opinionated and always builds with SBPFv0. We run `anchor build` first to generate the IDL and other artifacts, then remove the compiled program and rebuild it manually with `cargo build-sbf` using SBPFv1 (`--arch v1`) for better coverage results.

> **Note:** At the time of writing, best coverage results are achieved with SBPFv1 (dynamic stack frames), which is why we use `--arch v1`. Only with dynamic stack frames can we safely disable optimizations (`opt-level = 0`) without hitting stack size limits. The `--tools-version` can be v1.51 or higher, and `--debug` is required for coverage to work.

> **Warning:** Due to cleaning steps (`anchor clean` or `cargo clean`), the keypair and program ID may get out of sync. If you encounter issues, it's best to clean everything and start fresh with `anchor clean`, then run `anchor keys sync` to resync the keys, followed by the full build process.

## Run Tests

### Start Surfpool

Run this in the anchor project directory:

```bash
SBF_OUT_DIR=$PWD/target/deploy SBF_TRACE_DIR=$PWD/target/sbf_trace_dir surfpool-tracing start
```

> **Note:** Setting `SBF_TRACE_DIR` is what signals `LiteSVM` to enable register tracing dumps. The `SBF_OUT_DIR=$PWD/target/deploy` environment variable won't be necessary once Surfpool catches up with [LiteSVM's ELF data reading from program accounts](https://github.com/LiteSVM/litesvm/pull/278).

### Run Anchor tests

Use Surfpool instead of solana-test-validator:

```bash
anchor test --skip-local-validator --skip-build
```

## Generate Coverage

Install sbpf-coverage if not already installed:

```bash
cargo install sbpf-coverage
```

Generate and view coverage report:

```bash
sbpf-coverage --src-path=$PWD/programs/simple_anchor_app/src --sbf-path=$PWD/target/deploy --sbf-trace-dir=$PWD/target/sbf_trace_dir
genhtml --output-directory coverage target/sbf_trace_dir/*.lcov --rc branch_coverage=1 && open coverage/index.html
```

## License

[MIT](LICENSE) © [LimeChain](https://limechain.tech)
