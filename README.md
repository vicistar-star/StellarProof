# ⭐ StellarProof — The Anti-Deepfake Truth Layer for the Stellar Ecosystem

StellarProof is a decentralized anti-deepfake detection and creator authenticity platform built on the Stellar blockchain. It enables creators, journalists, and platforms to register original media on-chain and expose AI-generated or manipulated content using **Soroban smart contracts** — Stellar's native smart contract platform built on Rust/WASM.

By leveraging Stellar's ultra-low transaction fees (~0.00001 XLM), fast 3–5 second finality, and energy-efficient Stellar Consensus Protocol (SCP), StellarProof makes large-scale deepfake detection **affordable, scalable, and tamper-resistant**.

---

## 🔑 Quick Summary

| Property | Value |
|---|---|
| Project Name | StellarProof |
| Goal | Detect deepfakes and verify human-created media authenticity on-chain |
| Blockchain | Stellar Network |
| Smart Contracts | Soroban (Rust/WASM) |
| Frontend | Next.js + TypeScript + Tailwind CSS |
| Storage | IPFS (decentralized) or MongoDB (high performance) |
| AI Detection | On-premise deepfake detection model (TEE-isolated) |
| Trusted Execution | Oracle-driven TEE using AWS Nitro Enclave |
| Monorepo Manager | pnpm |

---

## 🌐 What StellarProof Solves

Deepfakes and AI-generated synthetic media are eroding trust in digital content — fabricated political speeches, fake celebrity videos, forged evidence. StellarProof fights back through:

- **Origin registration** — creators register original media on Stellar before it can be spoofed.
- **AI/human classification** — a TEE-isolated deepfake detection model scores media for synthetic manipulation.
- **Immutable detection records** — detection results are committed on-chain, creating an auditable history.
- **Creator reputation scores** — on-chain track record of a creator's verified-authentic content.
- **Trustless third-party checks** — any platform can query StellarProof to verify media without a central gatekeeper.
- **Dispute & challenge system** — anyone can challenge a verification result, triggering a re-analysis.

---

## 🚀 Core Architecture

```
Media Upload
      │
      ▼
Storage Layer (IPFS / MongoDB)
      │
      ▼
TEE Oracle Worker
      │
      ▼
AWS Nitro Enclave
  ├── Deepfake Detection Model (AI scoring)
  └── Cryptographic Attestation
      │
      ▼
Soroban Smart Contract
      │
      ▼
On-Chain Detection Certificate (Stellar)
      │
      ▼
Creator Reputation Registry
```

---

## 🏗️ Monorepo Structure

```
StellarProof/
├── package.json
├── pnpm-workspace.yaml
├── tsconfig.base.json
│
├── frontend/
│   ├── app/
│   │   ├── layout.tsx
│   │   ├── page.tsx
│   │   ├── api/health/route.ts
│   │   └── creator/submit/page.tsx     # Submit media for deepfake analysis
│   ├── components/
│   │   ├── DetectionResult.tsx         # AI/human verdict display
│   │   └── CreatorScore.tsx            # Reputation score card
│   ├── next.config.ts
│   └── package.json
│
├── contracts/
│   ├── oracle/                         # Verification request + attestation
│   │   ├── src/lib.rs
│   │   └── Cargo.toml
│   ├── detection/                      # Deepfake detection certificate minting
│   │   ├── src/lib.rs
│   │   └── Cargo.toml
│   ├── reputation/                     # Creator reputation scoring registry
│   │   ├── src/lib.rs
│   │   └── Cargo.toml
│   └── registry/                       # Approved TEE + model hash registry
│       ├── src/lib.rs
│       └── Cargo.toml
│
└── packages/
    └── shared/
        ├── types/index.ts
        ├── utils/hash.ts
        └── package.json
```

---

## ⚙️ Key Features

### 🎥 Deepfake Detection Pipeline
- Upload images or video clips for analysis.
- TEE-isolated AI model scores media on a **0–100 authenticity scale**.
- Detection result (score + confidence + model version) is committed on-chain.
- Supports detection of face-swap, voice cloning, and generative AI artifacts.

### 🧠 Trusted AI Execution (TEE Oracle)
- **AWS Nitro Enclaves** isolate the deepfake detection model from tampering.
- The enclave produces a signed attestation proving which model version ran and on what input hash.
- Model hashes are registered on-chain — any model update requires governance approval.

### 🏅 Creator Reputation System
- Every verified-authentic submission increments a creator's on-chain reputation score.
- Disputed or flagged-synthetic submissions reduce the score.
- Reputation is tied to a Stellar public key — portable across platforms.

### ⚖️ Dispute & Challenge System
- Any third party can challenge a detection result within a time window.
- A challenge triggers a re-analysis in a fresh TEE instance.
- Repeated false challenges are penalized via a small XLM stake.

### 📜 On-Chain Detection Certificates

Each certificate contains:

