# comet-flags vX.Y.Z

Sovereign feature-flag engine for NeuroPC and Organic_CPU, enforcing neurorights, RoH ≤ 0.3, and safer-only updates via ALN schemas, sovereigntycore guards, and donutloop audit trails. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

## Release summary

- Scope: `comet_flags` Rust crate, `*.comet-flags.aln`, `*.comet-profile.aln`, and associated CI policies.  
- Guarantees: auditability (`.evolve.jsonl` → `.donutloop.aln`), cross-repo profile consistency, monotone safety envelopes, RoH ≤ 0.3 at runtime. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/0d037bf3-6e87-4250-bc42-f5facdec403a/what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md)
- Compatibility: NeuroPC, Organic_CPU, Reality.os stacks using the canonical `.rohmodel.aln`, `.stake.aln`, `.evolve.jsonl`, `.donutloop.aln` layout. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

## Artifacts

- Crate: `crates/comet_flags` (Rust)  
- Schemas:  
  - `qpudatashards/particles/comet-flags.schema.aln`  
  - `qpudatashards/particles/comet-profile.schema.aln` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/2da95299-e40a-4432-ac53-a60d61c3fd08/legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md)
- Policies (subject-scoped):  
  - `policies/bostrom-comet-flags-v1.comet-flags.aln`  
  - `policies/bostrom-comet-profiles-v1.comet-profile.aln` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
- CI: `.github/workflows/comet-flags-ci.yml` (schema validation, invariants, cross-repo checks). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

## Invariants enforced in this release

- Mental privacy:
  - Flags may only reference normalized / derived metrics (`BioState.*`, QPU shards), never raw biophysical streams. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/2da95299-e40a-4432-ac53-a60d61c3fd08/legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md)
  - `data_sensitivity != "raw_biophysical"` is enforced at schema load and in CI. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/2da95299-e40a-4432-ac53-a60d61c3fd08/legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md)

- Cognitive liberty:
  - CONSERVATIVE / SAFE_BASELINE / COPILOT / AUTOEVOLVE profiles are defined and semantically identical across repos. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
  - SAFE_BASELINE forbids experimental flags and is always available as a fallback profile. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

- Monotone safety envelopes:
  - Safety parameters tied to flags (e.g., pain thresholds, lifeforce limits) have `monotone_constraint` fields; updates that loosen envelopes are rejected. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/2da95299-e40a-4432-ac53-a60d61c3fd08/legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md)
  - CI simulates proposals in `.evolve.jsonl` and fails on any safety regression. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

- RoH ≤ 0.3:
  - All flag/profile resolutions pass through sovereigntycore’s `RiskOfHarm` check using `.rohmodel.aln`; profiles requiring RoH gating activate only if `RoH <= 0.3`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/2da95299-e40a-4432-ac53-a60d61c3fd08/legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md)
  - Tests assert no accepted evolution event raises RoH ceiling or bypasses the global 0.3 limit. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

## Audit and governance wiring

- Proposals:
  - Every comet change is a line in `qpudatashards/particles/evolution-proposals.evolve.jsonl` with `kind: "FlagProfileUpdate"`, `rohbefore`, `rohafter`, `decision`, `hexstamp`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

- Ledger:
  - Accepted changes are mirrored into `logs/donutloopledger.aln` with `proposalid`, `changetype`, `rohbefore`, `rohafter`, `policyrefs`, `prevhexstamp`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

- Stake and neurorights:
  - `.stake.aln` defines which roles (Host, OrganicCPU, ResearchAgent) must sign off on comet-related updates (lifeforce/archchange scopes require multisig). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
  - Neurorights policies (mental privacy, integrity, cognitive liberty, non-commercial neural data) are enforced before comet-flags can expose new active sets. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/2da95299-e40a-4432-ac53-a60d61c3fd08/legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md)

## Cross-repo checks

This release expects:

- Presence of canonical comet artifacts in each repo:
  - `policies/bostrom-comet-profiles-v1.comet-profile.aln`  
  - `policies/bostrom-comet-flags-v1.comet-flags.aln` [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
- A shared index (`neurofs-index.aln` or equivalent) listing artifact versions and hashes.  
- CI to:
  - Verify profile IDs and forbidden flag lists match the canonical copy.  
  - Run the same label-based profile resolution tests across NeuroPC, Organic_CPU, and Reality.os. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/0d037bf3-6e87-4250-bc42-f5facdec403a/what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md)

## Upgrade notes

- Migration steps:
  - Ensure `.rohmodel.aln`, `.stake.aln`, `.evolve.jsonl`, `.donutloop.aln` are at or above the minimum versions required by this crate (see `Cargo.toml` and `neurofs-index.aln`). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
  - Run `comet-flags-validate` locally on modified `.comet-flags.aln` / `.comet-profile.aln` before pushing.

- Backwards compatibility:
  - Existing profiles without `monotone_constraint` fields are treated as non-safety-critical and remain valid, but will not gate safety parameters. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/2da95299-e40a-4432-ac53-a60d61c3fd08/legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md)
  - RoH ceiling remains hard-coded as 0.3 in `.rohmodel.aln` and sovereigntycore; raising it is not supported in this release. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

## Verification checklist for this release

Before tagging `vX.Y.Z`, verify:

- [ ] All comet-related ALN files parse and pass schema validation.  
- [ ] Unit tests for mental privacy, cognitive liberty, monotone envelopes, and RoH ≤ 0.3 all pass.  
- [ ] CI monotonicity simulation over `.evolve.jsonl` passes with no safety regressions.  
- [ ] Cross-repo CI confirms profile consistency across NeuroPC / Organic_CPU / Reality.os.  
- [ ] Donutloop ledger integrity check passes (no broken `prevhexstamp` chain).  

Hex-stamp for this release note (logical, not cryptographic): `0xCF_COMET_FLAGS_RELEASE_vX_Y_Z`.
