# Contributing

Contributions are welcome.

1. Start with an issue for contract or public API changes.
2. Keep generated behavior aligned with the pinned OpenAPI contract.
3. Add tests for authentication, errors, retries and idempotency when affected.
4. Run `cargo fmt --check`, Clippy, tests, documentation and `cargo package`.
5. Do not include API keys or personal, health, payment or production data.

All changes require review from the owners in `.github/CODEOWNERS`. The
[organization Code of Conduct](https://github.com/heyrafiki/.github/blob/main/CODE_OF_CONDUCT.md)
applies.