| Field | Description |
|---|---|
| `contentHash` | SHA-256 hash of the submitted media |
| `authenticityScore` | 0–100 score from the detection model |
| `modelVersionHash` | Hash of the exact model that ran |
| `attestationProof` | Cryptographic proof from the Nitro Enclave |
| `timestamp` | Block time of certificate minting |
| `creator` | Stellar public key of the submitter |
| `challengeStatus` | Current dispute state |

### 🧪 Proof-as-a-Service APIs

| Method | Endpoint | Description |
|---|---|---|
| `POST` | `/api/detect/submit` | Submit media for deepfake analysis |
| `GET` | `/api/detect/status/:jobId` | Poll detection job status |
| `GET` | `/api/creator/:stellarKey/reputation` | Fetch creator reputation score |
| `POST` | `/api/detect/challenge/:certId` | Challenge an existing certificate |

---

## 🛠️ Smart Contracts

| Contract | Purpose |
|---|---|
| `contracts/oracle` | Manages detection job submission and TEE attestation flow |
| `contracts/detection` | Mints immutable deepfake detection certificates |
| `contracts/reputation` | Tracks and updates per-creator authenticity reputation scores |
| `contracts/registry` | Governs approved TEE code hashes and AI model versions |

### Detection Manifest Schema

```json
{
  "contentHash": "sha256:...",
  "creator": "G...",
  "timestamp": "2026-03-15T17:00:00Z",
  "mediaType": "video/mp4",
  "metadata": {
    "durationSeconds": 12,
    "resolution": "1920x1080",
    "captureDevice": "iPhone 16 Pro",
    "aiModelClaim": "none"
  }
}
```

### Detection Result Schema

```json
{
  "certId": "uuid",
  "contentHash": "sha256:...",
  "authenticityScore": 94,
  "confidence": 0.97,
  "verdict": "HUMAN_CREATED",
  "modelVersionHash": "sha256:...",
  "attestationProof": "0x...",
  "challengeDeadline": "2026-03-22T17:00:00Z"
}
```

---

## 🧰 Tech Stack

| Component | Technology |
|---|---|
| Blockchain | Stellar Network |
| Smart Contracts | Soroban (Rust/WASM) |
| Frontend | Next.js 15 + TypeScript + Tailwind CSS |
| Storage | IPFS / MongoDB |
| AI Detection | Deepfake detection model (TEE-isolated) |
| Trusted Compute | AWS Nitro Enclave |
| Oracle | Node.js Worker |
| Package Manager | pnpm |

---

## ⚡ Getting Started

### Prerequisites

- Node.js 20+
- Rust (latest stable) + Cargo
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli)
- pnpm
- [Freighter Wallet](https://www.freighter.app/) (for Stellar testnet)

### Installation

```bash
git clone https://github.com/your-org/StellarProof.git
cd StellarProof
pnpm install
```

### Run Frontend

```bash
pnpm dev:frontend
# opens at http://localhost:3000
```

### Build Soroban Contracts

```bash
cd contracts/oracle     && stellar contract build
cd ../detection         && stellar contract build
cd ../reputation        && stellar contract build
cd ../registry          && stellar contract build
```

### Deploy to Testnet

```bash
stellar contract deploy \
  --wasm contracts/detection/target/wasm32-unknown-unknown/release/detection.wasm \
  --network testnet
```

---

## 🌍 Use Cases

| Domain | Application |
|---|---|
| Political media | Verify authenticity of speeches and press footage before publication |
| Journalism | Newsrooms confirm source video hasn't been synthetically altered |
| Social platforms | Integrate StellarProof API to label AI-generated content at upload |
| Legal evidence | Establish chain of custody and rule out synthetic manipulation |
| Creator monetization | Verified-human creators earn a trust badge boosting audience confidence |
| Insurance & finance | Verify identity videos submitted for KYC aren't deepfaked |

---

## 🗺️ Roadmap

| Phase | Description |
|---|---|
| Phase 0 | Architecture design — detection schema, Soroban contract interfaces, TEE model selection |
| Phase 1 | MVP — upload UI, basic deepfake scoring, on-chain certificate minting |
| Phase 2 | Reputation system — creator scoring, on-chain history, public reputation API |
| Phase 3 | Dispute system — challenge flow, re-analysis, XLM stake mechanism |
| Phase 4 | Model governance — on-chain model hash registry, community upgrade voting |
| Phase 5 | Platform integrations — SDK, webhooks, social media plugin APIs |

---

## 🤝 Contributing

1. Fork the repository.
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Commit your changes: `git commit -m 'Add my feature'`
4. Push: `git push origin feature/my-feature`
5. Open a Pull Request.

---

## 📄 License

[MIT License](./LICENSE)

---

## 🙏 Acknowledgments

- Built on the **Stellar Blockchain** — [stellar.org](https://stellar.org)
- Powered by **Soroban Smart Contracts** — [developers.stellar.org](https://developers.stellar.org)
- Inspired by the fight against synthetic media manipulation

---

## ❤️ Vision

StellarProof aims to become the **universal anti-deepfake trust layer** for digital media across the Stellar ecosystem — giving creators, journalists, and platforms a cryptographically verifiable answer to the question:

> *"Is this real?"*
