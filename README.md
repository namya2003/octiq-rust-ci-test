# octiq-rust-ci-test

Rust fixture whose **only** statement that it owns tests is its CI
(`cargo test`). No `octiq.verify.json`.

Used to exercise the indexed sync path where the suite's command comes from
project-edit-agent's CI policy, not from a contract or a recipe default.
