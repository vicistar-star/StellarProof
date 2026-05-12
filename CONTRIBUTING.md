# Contributing to StellarProof

Thanks for your interest in contributing. This document covers everything you need to get started.

---

## Prerequisites

| Tool | Version |
|---|---|
| Node.js | 20+ |
| pnpm | 9+ |
| Rust + Cargo | latest stable |
| Stellar CLI | latest |

Install Stellar CLI:
```bash
cargo install --locked stellar-cli --features opt
```

---

## Setup

```bash
git clone https://github.com/vicistar-star/StellarProof.git
cd StellarProof
pnpm install
cp .env.example .env.local   # fill in values as needed
```

Run the frontend:
```bash
pnpm dev:frontend             # http://localhost:3000
```

Build all Soroban contracts:
```bash
pnpm build:contracts
```

Run contract tests:
```bash
cd contracts/detection && cargo test
```

---

## Project Structure

```
frontend/          Next.js 15 app — UI + API routes
contracts/         Soroban smart contracts (Rust)
  detection/       Certificate minting
  oracle/          Job submission + TEE attestation
  reputation/      Creator score tracking
  registry/        Approved TEE/model hash governance
packages/shared/   Shared TypeScript types and utilities
scripts/           Build and deploy helpers
```

---

## What Needs Work (open areas)

The project is at ~60% completion. Key areas open for contribution:

| Area | Description |
|---|---|
| TEE Oracle worker | Node.js worker that polls jobs and calls the Nitro Enclave |
| Stellar RPC integration | Wire API routes to on-chain contracts via `@stellar/stellar-sdk` |
| Storage layer | IPFS upload + MongoDB job persistence |
| Contract tests | `oracle`, `reputation`, `registry` contracts need unit tests |
| Dispute UI | Frontend flow for challenging a certificate |
| Reputation page | `/creator/[stellarKey]` page using `CreatorScore` component |

Check [open issues](https://github.com/vicistar-star/StellarProof/issues) for specific tasks.

---

## Workflow

1. Fork the repo and create a branch: `git checkout -b feat/your-feature`
2. Make your changes — keep commits focused and descriptive
3. For contract changes, run `cargo test` in the relevant contract directory
4. For frontend changes, run `pnpm lint` and verify `pnpm build:frontend` passes
5. Open a PR against `main` with a clear description of what changed and why

### Commit style

Use conventional commits:

```
feat(contracts): add dispute resolution to oracle contract
fix(frontend): correct score bar width calculation
chore: update soroban-sdk to 22.1.0
test(contracts): add reputation record_synthetic test
```

### PR checklist

- [ ] `pnpm lint` passes (frontend)
- [ ] `cargo test` passes (any modified contract)
- [ ] No `.env` or secret values committed
- [ ] PR description explains the change and links to an issue if applicable

---

## Smart Contract Guidelines

- All contract entry points must call `require_auth()` on privileged callers
- Use `persistent` storage for user/certificate data, `instance` for contract-level counters
- Add a `#[cfg(test)]` module in `src/test.rs` for every new entry point
- Do not introduce new crate dependencies without discussion — WASM binary size matters

---

## Questions

Open a [GitHub Discussion](https://github.com/vicistar-star/StellarProof/discussions) or file an issue.
