# PayMe Contracts

PayMe is a non-custodial enterprise payment protocol on Stellar that combines multi-signature DAO treasury controls with automated payroll and milestone-aware escrow execution.

This repository contains the Soroban smart contract workspace that powers on-chain policy enforcement for the PayMe stack.

## Why This Matters

- Treasury security: weighted signer approvals for sensitive DAO payment actions.
- Deterministic payroll rails: programmable payment state transitions in Soroban.
- Indexer and client readiness: contracts are built to support type-safe bindings for the NestJS indexer and Next.js frontend.

## Architecture Context

```text
PayMe Client (Next.js) -> PayMe Server (NestJS Indexer) -> Stellar Soroban RPC
															|                                  |
															+------ event indexing ------------+
```

## Workspace Layout

```text
.
├── Cargo.toml
├── rust-toolchain.toml
├── rustfmt.toml
├── .github/
│   └── workflows/
│       └── ci.yml
└── contracts/
		└── payme_escrow/
				├── Cargo.toml
				└── src/
						├── lib.rs
						└── test.rs
```

## Contract Modules

- `contracts/payme_escrow`: baseline payroll escrow state machine with one-time initialization, admin storage, signer weight map, and threshold persistence.

## Local Development

### Prerequisites

- Rust stable (toolchain pinned via `rust-toolchain.toml`)
- Stellar CLI

Install Stellar CLI:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://developer.stellar.org/releases/stellar-cli/install.sh | sh
```

### Format, Lint, and Test

```bash
cargo fmt --manifest-path Cargo.toml --all -- --check
cargo clippy --manifest-path Cargo.toml --workspace --all-targets -- -D warnings
cargo test --manifest-path Cargo.toml --workspace --all-targets
```

### Build Wasm Artifacts

```bash
stellar contract build
```

## TypeScript Bindings

Generate Soroban TypeScript bindings for downstream repositories:

```bash
stellar contract bindings ts \
	--wasm target/wasm32-unknown-unknown/release/payme_escrow.wasm \
	--output-dir ../payme-client/src/bindings/payme-contract
```

## CI Quality Gates

The CI pipeline in `.github/workflows/ci.yml` enforces:

- Formatting (`cargo fmt --check`)
- Linting (`cargo clippy -D warnings`)
- Unit tests in Soroban simulation environment
- Reproducible Soroban wasm build

## Baseline Testing Strategy

The escrow contract test suite currently validates:

- deterministic contract registration and client bootstrapping
- signer map and threshold initialization
- persisted state readbacks
- rejection of duplicate initialization

This baseline is intended to grow into milestone streaming, release schedule validation, and weighted multi-sig authorization path tests.

## Security and Production Readiness Notes

- Keep contract APIs narrow and explicit; avoid overloading write methods.
- Add fuzz/property tests before mainnet deployment.
- Run independent security review before production protocol upgrades.
- Use semantic versioning for contract interface changes.

## License

Private project at this stage. Add a formal OSS/commercial license before public release or grant submission.