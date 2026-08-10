# Release process

## Release gates

1. Confirm the pinned OpenAPI commit and run the contract conformance test.
2. Update the version and changelog together.
3. Run formatting, Clippy, tests, documentation, dependency policy and `cargo package`.
4. Inspect the `.crate` archive and its checksum.
5. Confirm repository protection, crates.io ownership and recovery contacts.
6. Create a signed tag only after review.
7. Publish through crates.io trusted publishing after the package owner approves the release.
8. Verify the package from a clean consumer project before announcing it.

Beta releases may add contract-backed operations and fields. Breaking API
changes require a documented migration and a SemVer major change after 1.0.
