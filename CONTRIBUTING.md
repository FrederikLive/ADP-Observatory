# Contributing

Read README.md, AGENTS.md, and the applicable documents in docs/ before making changes.

Keep pull requests focused. Explain the problem, proposed behavior, compatibility impact, security/privacy implications, tests, and limitations. Do not claim planned features are implemented. Add fixtures for supported formats and regression cases for parser/rule changes.

Before submitting, run:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets --all-features`
- `cargo build --release`

Do not add telemetry, remote uploads, write-capable target-project operations, or downloaded executable plugins without an explicit design review. Normative ADP changes belong in the separate canonical ADP repository.
