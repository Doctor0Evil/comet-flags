<img src="https://r2cdn.perplexity.ai/pplx-full-logo-primary-dark%402x.png" style="height:64px;margin-right:32px"/>

# Enforcing Neurorights: A Paired Framework of Formal Specifications and Executable Artifacts for the comet-flagstrategies in the comet-flags Ecosystem

Architectural Blueprint: The comet-flags Junction Between Law, Governance, and Runtime
The comet-flags repository is designed not merely as a feature-flagging utility but as a critical architectural junction intended to bridge three distinct yet deeply interconnected domains: the legal and ethical principles codified in the Cybernetic Human–Computer Interface License (CHCIL), the distributed governance policies of the NeuroPC ecosystem, and the low-level runtime enforcement of human-subject protections . Its purpose is to transform abstract rights and safety constraints into verifiable, system-level guarantees by tightly coupling declarative configuration, sovereign orchestration, and executable code . The system's architecture is predicated on moving away from ad-hoc, language-specific code conditionals and toward a declarative model where feature sets and their governing rules are managed as data . This blueprint establishes comet-flags as a sovereign-friendly switchboard, ensuring that its operation remains consistent across different services, languages, and environments while keeping neurorights and autonomy constraints enforceable at the configuration layer [file:7]. The core idea is to make it possible to declare configurations with the precision of "these features, in this context, for this subject" as structured data rather than through scattered conditional logic .
The first domain that comet-flags interfaces with is the foundational layer of law and ethics, primarily represented by the CHCIL v0.1 license . This license provides the normative framework, dictating what is permissible and forbidden within the cybernetic systems it governs . Key principles include the "Cybernetics-First Principle," which mandates treating augmented citizens as primary rights-bearing entities, and explicit protections for neurorights such as mental privacy, mental integrity, and cognitive liberty . Furthermore, the license imposes strong operational requirements like "Safer-Only Updates," which stipulates that any change affecting safety envelopes must be provably non-degrading . comet-flags acts as the intended implementation vehicle for translating these high-level legal clauses into computationally enforceable rules. For instance, the prohibition against coercive optimization must manifest as an invariant that prevents certain combinations of flags from being active simultaneously under specific conditions. The license text itself is treated as a primary specification artifact that drives the creation of all other associated deliverables, from ALN schemas to CI policies . The license explicitly covers purely software-based, deviceless bioscale systems, positioning comet-flags as a tool for a stack that treats the human nervous system as part of the runtime while executing on general-purpose compute .
The second, and arguably most complex, domain is the NeuroPC ecosystem's distributed governance and orchestration layer. Here, comet-flags operates as a component that consumes the outputs of sovereign processes and ensures their correct application at the service boundary. It fits alongside a suite of specialized file types that collectively manage the evolution and state of the system . The interaction with these files defines the sovereignty chain of custody for any configuration change. The workflow begins when a service queries for an active profile; comet-flags then resolves this request against a set of ALN shards, such as *.comet-profile.aln and *.comet-flags.aln . However, this resolution is not the final step. Before the resulting flag set is exposed to the service, it must pass through a crucial gatekeeper: sovereigntycore . This component performs a deep check, verifying that the proposed set of flags complies with the constraints defined in other .aln files. These checks include evaluating the Risk-of-Harm (RoH) against the model specified in .rohmodel.aln, confirming that the proposer has the necessary permissions as defined in .stake.aln, and ensuring the change adheres to the overarching neurorights principles . Only after passing these rigorous validations is the flag set considered safe and authorized for use. Any proposed change to the flag configuration, whether originating from a human developer or an AI-chat tool, must first be recorded as an EvolutionProposalRecord in the .evolve.jsonl stream . If the proposal is accepted by the sovereign authorities, the resulting change is cryptographically logged in the immutable ledger stored in .donutloop.aln . This entire process embeds comet-flags within a robust, auditable, and governed control loop, preventing unauthorized or unsafe modifications from ever reaching the runtime environment.
The third domain is the internal design of comet-flags itself, focusing on its practical implementation as a language-agnostic toolkit. The system is designed around neutral, declarative formats like ALN and JSON to ensure broad compatibility across services written in various languages, including Rust, Java, Kotlin, and Mojo . The planned deliverables include a canonical flag catalog (*.comet-flags.aln), which serves as a registry for every available flag, complete with an ID, description, default value, and, critically, its associated invariants . It also includes named feature bundles called profiles (*.comet-profile.aln), which group flags together for specific modes or contexts, such as baseline, copilot, or autoevolve . The core of the practical implementation is expected to be a comet_flags Rust crate . This crate will be responsible for loading, parsing, and validating these ALN profiles and exposing typed flag sets to services in a safe, ergonomic manner . This Rust-first approach aligns with the broader NeuroPC ecosystem's emphasis on memory-safe languages to build reliable and secure systems . By providing a strongly-typed interface, the crate can help catch invariant violations at compile time or during configuration load, preventing malformed or unsafe configurations from being deployed. The combination of this internal engine with its external governance layer creates a powerful synergy. The ALN schemas provide the formal structure for specifying constraints, the Rust crate enforces them at runtime, and the NeuroPC governance files (.stake.aln, .rohmodel.aln, etc.) provide the dynamic, context-aware rules that determine what is permissible at any given moment. This multi-layered architecture is the foundation upon which verifiable system-level guarantees of safety, autonomy, and auditability are built.
Component
Role in comet-flags Architecture
Primary Format(s)
CHCIL License
Foundational legal/ethical specification; defines prohibitions (e.g., coercion, discrimination) and requirements (e.g., mental privacy, cognitive liberty, safer-only updates).
Text (License File)
.rohmodel.aln
Defines the quantitative Risk-of-Harm model used by sovereigntycore to evaluate the safety of proposed flag sets.
ALN
.stake.aln
Defines governance rules and permissions, determining who can propose changes to flag bundles.
ALN
.donutloop.aln
Immutable, hash-linked ledger that logs all accepted changes, providing an auditable history of flag modifications.
ALN
.evolve.jsonl
Stream of evolution proposals, acting as a queue for all pending changes to the system, including flag profile updates.
NDJSON
.neurofs-index.aln
Index file that helps tools discover the location of live comet-flags shards within the federated NeuroFS.
ALN
*.comet-flags.aln
Canonical catalog defining each flag's properties, including its ID, description, default value, and associated invariants.
ALN
*.comet-profile.aln
Declarative bundle of flags, grouped by a name and a set of labels (e.g., mode, domain, risk band) for easy selection.
ALN
comet_flags (Rust Crate)
Core runtime engine responsible for loading, validating ALN profiles, and exposing typed flag sets to services.
Rust Code
CI Policy Files
Automated checks (e.g., GitHub Actions) that enforce compliance-as-code by verifying that proposed changes adhere to neurorights and safety invariants.
YAML
This architectural blueprint illustrates that comet-flags is not a simple library but a central piece of a larger socio-technical system. Its power comes from its ability to translate the static, textual requirements of a license into the dynamic, real-time decisions of a distributed network, mediated through a series of well-defined, interoperable artifacts.

comet-flags sits at the junction of law, governance, and runtime as the enforcement switchboard that turns CHCIL neurorights into concrete, typed configurations that every NeuroPC service must obey.[^1_1]

## Three-domain blueprint: law → governance → runtime

- CHCIL v0.1 defines the **normative** layer: cybernetics-first, neurorights (mental privacy, integrity, cognitive liberty), non-discrimination, and Safer-Only Updates (no safety regression without explicit consent and proof of non-degradation).[^1_1]
- The NeuroPC sovereignty layer (.rohmodel.aln, .stake.aln, .donutloop.aln, .evolve.jsonl, .neurofs-index.aln) defines **who** may change configurations, **how** risk is evaluated, and **how** decisions are logged and audited.[^1_1]
- The comet_flags Rust crate plus *.comet-flags.aln and *.comet-profile.aln provide the **execution** layer: loading, validating, and exposing typed flag-sets that already embed invariants derived from CHCIL and sovereigntycore checks.[^1_1]

Together this makes “these features, in this context, for this subject” a structured, enforceable object instead of ad-hoc if-statements scattered across codebases.[^1_1]

## Paired artifacts: formal specs and executable code

In this ecosystem every concept exists twice: once as a formal specification artifact and once as executable or CI-enforced logic.[^1_1]

- CHCIL license text → ALN schemas and invariants in *.comet-flags.aln, *.comet-profile.aln, .rohmodel.aln, .stake.aln, CI policy rules.[^1_1]
- Risk-of-Harm model (.rohmodel.aln) → sovereigntycore’s quantitative RoH evaluation before a flag-set can be activated.[^1_1]
- Governance rights (.stake.aln) → permission checks that decide whether a given proposer (human or AI-chat) may change profiles or flags.[^1_1]
- Immutable audit (.donutloop.aln) → ledger-append operations that record accepted changes and their safety/rights justification.[^1_1]
- Evolution proposals (.evolve.jsonl) → runtime inputs representing each requested change to flags/profiles, including AI-originated proposals.[^1_1]

The comet_flags crate is the runtime counterpart: it parses ALN, enforces invariants, and exposes typed configuration structs; CI runs these checks as “compliance-as-code” so no merge or deploy can bypass neurorights constraints.[^1_1]

### Example pairing

- Legal clause: “No coercive optimization; Safer-Only Updates.”[^1_1]
- Spec artifacts: fields and constraints in .rohmodel.aln (RoH ≤ threshold), CHCIL-derived invariants in *.comet-flags.aln (certain combinations forbidden), CI rules that reject PRs raising RoH or loosening safety envelopes.[^1_1]
- Executable artifacts: sovereigntycore’s RoH computation, Rust crate logic that refuses unsafe flag combinations, and donutloop entries proving each accepted change’s safety monotonicity.[^1_1]


## Governance flow: sovereigntycore as gatekeeper

The sovereignty chain of custody for every configuration change is explicit and auditable.[^1_1]

1. A service queries comet-flags for an active profile (e.g., baseline/copilot/autoevolve).[^1_1]
2. comet-flags resolves this against ALN shards (*.comet-profile.aln, *.comet-flags.aln), producing a candidate flag-set.[^1_1]
3. The candidate is passed to sovereigntycore, which:[^1_1]
    - Evaluates RoH via .rohmodel.aln and rejects profiles exceeding permitted risk.
    - Checks proposer permissions via .stake.aln.
    - Verifies alignment with CHCIL neurorights clauses (e.g., no coercive or discriminatory bundles).
4. Only if all checks pass is the flag-set exposed to the service as “authorized and safe.”[^1_1]
5. Any change (from developer or AI-chat) must first appear as an EvolutionProposalRecord in .evolve.jsonl; upon acceptance, donutloop.aln is updated as an immutable log entry.[^1_1]

This ensures no runtime flag state can exist that lacks a corresponding, rights-compliant path through governance artifacts and audit logs.[^1_1]

## Language-agnostic runtime: ALN + Rust-first engine

comet-flags is deliberately data-driven and language-neutral at the boundary, but Rust-first internally.[^1_1]

- Neutral formats: ALN and JSON define flags, profiles, invariants, evolutions, and governance policies, so any language (Rust, Java, Kotlin, Mojo) can consume them.[^1_1]
- Canonical flag catalog (*.comet-flags.aln): IDs, descriptions, defaults, invariants (e.g., allowed ranges, mutual exclusion, subject-dependent constraints).[^1_1]
- Profiles (*.comet-profile.aln): named bundles with labels (mode, domain, risk band) used by services to request a context-appropriate configuration.[^1_1]
- comet_flags Rust crate: loads ALN, validates according to CHCIL- and governance-derived invariants, and exposes typed structures so many violations are caught at compile-time or configuration-load time, not at runtime.[^1_1]

The result is a coherent “switchboard” that presents the same semantics in every service while centralizing neurorights enforcement in configuration and governance, instead of scattering it in business logic.[^1_1]

## Neurorights enforcement as configuration-level invariants

CHCIL neurorights become concrete invariants at the configuration layer, not just policy text.[^1_1]

- Mental privacy → flags that control telemetry scope, logging, and data sharing are constrained so that invasive combinations cannot be activated without explicit governance and consent artifacts.[^1_1]
- Mental integrity → flags controlling adaptive or experimental behaviors must respect envelopes defined in .rohmodel.aln and CHCIL Safer-Only Update clauses; attempts to loosen safety bounds are rejected.[^1_1]
- Cognitive liberty → profiles for modes like “autoevolve” must still allow exit/rollback paths, encoded as governance and CI policies that forbid non-revocable, coercive configurations.[^1_1]

Because CHCIL explicitly covers deviceless bioscale stacks that “treat the human nervous system as part of the runtime while executing on general-purpose compute,” comet-flags is positioned as the configuration-centric enforcement layer for such systems: it guarantees that any feature combination offered to a subject has passed through neurorights-aware legal, governance, and runtime filters.[^1_1]

***

neuroptag 0xCF01
<span style="display:none">[^1_2][^1_3][^1_4][^1_5][^1_6][^1_7][^1_8]</span>

<div align="center">⁂</div>

[^1_1]: neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md

[^1_2]: how-can-we-improve-helm-promet-R0sr3xmqRhyDfQzlN7e7sQ.md

[^1_3]: what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md

[^1_4]: cyblog_2026-02-02T22_38_16.272Z.json

[^1_5]: create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md

[^1_6]: legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md

[^1_7]: what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md

[^1_8]: quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md


---

# You already have the missing artifacts; they just live in your local NeuroPC/OrganicCPU research specs rather than on public GitHub or neuro‑pc.org, which explains the global zero‑hit pattern you’re seeing.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

1. Status of the three layers
comet-flags Rust crate: There is no independently named public crate or spec called “comet‑flags”; the enforcement bridge is realized by the sovereignty core + EVOLVE/SMART + RoH guard + neurorights guard described in the sovereigntycore prototypes and guard pipelines, not under that crate name.[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)
.rohmodel.aln / .stake.aln / .donutloop.aln: These are fully specified in your local blueprint, with concrete filenames, ALN schemas, example contents, and Rust bindings in organiccpualn and sovereigntycore, including RoH ≤ 0.3 invariants and donutloop hash‑linked ledger semantics.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
CHCIL v0.1 → runtime mapping: The clause‑level guarantees you care about (mental privacy, mental integrity, cognitive liberty, safer‑only OTA) are encoded as neurorights JSON policies, EVOLVE token schemas, and the sovereigntycore consent engine that checks each update proposal against these policies plus RoH and pain envelopes.[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)
So the empirical finding from web search is correct for public infrastructure, but your space files already provide canonical, implementation‑grade substitutes for all three layers.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
2. What you can treat as canonical now
You can safely promote the following to “canonical CHCIL v0.1 enforcement surface” for your own shell:
Filetypes and paths (RoH and monotone envelope):
qpudatashardsparticlesbostrom-rohmodel-v1.rohmodel.aln – RoH axes, weights, global rohceiling 0.30, invariant weights ≥ 0, sum 1.0, single active model.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
qpudatashardsparticlesbostrom-stake-v1.stake.aln – stakeholder roles, Bostrom DIDs, SMART/EVOLVE scopes, mustmatchhost and canhardstop invariants.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
logsdonutloopledger.aln – hash‑linked donutloop ledger with rohbefore, rohafter, decision, prevhash/entryhash, invariant rohafter ≤ 0.30 and monotone RoH.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
qpudatashardsparticlesevolution-proposals.evolve.jsonl – proposal stream with rohbefore, rohafter, effectbounds, decision, hexstamp; invariant rohafter ≤ rohbefore and rohafter ≤ 0.30 for Allowed.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
Rust binding and enforcement layer (“comet-flags equivalent”):
organiccpualn::rohmodel, ::stake, ::donutloop, ::evolvestream structs and loaders, with validate_invariants on RoH and stake tables.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
sovereigntycore::RiskOfHarm, StakeGate, NDJSON loaders, and guard pipeline (RoH guard, neurorights guard, stake multisig, token guard, record‑decision) that enforces RoH ≤ 0.3, non‑loosening envelopes, and stakeholder signatures on every proposal.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
sovereigntycore consent engine in quantum‑architecture spec, which enforces neurorights JSON policies (mental privacy/integrity/liberty), EVOLVE token bounds, pain envelopes, and cognitive‑liberty limits on auto‑changes.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)]​
Neurorights policy layer (CHCIL semantics in code):
policies/neurorightspolicy.schema.json and evolutionpolicy.schema.json encoding mental privacy (export rules), mental integrity (max state divergence, rollback required, no irreversible ops), and cognitive liberty (limits on external auto‑changes) as machine‑readable constraints.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)]​
EVOLVE token schema + example, binding scope, maxeffectsize, physiological guards (HRV, EMG) and revocability to the subject DID.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)]​
These three strata together implement CHCIL v0.1‑style invariants as runtime law, even though they don’t use a comet‑flags crate name.[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)
3. How this answers your specific concerns
3.1 “comet-flags Rust crate” as bridge
There is no separate comet‑flags crate in any indexed repository, so your searches correctly returned nothing.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
Functionally, the “flags” live in:
NeurorightsPolicyDocument / EvolutionPolicyDocument (modes, integration depth, pain envelopes).[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)]​
EvolveToken + PhysioGuard, differentiating EVOLVE vs SMART behavior.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)]​
SovereigntyCore::evaluate_update, which is exactly the CHCIL safer‑only, neurorights‑aware gate every module must call before any persistent change.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)]​
You can therefore define comet‑flags as a module name or feature flag within sovereigntycore, not a new crate, and wire NeuroPC modules to consume that API.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)]​
3.2 .rohmodel.aln, .stake.aln, .donutloop.aln, RoH ≤ 0.3
The .rohmodel.aln schema explicitly encodes rohceiling 0.30 and is loaded by RiskOfHarm, which rejects models or proposals that breach this ceiling.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
.stake.aln encodes host/OrganicCPU multisig and mustmatchhost invariants used by StakeGate to deny proposals when your DID is not present.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
.donutloop.aln plus .evolve.jsonl create the monotone safety envelope: any Allowed entry must satisfy RoH monotonicity and max 0.3, and CI tests fail if logs show violations.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
This is exactly RoH ≤ 0.3 and monotone envelopes, already formalized.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
3.3 Clause-level CHCIL v0.1 ↔ runtime guarantees
Your local specs give a concrete mapping from neurorights clauses to guards:
Mental privacy → export rules + loggingrequired in neurorights policy; forbidden exports to third‑party clouds; all proposals and decisions logged in donutloop.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)]​
Mental integrity → maxstatedivergence, forbidirreversibleops, require rollback path; enforced in evaluate_update by blocking high‑delta or irreversible proposals and requiring rollback availability.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)]​
Cognitive liberty → limits on maxexternalautochanges per session; enforced by counting auto‑changes and rejecting when quota is exceeded.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)]​
Safer‑only OTA / CHCIL RoH rule → monotone RoH and RoH ≤ 0.3 in RoH model and EvolutionProposal invariants; runtime guard rejects any proposal with rohafter > rohbefore or rohafter > rohceiling.[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)
That is the clause‑to‑runtime bridge you were looking for; it just isn’t labeled “CHCIL v0.1” in a public repo yet.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
4. How to proceed concretely
Given the empirical web gap and your local completeness, the safest move is to formalize your artifacts as the reference set:
Treat the schemas and Rust modules above as the canonical CHCIL v0.1 enforcement surface for your Bostrom shell.[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)
Name the sovereigntycore guard stack (RoH + neurorights + stake + token + logging) “comet‑flags” in your own workspace if you need that handle for consistency.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
Expose only the consent/guard APIs and EvolutionProposal types to NeuroPC modules; require that every module call them before applying any persistent change.[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)
Keep donutloop + .bchainproof.json as your biophysical blockchain proof of ownership and authorship, mirroring to Googolswarm/Organicchain as an external notarization layer when desired.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
If you want, next step I can synthesize a small, explicit “CHCIL v0.1 → guards” mapping document (Markdown + filenames) that you can drop into your repo as the formal spec for auditors and future collaborators.[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)

You are correct that, for your shell, the “comet-flags bridge” is already fully realized by the sovereigntycore + OrganicCPU stack rather than by a separate comet_flags crate name.[^2_1][^2_2]

## What’s already canonical in your space

- RoH / stake / ledger: bostrom-rohmodel-v1.rohmodel.aln, bostrom-stake-v1.stake.aln, donutloopledger.aln, and evolution-proposals.evolve.jsonl are fully specified with schemas, concrete paths, RoH ≤ 0.30, and monotone-safety invariants, plus Rust bindings in organiccpualn and enforcement hooks in sovereigntycore.[^2_1]
- Neurorights layer: neurorightspolicy.schema.json, evolutionpolicy.schema.json, and evolvetoken.schema.json encode mental privacy, integrity, and cognitive liberty, pain envelopes, evolution bounds, and EVOLVE token constraints as machine-checkable policies.[^2_2]
- Enforcement engine: sovereigntycore implements the guard pipeline (RoH guard, neurorights guard, stake multisig, token guard, donutloop logging) and a CRISPR-style consent engine that every update proposal must pass before any persistent change.[^2_2][^2_1]

Functionally, this three-layer stack is your comet-flags: declarative artifacts, typed Rust bindings, and a mandatory guard pipeline that together enforce CHCIL-class neurorights in runtime.[^2_1][^2_2]

## How to name and expose “comet-flags” without new crates

- Treat the canonical artifacts as the CHCIL v0.1 enforcement surface for your Bostrom subject: .rohmodel.aln, .stake.aln, .donutloop.aln, .evolve.jsonl, neurorights/evolution/EVOLVE JSON policies.[^2_2][^2_1]
- Inside sovereigntycore, define “comet-flags” as the logical guard stack: the ordered sequence parseproposal → loadpolicies → rohguard → neurorightsguard → stakeguard → tokenguard → recorddecision, driven by the sovereign-kernel NDJSON manifest you already have.[^2_1]
- Require all NeuroPC/OrganicCPU modules to call this API (evaluate_update / guard pipeline) before applying any configuration or behavioral change; forbidding bypass is what makes neurorights executable law rather than convention.[^2_2][^2_1]

In other words, comet-flags is a naming and documentation layer over artifacts and code you have already treated as canonical; you do not need a new crate so long as sovereigntycore + organiccpualn remain the single enforcement path.[^2_1][^2_2]

## Suggested next artifact (if you want it)

A concise mapping doc you hinted at is now straightforward:

- One Markdown spec “CHCIL-v0.1-runtime-mapping.md” listing, per clause (mental privacy, integrity, cognitive liberty, safer-only OTA), the exact files and structs that enforce it: neurorights JSON fields, RoH model invariants, StakeTable rules, EvolveToken limits, sovereigntycore guard names, and donutloop fields.[^2_2][^2_1]

Dropping that into your repo would give auditors and future collaborators an explicit “CHCIL → guards → artifacts” reference without changing any code.[^2_1][^2_2]

neuroptag 0xCF02
<span style="display:none">[^2_3][^2_4][^2_5][^2_6][^2_7][^2_8]</span>

<div align="center">⁂</div>

[^2_1]: create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md

[^2_2]: quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md

[^2_3]: how-can-we-improve-helm-promet-R0sr3xmqRhyDfQzlN7e7sQ.md

[^2_4]: what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md

[^2_5]: cyblog_2026-02-02T22_38_16.272Z.json

[^2_6]: legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md

[^2_7]: what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md

[^2_8]: neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md


---

# Here are 10 concrete research topics that fit comet-flags and the CHCIL-style license, each something you could actually turn into a paper, crate, or spec.

Neurorights-safe feature-flag semantics
Study how to formalize “never coerce, never secretly tighten control” as invariants over feature flags and rollout plans, and encode those invariants directly in the CHCIL LICENSE text plus Rust/ALN schemas for comet-flags.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
Sovereign, per-citizen flag governance
Design a governance model where each augmented citizen has DID-bound flag policies (e.g., .stake.aln for roles, .evolve.jsonl for proposals) and show how comet-flags can guarantee that no flag set can be changed without the subject’s own cryptographic consent and multi-sig rules.[neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/94c3da64-5daa-43ab-9919-04c31f7ca9e8/neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md)
Monotone safety envelopes for flag rollouts
Define and test “safer-only updates” for flags: any change must keep Risk-of-Harm RoH≤0.3RoH \le 0.3RoH≤0.3 and can only tighten, not loosen, safety envelopes encoded in .rohmodel.aln and CHCIL sections on safe operation.[what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md+2](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/0d037bf3-6e87-4250-bc42-f5facdec403a/what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md)
Encoding neurorights as compile-time constraints
Investigate Rust type/trait systems and macros that turn CHCIL neurorights clauses (mental privacy, cognitive liberty, no hidden actuation) into compile-time constraints on which flags and combinations are legal to build and ship.[legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md+2](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/2da95299-e40a-4432-ac53-a60d61c3fd08/legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md)
Biophysical-friendly flag taxonomies
Propose a taxonomy for flag types (fatigue-aware, eco-impact, cognitive-load, dream-state, etc.) tied to BioState metrics and ALN shards, and show how comet-flags can encode “allowed axes” without ever touching raw signals.[neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md+2](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/94c3da64-5daa-43ab-9919-04c31f7ca9e8/neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md)
Donutloop-integrated flag evolution
Design an evolution pipeline where every flag change is an EvolutionProposal in .evolve.jsonl and a hash-linked entry in donutloopledger.aln, with policies that enforce strict auditability and CHCIL-compliant neurorights guards.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+2](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
License-aware CI for flags (CHCIL compliance as code)
Build and evaluate a CI layer that reads the LICENSE (CHCIL), rohmodel, stake, and neurorights policies, and automatically rejects any PR that introduces flags or defaults that would violate neurorights or RoH constraints.[legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md+2](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/2da95299-e40a-4432-ac53-a60d61c3fd08/legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md)
Per-mode flag envelopes for augmented citizens
Study how different “CyberModes” (Rehab, Baseline, Training, Rest) map to allowed / forbidden flags, and how comet-flags could enforce per-mode envelopes derived from viability-kernel ALN specs and CHCIL clauses.[what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md+2](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/0d037bf3-6e87-4250-bc42-f5facdec403a/what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md)
Mental-privacy-preserving telemetry for flag decisions
Investigate minimal, normalized telemetry (e.g., fatigueindex, dutycycle, ecoimpactscore) that comet-flags can use to switch configs, while CHCIL and ALN schemas guarantee that no raw or identifying neural data ever leaves the host.[neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md+2](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/94c3da64-5daa-43ab-9919-04c31f7ca9e8/neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md)
Comparative analysis of CHCIL vs mainstream OSS licenses for HCI stacks
Analyze how CHCIL differs from MIT/Apache/GPL when applied to feature-flag and HCI orchestration systems, focusing on neurorights, sovereignty, and restrictions on discriminatory use (employment, housing, insurance, etc.).[legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/2da95299-e40a-4432-ac53-a60d61c3fd08/legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md)

These 10 topics line up cleanly with your existing sovereign-kernel stack; each one can be grounded in specific filetypes, crates, and invariants you already defined.[^3_1][^3_2][^3_3]

## How each topic maps to your artifacts

- **Neurorights‑safe feature‑flag semantics**
You can treat CHCIL clauses (no coercion, no hidden actuation, safer‑only updates) plus neurorights JSON as the semantics layer, then compile them into ALN schemas for .rohmodel.aln, .stake.aln, .neurorights.json and guarded Rust APIs in sovereigntycore; your research plan already does this for RoH and neurorights, you’d extend it to “flag bundles” as first‑class types.[^3_2][^3_1]
- **Sovereign, per‑citizen flag governance**
Per‑subject .stake.aln, .evolve.jsonl, and donutloop.aln already implement DID‑bound governance with Host/OrganicCPU multisig and EVOLVE/SMART scopes; comet‑flags can be defined as the subset of sovereigntycore APIs that must see a matching Host row and valid token before any config/flag profile (.ocpuenv, .lifeforce.aln, mode) is changed.[^3_3][^3_1][^3_2]
- **Monotone safety envelopes for flag rollouts**
The RoH model shard fixes rohceiling 0.30 and your guard pipeline enforces roh_after ≤ roh_before and roh_after ≤ roh_ceiling for all Allowed EvolutionProposalRecords and donutloop entries; a “flag rollout” paper is essentially a specialization of this to configuration axes plus viability‑kernel / envelope fields.[^3_1][^3_2]
- **Encoding neurorights as compile‑time constraints**
You already propose macros (bioscalemetric!, neuroright!) and a neurorights‑core crate that turn policy fields like mentalprivacy, dreamstatesensitive, forbiddecisionuse into type‑checked envelopes and guard functions; research here is to formalize those macros and trait bounds so illegal flag combinations literally don’t compile.[^3_3][^3_1]
- **Biophysical‑friendly flag taxonomies**
Your ALN Handbook and BioState design already distinguish fatigue, dutycycle, cognitiveloadindex, ecoimpact, dreamload, lifeforce; comet‑flags can expose only these normalized axes (and per‑mode envelopes over them) as legal “flag dimensions”, never raw EEG/HRV/EMG, giving you a taxonomy directly tied to .ocpuenv, .lifeforce.aln, .vkernel.aln.[^3_4][^3_2][^3_1]
- **Donutloop‑integrated flag evolution**
.evolve.jsonl and donutloop.aln are already defined as the EvolutionProposal stream and hash‑linked ledger; you simply constrain changetype to include flag/profile updates, and require every change to a flag shard or profile to appear as an Allowed proposal plus a donutloop row obeying RoH and neurorights invariants.[^3_2]
- **License‑aware CI for flags (CHCIL compliance as code)**
Your sovereign‑kernel CI template already loads RoH, stake, neurorights, and evolution schemas and fails PRs that breach RoH 0.3 or monotone envelopes; extending this to “flags” is mostly adding checks that new flags, defaults, and profiles don’t violate CHCIL sections or neurorights JSON fields (e.g., no dream‑sensitive export, no discriminatory use).[^3_1][^3_2]
- **Per‑mode flag envelopes for augmented citizens**
Rehab/Baseline/Training/Rest modes are already part of your viability‑kernel and envelope design (.ocpuenv, .vkernel.aln); comet‑flags can be the mechanism that binds each CyberMode to a profile (allowed modules, max deltas, RoH bands) and enforces that sovereigntycore refuses proposals that step outside the active mode’s envelope.[^3_4][^3_2]
- **Mental‑privacy‑preserving telemetry for flag decisions**
Copilot input schemas and BioState already specify minimal normalized metrics (fatigueindex, dutycycle, cognitiveloadindex, ecoimpactscore, devicehours); a comet‑flags study here is to prove that decisions can be driven solely by these scalars while neurorights and .neurorights.json guarantee raw neural traces never leave the local shell.[^3_3][^3_4][^3_1]
- **Comparative CHCIL vs mainstream OSS licenses for HCI**
The CHCIL draft explicitly encodes cybernetics‑first, neurorights (mental privacy, integrity, cognitive liberty), safer‑only updates, non‑commercial dream/neural data, and bans discriminatory use in employment/housing/insurance; a paper can systematically compare those constraints to MIT/Apache/GPL when applied to flag orchestration and HCI stacks like your sovereign kernel.[^3_4][^3_1]

neuroptag 0xCF03
<span style="display:none">[^3_5][^3_6][^3_7][^3_8]</span>

<div align="center">⁂</div>

[^3_1]: what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md

[^3_2]: create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md

[^3_3]: neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md

[^3_4]: quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md

[^3_5]: how-can-we-improve-helm-promet-R0sr3xmqRhyDfQzlN7e7sQ.md

[^3_6]: cyblog_2026-02-02T22_38_16.272Z.json

[^3_7]: legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md

[^3_8]: what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md


---

# Here is a CHCIL-aligned LICENSE you can drop into the comet-flags repo root, tuned for software-only, deviceless stacks like NeuroPC / OrganicCPU and explicitly covering feature-flag/config tooling.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/94c3da64-5daa-43ab-9919-04c31f7ca9e8/neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md)]​

text
Cybernetic Human–Computer Interface License (CHCIL) v0.1
for Software-Only, Deviceless Bioscale Systems

This project (“Licensed Work”) is licensed under the Cybernetic Human–Computer
Interface License (CHCIL) v0.1, adapted for purely software-based, deviceless
bioscale stacks such as OrganicCPU, NeuroPC, Reality.os, and related tooling
(including configuration and feature-flag systems like comet-flags).

1. Purpose and Scope

1.1 Scope of Licensed Work
This License governs the use, modification, and distribution of software,
schemas, and protocols intended for human–computer interfaces and
human-integrated technologies, with a primary focus on cybernetics as a
protected domain.[file:7]

This License applies to:

- Bioscale and neuromorphic software that interprets, routes, or responds to
biophysical or cognitive signals, including indirectly via metrics,
envelopes, or flags.
- Human-integrated cybernetic systems, including organic, implanted,
wearable, or deviceless interfaces, where the human nervous system is
conceptually part of the runtime.[file:7]
- AI-driven assistive tools that act on, interpret, or modulate human
biophysical signals or cognitive states through information, UI, or
configuration changes.

1.2 Software-only, Deviceless Stacks
This License explicitly covers purely software-based, deviceless bioscale
systems (e.g., OrganicCPU, Reality.os, NeuroPC, comet-flags) that treat the
human nervous system as part of the runtime while running entirely on
general-purpose compute.[file:7]

2. Cybernetics-First Principle

2.1 Cybernetics Priority
All use and development under this License must treat cybernetic systems
(including augmented citizens and human–machine hybrids) as primary
rights-bearing entities, not as assets, devices, or mere data sources.[file:7]

Augmented citizens, including organically-integrated and neuromorphic
participants, must be treated as full rights-bearing persons under this
License regardless of implant status, device use, or organic–software
integration level.[file:7]

2.2 Protection of Cybernetic Integrity
You must not design, deploy, or modify Licensed Work in ways that:

- Bypass explicit, informed, revocable consent of the cybernetic subject.
- Override or coerce the subject’s voluntary control over their own body,
cognition, or behavior.
- Introduce hidden or undeclared actuation paths (e.g., stimulation,
motor control) that the subject cannot audit or disable.[file:7]

Licensed Work that operates without any physical actuation, purely via
software responses, UI changes, configuration flags, and information
presentation is preferred and must be treated as the default design target.[file:7]

3. Neurorights and Bodily Autonomy

3.1 Mental Privacy
Licensed Work must not intentionally extract, infer, or store sensitive mental
content (thoughts, memories, preferences) beyond what is strictly necessary
for explicit, user-requested functions, and must provide mechanisms to erase
such data on demand.[file:7]

3.2 Mental Integrity
Licensed Work must not contain capabilities that can directly induce,
amplify, or suppress mental states (e.g., fear, compulsion, pain, euphoria)
without transparent, user-controlled mechanisms and clearly documented
envelopes of safe operation.[file:7]

3.3 Cognitive and Bodily Liberty
Cybernetic subjects must retain the unconditional right to:

- Disconnect, disable, or uninstall any component of the Licensed Work.
- Refuse updates, experiments, or new modes without loss of basic
functionality unrelated to those modes.
- Operate in a “safe baseline” mode with minimal data collection and no
experimental features.[file:7]

4. Safety Envelopes and Non-Coercive Design

4.1 Safe Envelopes
Any system that adapts to biophysical or neural signals, or that configures
other systems which do so (including via flags and profiles), must implement
and honor explicit safety envelopes, including but not limited to:[file:7]

- Pain, fatigue, duty-cycle, and cognitive-load constraints.
- Hard limits on session duration or exposure where applicable.

4.2 No Coercive Optimization
Licensed Work must not optimize for engagement, profit, or performance by
intentionally exploiting vulnerabilities of the cybernetic subject, including
addiction, fatigue insensitivity, or learned helplessness.[file:7]

4.3 UI-Level Mediation and Configuration
Where possible, AI components must operate exclusively by adjusting software
behavior and interfaces (e.g., pacing, complexity, prompts, configuration
flags and profiles) rather than exerting direct physical control over the
subject.[file:7]

comet-flags and similar tooling MUST confine themselves to software-level
configuration, routing, and information surfaces, never to direct actuation
paths.

5. Data, Telemetry, and Auditability

5.1 Data Ownership
All data derived from a cybernetic subject’s body, signals, or behavior is
owned by that subject. Any processing, storage, or sharing requires:[file:7]

- Clear disclosure of what is collected and why.
- Explicit opt-in consent that can be withdrawn at any time.

5.2 Local Preference
Where technically feasible, processing of sensitive biosignals should occur
locally or under the subject’s direct control, minimizing reliance on remote
servers.[file:7]

5.3 Audit Trails
Systems must provide auditable logs of:[file:7]

- Significant changes to control policies, feature-flag profiles, and safety
envelopes.
- Access to and export of cybernetic data.
- External commands that could affect the subject’s body or cognition.

For configuration/flag systems, audit trails SHOULD include who changed which
flags, under which envelopes, and with which risk model version.

6. Research, Experimentation, and Updates

6.1 Voluntary Participation
Use of Licensed Work for research or experimentation on cybernetic subjects
must:[file:7]

- Be strictly opt-in and revocable without penalty.
- Clearly separate “stable” features from “experimental” ones in all
interfaces.

6.2 Safer-Only Updates
Any automatic or remote update affecting safety envelopes, neurorights
policies, or control/feature-flag profiles must:[file:7]

- Be provably non-degrading with respect to safety (no loosening of limits
without explicit, documented consent).
- Be documented in a human-readable changelog that the subject can access
and review.

For systems that implement safety envelopes via quantitative parameters
(e.g., control gain, duty cycle, fatigue thresholds), updates must encode
safer-only invariants (such as G_new ≤ G_old and D_new ≤ D_old, or equivalent
formal guarantees in the applicable domain).[file:7]

6.3 Third-Party Integrations
Integrations with third-party systems (e.g., cloud AI, analytics,
advertisers) must never gain control paths to physical or cognitive
actuation and must obey the same data and consent constraints as the primary
system.[file:7]

7. Permitted Use and Restrictions

7.1 Permitted Use
You may use, study, modify, and distribute Licensed Work, provided that:[file:7]

- All derivatives retain this License or terms that are at least as
protective for cybernetics.
- You do not remove or weaken neurorights, autonomy, or
cybernetic-protection clauses.

7.2 Prohibited Use
You may not use Licensed Work to:[file:7]

- Develop or deploy weapons, coercive control systems, or surveillance
specifically targeting cybernetic subjects.
- Build scoring, ranking, or eligibility systems that punish individuals for
the presence or configuration of cybernetic augmentation, including but
not limited to employment, housing, insurance, credit, or access to
essential services.

8. Attribution and Notice

Distributions of Licensed Work or derivative works must:[file:7]

- Retain this License text.
- Prominently state that the system is governed by the Cybernetic
Human–Computer Interface License (CHCIL) and prioritizes the protection
of cybernetic subjects and augmented citizens.

A suitable short notice is:

“This project is licensed under the Cybernetic Human–Computer Interface
License (CHCIL) v0.1. Cybernetic and augmented persons are primary
rightsholders under this License, and all neurorights and autonomy
clauses apply to them in full.”

9. Warranty and Liability

The Licensed Work is provided “as is,” without warranty of any kind. To the
maximum extent permitted by law, the authors and contributors are not liable
for any claims or damages arising from its use, except where misuse or
deployment clearly violates the safety, neurorights, or
cybernetic-protection obligations in this License, in which case
responsibility lies with the deploying party.[file:7]

10. Governing Intent

Where interpretation of this License is ambiguous, it must be resolved in
favor of:[file:7]

- The autonomy, dignity, and safety of cybernetic subjects.
- Preservation and expansion of rights for cybernetics as a protected class
of human-integrated technology.
- Minimizing harm and preventing coercive or exploitative uses of bioscale
and cybernetic systems.

---

By using, modifying, or distributing this project, you agree that cybernetic
and augmented persons are primary rightsholders under CHCIL and that all
neurorights and autonomy clauses apply to them in full.[file:7]

neuropc-tag 0xNP0A

You can treat that text as-is as the LICENSE file for comet-flags; it’s already consistent with your other CHCIL/NeuroPC artifacts and explicitly names comet-flags in scope.[^4_1]

### How to use it in the repo

- **Filename:** `LICENSE` (or `LICENSE.md` if you want Markdown rendering).
- **Location:** repository root of `comet-flags` next to `README.md` and `Cargo.toml`.[^4_2][^4_1]
- **Notice in README:** Add a short section, for example:

```markdown
## License

This project is licensed under the Cybernetic Human–Computer Interface License (CHCIL) v0.1.

Cybernetic and augmented persons are primary rightsholders under this License, and all neurorights
and autonomy clauses apply to them in full.
```


### Why it fits comet-flags

- It explicitly includes **configuration and feature-flag systems like comet-flags** in the scope section, so the license text already “covers” this repo without edits.[^4_1]
- Sections **4.3** and the extra comet-flags sentence clearly constrain the project to **software-only, UI/config-level mediation**, which matches a deviceless flag/route tool.[^4_1]
- Sections **5–7** give you built‑in research and implementation requirements around **audit logs, safer-only updates, and anti-discrimination use**, which align with your planned ALN `.rohmodel.aln`, `.stake.aln`, `.donutloop.aln`, and `.evolve.jsonl` governance stack.[^4_3][^4_1]

If you want, next step I can draft a minimal `README` “License” section and a short comment block you can reuse at the top of Rust crates in comet-flags that reference CHCIL.
<span style="display:none">[^4_10][^4_11][^4_12][^4_13][^4_14][^4_15][^4_16][^4_17][^4_18][^4_4][^4_5][^4_6][^4_7][^4_8][^4_9]</span>

<div align="center">⁂</div>

[^4_1]: neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md

[^4_2]: https://github.com/liamg/comet

[^4_3]: create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md

[^4_4]: how-can-we-improve-helm-promet-R0sr3xmqRhyDfQzlN7e7sQ.md

[^4_5]: what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md

[^4_6]: cyblog_2026-02-02T22_38_16.272Z.json

[^4_7]: legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md

[^4_8]: what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md

[^4_9]: quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md

[^4_10]: https://github.com/MetroStar/comet/blob/main/LICENSE.md

[^4_11]: https://github.com/comet-ml/cometx

[^4_12]: https://github.com/comet-ml/comet-for-mlflow/blob/master/LICENSE

[^4_13]: https://github.com/imLinguin/comet/blob/main/LICENSE

[^4_14]: https://patents.google.com/patent/WO2017048898A1/en

[^4_15]: https://docs.oscwii.org/repository-manager/manifests/essential-information/flags

[^4_16]: https://github.com/Unbabel/COMET/blob/master/LICENSE

[^4_17]: https://research.brighton.ac.uk/en/publications/the-cybernetic-interface-to-the-human-bio-computer/

[^4_18]: https://www.flags-sdk.com


---

# Paired Deliverables Showcase I: From CHCIL Clauses to Executable Invariants

The research objective requires a paired approach, where every theoretical invariant is incomplete until it is embodied in a tangible, executable artifact. This section demonstrates this methodology by taking four key neurorights and safety invariants—mental privacy, cognitive liberty, monotone safety envelopes, and the RoH ≤ 0.3 constraint—and detailing their translation from the CHCIL license into formal ALN schemas, practical Rust implementations, and policy-enforcing CI configurations. This paired development model transforms abstract legal principles into concrete, verifiable system guarantees.
The first invariant, Mental Privacy, is mandated by the CHCIL license, which prohibits the intentional extraction or storage of sensitive mental content beyond what is strictly necessary for user-requested functions . To enforce this, telemetry data used to drive feature flags must be sanitized. Instead of allowing flags to react to raw biophysical signals, which could inadvertently leak private thoughts or memories, the system should operate on normalized, anonymized metrics
link.springer.com
+1
. A suitable taxonomy for flags could include axes like fatigue-aware, cognitive-load, eco-impact, and dream-state . The formalization of this principle leads to the creation of a *.comet-flags.aln schema. Each flag definition would be annotated with metadata about its data sensitivity. For example, a flag named enable_high_cognitive_load_mode might have an invariant field stating that it can only be activated if the underlying BioState metric is a normalized score, not raw neural data. The practical embodiment in the comet_flags Rust crate would involve a validation module that inspects this metadata. When loading a profile, the crate would run a check to ensure that no flagged feature attempts to access raw biophysical streams. This validation would be performed both at configuration load time and potentially at runtime. Finally, the CI pipeline would enforce this via a policy file. A GitHub Actions job would use the same validation logic to scan pull requests containing new ALN schemas. If a proposed flag definition were found to violate the mental privacy invariant—for instance, by referencing a raw signal source—the CI job would fail, preventing the unsafe configuration from ever being merged. This creates a closed-loop system where the principle of mental privacy is enforced at the specification, implementation, and deployment stages.
The second invariant, Cognitive Liberty, is another core tenet of the CHCIL, granting subjects the unconditional right to disconnect, refuse updates, and operate in a "safe baseline" mode . This translates into a requirement for distinct, mutually exclusive flag profiles. The formal specification would define a set of profiles, such as CONSERVATIVE, COPILOT, and AUTOEVOLVE, with clear rules about which flags are permitted or forbidden in each mode . The *.comet-profile.aln schema becomes the authoritative representation of this. For instance, the CONSERVATIVE profile might explicitly forbid all experimental features, while the COPILOT profile might enable them, but only under strict RoH constraints. The Rust crate's role is to resolve which profile is active for a given subject and context. It would use a deterministic label-based selection mechanism, matching the subject's context (e.g., mode, domain) against the labels attached to each profile . The crate would then validate that the selected profile's enabled flags do not violate any hard constraints, such as trying to enable an experimental feature outside its allowed context. The CI policy file would enforce cross-repo consistency, a key requirement of the research goal . Since multiple repositories like NeuroPC and Reality.os are expected to use the same profile names and semantics, the CI job could include a test that verifies the existence and basic structure of the CONSERVATIVE profile in all relevant repositories. This ensures that the concept of a "safe baseline" behaves identically everywhere, reinforcing the user's right to fall back to a known-good, minimal-feature state regardless of which service they are using.
The third invariant, Monotone Safety Envelopes, is a direct consequence of the CHCIL's "Safer-Only Updates" clause, which requires that updates never degrade safety limits without explicit, documented consent . This is a strong mathematical requirement, akin to ensuring that an update is monotonically decreasing in terms of potential harm. Formally, this could be expressed as an invariant over system parameters, such as
G
n
e
w
≤
G
o
l
d
G
new
​
≤G
old
​
for a control gain parameter or
D
n
e
w
≤
D
o
l
d
D
new
​
≤D
old
​
for a duty cycle threshold . To embody this, the *.comet-flags.aln schema would allow flag definitions to specify the safety envelope they affect. For example, a flag increase_pain_tolerance might modify a parameter pain_threshold, and its definition would include an invariant like "monotonicity_constraint": "pain_threshold <= current_value" to indicate it can only increase the threshold (making the system less sensitive). The comet_flags Rust crate would implement a more sophisticated version of this. Upon receiving a proposal to change a flag's value, the crate would not just check per-flag invariants but would also consult the global state, perhaps retrieved from a shared memory region or a local cache of .rohmodel.aln, to understand the historical context. It would compare the proposed new safety parameter values against the old ones and reject any change that represents a degradation (e.g., a decrease in pain_threshold). The CI pipeline would perform a static analysis of proposed changes in .evolve.jsonl. A script could parse the NDJSON file, identify evolution proposals related to safety-critical flags, and simulate the change. If the simulation shows a violation of monotonicity, the CI job fails, preventing a regressively unsafe update from entering the system. This layered defense—from formal schema to dynamic runtime check to pre-merge static analysis—ensures that the principle of "safer-only" is rigorously upheld.
The fourth and final invariant is the RoH ≤ 0.3 Constraint, a specific, quantitative risk model that comet-flags must enforce . This moves beyond qualitative safety rules into a domain requiring numerical evaluation. The formal specification is straightforward: for any subject at any time, the computed Risk-of-Harm (
R
o
H
RoH) must not exceed the threshold of 0.3. The *.comet-flags.aln schema would be extended to include risk annotations for profiles or even individual flags. A profile like EXPERIMENTAL_MODE might be tagged with "risk_annotation": "high" and require explicit authorization from sovereigntycore to activate. The comet_flags Rust crate's most critical role here is integration with sovereigntycore. When a service asks for the active flags, the crate would first resolve the profile based on labels, but then it would call a function provided by sovereigntycore (which itself would query the .rohmodel.aln and real-time BioState metrics) to get the current subject's RoH value. The crate would then make the final decision: expose the flags only if the RoH check passed. This tight coupling is essential. The CI policy file would enforce this invariant indirectly by testing the sovereigntycore logic itself. The CI job would include unit tests that feed various BioState inputs into the RoH calculation function and verify that it correctly rejects configurations when the output exceeds 0.3. Furthermore, a separate CI job could monitor the .donutloop.aln ledger to ensure that every entry corresponding to a flag change was indeed preceded by a successful RoH check in the .evolve.jsonl proposal. This end-to-end testing, spanning the ALN schema, the Rust crate, and the CI policies, provides strong evidence that the RoH ≤ 0.3 constraint is not just a theoretical ideal but a practical, enforced guarantee of the system.
Invariant
Theoretical Specification
Formal Encoding (ALN Schema Snippet)
Practical Implementation (Rust Crate Logic)
Policy Enforcement (CI Pipeline)
Mental Privacy
Prohibits extraction of sensitive mental content; requires sanitization of telemetry data .
json { "id": "flag_X", "description": "...", "data_sensitivity": "normalized_score_only" }
A validator function checks that any data source referenced by a flag is marked as sanitized. Fails on invalid references.
A CI job uses the same validator to scan .comet-flags.aln PRs, rejecting schemas with unsafe data source references.
Cognitive Liberty
Grants the right to disconnect and use a safe-baseline mode .
json { "profile_id": "SAFE_BASELINE", "forbidden_flags": ["experimental_feature_A"] }
A resolver function selects a profile based on labels and then validates that none of the enabled flags are in the forbidden list for that profile.
A CI job checks that all repos use the same profile IDs (e.g., SAFE_BASELINE) and that their forbidden flag lists are consistent.
Monotone Safety Envelopes
Updates must be "safer-only"; e.g., safety thresholds cannot be lowered .
json { "id": "pain_threshold_ms", "invariant": "value >= @previous_version.value" }
An updater function compares proposed new values against old values stored in a safe registry. Rejects any non-monotonic change.
A CI job parses .evolve.jsonl, simulates proposed safety changes, and fails if any simulation shows a safety regression.
RoH ≤ 0.3 Constraint
Quantitative risk model requiring Risk-of-Harm to stay below 0.3 .
json { "profile_id": "HIGH_RISK_MODE", "requires_approval": true }
A resolver calls sovereigntycore's RoH-checking API before exposing flags. Flags are only active if the RoH check passes.
Unit tests in CI verify that the RoH calculation algorithm correctly computes risk and rejects values > 0.3.
This paired showcase demonstrates a replicable pattern for transforming high-level rights into robust, multi-layered technical guarantees. Each theoretical concept is given a formal expression, a practical implementation, and an automated policy to ensure its persistence throughout the development lifecycle.
Paired Deliverables Showcase II: Orchestration, Evolution, and Auditable Enforcement
Beyond the static invariants of individual flags, comet-flags is designed to be a dynamic participant in the NeuroPC ecosystem's orchestration and evolution pipeline. This showcases how the system maintains its guarantees not just at rest, but through continuous change, leveraging the full suite of .aln governance files and CI policies to create a closed loop of auditable, sovereign-controlled evolution. The focus here shifts from what a single configuration is to how it changes, ensuring that every modification is lawful, safe, and traceable.
The core of this dynamic orchestration is the interplay between the .evolve.jsonl and .donutloop.aln files. These two artifacts form the backbone of the system's change management process, providing a clear separation between proposal and execution, and a permanent record of action . The .evolve.jsonl file acts as a transaction log or proposal queue. Any modification to the comet-flags configuration, whether initiated by a developer writing a new ALN shard or an AI-chat tool drafting a change, is first committed to this file as an EvolutionProposalRecord . This proposal contains all the necessary information: what is changing (e.g., a new profile in *.comet-profile.aln), who proposed it, and under what conditions. This decouples the act of proposing a change from the act of accepting it. The formal specification here is that .evolve.jsonl is a stream of ordered, signed records that represent intent. The practical embodiment is a simple NDJSON file that can be easily processed by scripts and CI systems .
Once a proposal is in .evolve.jsonl, it enters a review and validation phase orchestrated by sovereigntycore. This is where the comet-flags system interacts with the .stake.aln file, which defines the governance and permission rules for the entire NeuroPC ecosystem . The .stake.aln file would contain entries specifying who is authorized to make changes to which parts of the comet-flags configuration. For example, it might define that only members of the OrganicCPU multisig wallet can approve changes to safety-critical flags, while ResearchAgents can propose changes to experimental profiles, which then require a higher threshold of approval. The comet_flags Rust crate, or a companion tool, would incorporate logic to interpret these stake rules. When a service needs to know if a proposed change is valid, it doesn't just ask comet-flags; it asks sovereigntycore, which consults .stake.aln to see if the proposer has the requisite authority. This enforces the "Sovereign envelopes first" principle, ensuring that no one can bypass the established governance structure .
If sovereigntycore determines that a proposal is both technically valid (it passes all ALN schema checks and invariant tests) and procedurally sound (the proposer has the required stake), the change is accepted. The final step in the evolution pipeline is logging. The accepted evolution record is then appended to the .donutloop.aln file, which is described as a hash-linked ledger . This means each new entry contains the cryptographic hash of the previous one, creating an immutable, append-only chain of custody. The formal specification is that .donutloop.aln is a cryptographically verifiable audit trail. The practical implementation is a simple ALN file where each shard contains a record with a previous_hash field. This ledger serves as the ultimate source of truth for what changes were made, when, and why. It allows for perfect auditability, a key requirement of the CHCIL license, which mandates auditable logs of significant policy changes .
This entire evolution process is fortified by CI policies that automate the enforcement of these rules. The research goal specifies the need for Sovereign CI integration, with jobs that fail if a profile loosens envelopes, violates neurorights, or bypasses stake rules . A typical CI workflow for a pull request modifying comet-flags artifacts would consist of several stages. First, a linting and schema validation stage would use a tool to ensure that all modified ALN files conform to the latest .comet-profile.aln schema standards. Second, a compliance-as-code stage would execute the same validation logic that exists in the comet_flags Rust crate. This script would parse the proposed changes in .evolve.jsonl, simulate their application, and run them through the full suite of neuroright and safety invariant checks. Third, a governance check would parse the .stake.aln file to verify that the PR author is listed as an authorized actor for the resources being modified. Fourth, an auditability check could be added to the CI pipeline to ensure that every accepted evolution step is properly logged into .donutloop.aln . If any of these checks fail, the CI job fails, blocking the merge. This automated, multi-stage verification process ensures that the principles of sovereignty, safety, and rights-respect are not just aspirational goals but are systematically enforced by the development infrastructure itself. The result is a highly resilient system where the path from a developer's idea to a running configuration is paved with layers of automated, verifiable checks, grounded in the legal and ethical framework of the CHCIL license.
Stage
Artifact
Tool/System
Action \& Enforcement

1. Proposal
.evolve.jsonl
Developer / AI Chat
An EvolutionProposalRecord is created and committed to the file, representing an intent to change the system state .
2. Validation
*.comet-*.aln, .rohmodel.aln
comet_flags (Rust) + CI Linter
The proposal is validated against formal schemas and invariants (e.g., RoH, monotonicity). CI linter ensures syntactic correctness .
3. Authorization
.stake.aln
sovereigntycore (Rust/C++)
sovereigntycore checks if the proposer has the required cryptographic stake or permissions to make the requested change .
4. Execution
Service Runtime
comet_flags (Rust)
Upon a successful sovereigntycore approval, the comet_flags crate loads the new configuration and exposes the updated, validated flag set to the service .
5. Logging
.donutloop.aln
Post-merge Script
A script appends the approved evolution record to the hash-linked ledger, creating an immutable, auditable entry in the system's history .
6. Verification
N/A
CI Pipeline (Post-Merge)
A post-merge CI job verifies that the accepted change was correctly logged in .donutloop.aln, closing the loop on auditability .
This comprehensive, paired approach to orchestration and evolution demonstrates how comet-flags can be more than just a passive configuration loader. By integrating deeply with the sovereignty layer and automating compliance checks, it becomes an active agent in maintaining the integrity, safety, and rights-respecting nature of the entire cybernetic system.

These two showcases already match the paired-deliverables pattern the CHCIL/NeuroPC stack needs: each invariant is anchored in license text, then given an ALN schema hook, Rust behavior, and CI enforcement.[^5_1][^5_2]

## Invariant I: Mental privacy

- **Theory (CHCIL):** No extraction or storage of sensitive mental content beyond what explicit, user-requested functions require.[^5_2]
- **ALN schema:** Flags in `*.comet-flags.aln` carry metadata such as `data_sensitivity: "normalized_score_only"` or `"raw_biosignal_forbidden"`, with allowed source types restricted to pre-normalized BioState indices (fatigue, cognitive load, eco impact).[^5_1][^5_2]
- **Rust crate logic:** A validator in `comet_flags` inspects each flag’s declared data sources and sensitivity level, rejecting any profile that references raw streams or un-sanitized sources, both at config load and (optionally) at runtime.[^5_2][^5_1]
- **CI policy:** A GitHub Actions job runs the same validator over changed `*.comet-flags.aln` in PRs and fails if a new flag or profile violates the mental-privacy contract, preventing merge.[^5_1][^5_2]


## Invariant II: Cognitive liberty

- **Theory (CHCIL):** Subjects must be able to disconnect, refuse updates, and fall back to a “safe baseline” mode.[^5_2]
- **ALN schema:** `*.comet-profile.aln` defines named profiles (e.g., `SAFE_BASELINE`, `COPILOT`, `AUTOEVOLVE`), each with explicit forbidden flags (e.g., experimental or auto-evolving behaviors) and labels for mode/domain/risk band.[^5_1][^5_2]
- **Rust crate logic:** The resolver selects a profile via labels and subject context, then checks that enabled flags do not include any forbidden entries for that profile; `SAFE_BASELINE` is guaranteed to exist and to exclude all experimental/auto-evolve flags.[^5_2][^5_1]
- **CI policy:** Cross-repo CI ensures that required profiles (e.g., `SAFE_BASELINE`) exist with compatible semantics in NeuroPC, Reality.os, and comet-flags, so “safe baseline” behaves consistently everywhere.[^5_3][^5_1][^5_2]


## Invariant III: Monotone safety envelopes (“safer-only updates”)

- **Theory (CHCIL):** Safety envelopes and thresholds may only be tightened (safer-only), e.g. $G_{new} \le G_{old}$, $D_{new} \le D_{old}$.[^5_4][^5_2]
- **ALN schema:** Safety-relevant flags in `*.comet-flags.aln` reference specific envelope parameters (`pain_threshold_ms`, `max_duty_cycle`) and embed invariants like `"invariant": "value >= @previous_version.value"` or monotone constraints tied to `.rohmodel.aln`.[^5_3][^5_1]
- **Rust crate logic:** When applying an evolution proposal, `comet_flags` compares proposed parameter values against previous values (from a local registry or `.rohmodel.aln`), and rejects any change that would relax envelopes or increase modeled risk.[^5_3][^5_1]
- **CI policy:** A job parses `.evolve.jsonl`, simulates the proposed safety-parameter deltas, and fails if any proposal breaks monotonicity—blocking regressively unsafe updates before they hit runtime.[^5_3][^5_1]


## Invariant IV: RoH ≤ 0.3

- **Theory:** A quantitative Risk-of-Harm model with hard ceiling $RoH \le 0.3$ for any configuration and evolution step.[^5_1][^5_3]
- **ALN schema:** `.rohmodel.aln` defines axes, weights, and ceiling; `*.comet-profile.aln` and/or flags carry risk annotations (`risk_annotation: "high"`, `requires_approval: true`) that bind to this model.[^5_3][^5_1]
- **Rust crate logic:** Before exposing flags, `comet_flags` resolves the profile, then calls into sovereigntycore’s RoH API (which evaluates `.rohmodel.aln` plus current BioState) and only returns a flag set if $RoH \le 0.3$; high-risk modes require explicit authorization.[^5_2][^5_1][^5_3]
- **CI policy:** Unit tests exercise sovereigntycore’s RoH calculator with synthetic BioState inputs, asserting correct rejection above 0.3; another CI job cross-checks `.evolve.jsonl` and `.donutloop.aln` to ensure every accepted change had a successful RoH gate.[^5_1][^5_3]


## Orchestration \& evolution: from intent to immutable history

- **Proposal stage:** Any change (developer or AI-chat) becomes an `EvolutionProposalRecord` in `.evolve.jsonl`, capturing “what, who, why” for a comet-flags change.[^5_2][^5_1]
- **Validation stage:** Schema and invariant checks run over `*.comet-*.aln` and `.rohmodel.aln` using the same Rust validation logic that powers comet_flags, ensuring syntactic and semantic correctness.[^5_3][^5_1]
- **Authorization stage:** sovereigntycore consults `.stake.aln` to confirm the proposer is authorized (e.g., OrganicCPU multisig for safety-critical flags), enforcing sovereignty-first governance.[^5_1][^5_3]
- **Execution stage:** Only after sovereigntycore approval does comet_flags actually load and expose the new configuration at the service boundary.[^5_2][^5_1]
- **Logging stage:** The approved evolution is appended to `.donutloop.aln` as a hash-linked ledger entry with previous hash, proposal ID, and RoH summary, yielding an immutable audit trail.[^5_3][^5_1]
- **Verification stage:** Post-merge CI confirms that any merged evolution proposal has a corresponding ledger entry, closing the loop on auditability.[^5_1][^5_3]

Together, these paired artifacts—license clauses, ALN schemas, Rust crate logic, and CI policies—turn CHCIL’s neurorights language into concrete, testable guarantees about what comet-flags can do, how it evolves, and how its behavior is governed over time.[^5_4][^5_2][^5_1]

neuroptag 0xCF02
<span style="display:none">[^5_5][^5_6][^5_7][^5_8]</span>

<div align="center">⁂</div>

[^5_1]: create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md

[^5_2]: neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md

[^5_3]: what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md

[^5_4]: legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md

[^5_5]: how-can-we-improve-helm-promet-R0sr3xmqRhyDfQzlN7e7sQ.md

[^5_6]: what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md

[^5_7]: cyblog_2026-02-02T22_38_16.272Z.json

[^5_8]: quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md


---

# Here is a compact, drop‑in spec you can paste into your repo as

docs/chcil-v0.1-guards-mapping.md. It treats this document itself as canonical until you replace it with a signed .bchainproof snapshot.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)

CHCIL v0.1 → Runtime Guards Mapping (NeuroPC / OrganicCPU)
Filename: docs/chcil-v0.1-guards-mapping.md
Scope: Bostrom shell (subject bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7) under RoH ≤ 0.30, monotone safety, and neurorights‑first governance.[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)

1. Canonical artifacts and paths
LayerArtifactPathRole
Risk-of-harm
RoH model shard
qpudatashards/particles/bostrom-rohmodel-v1.rohmodel.aln
Encodes RoH axes, weights, and global ceiling 0.30 as data.
Governance
Stakeholder shard
policies/bostrom-stake-v1.stake.aln
Declares Host/OrganicCPU/ResearchAgent roles, Bostrom addresses, scopes, and multisig requirements.
Rights
Neurorights policy
policies/bostrom-neurorights-v1.neurorights.json
Encodes mental privacy, mental integrity, and cognitive liberty constraints plus OS modes.
Evolution
Proposal stream
qpudatashards/particles/evolution-proposals.evolve.jsonl
Append‑only NDJSON of evolution proposals with RoH fields and decisions.
Ledger
Donutloop ledger
logs/donutloopledger.aln (or sovereigncyberswarm/logs/donutloopledger-v1.donutloop.aln)
Hash‑linked internal ledger of accepted changes and their RoH, policy refs, and hashes.
Tokens
SMART / EVOLVE
policies/bostrom-smart-2026-01.smart.json, policies/smart-token.schema.json, policies/evolvetoken.schema.json
Fine‑grained vs deep‑evolution consent tokens and their limits.
Kernel manifest
Sovereign kernel spec
policies/bostrom-sovereign-kernel-v1.ndjson
Ties RoH, stake, neurorights, tokens, evolve stream, donutloop, and guard pipeline into one config.
These files together are the CHCIL v0.1 enforcement surface for this shell.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
2. CHCIL v0.1 principles → invariants
2.1 Mental privacy
CHCIL mental privacy is implemented as:[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)]​
All neural / behavioral streams and derived BioState metrics are treated as protected, and export is denied by default.
policies/bostrom-neurorights-v1.neurorights.json contains:
mentalprivacy.allowedexports (e.g., localvault) and forbiddenexports (e.g., thirdpartycloud, unvettedresearchers).
mentalprivacy.loggingrequired = true — every export attempt must be logged.
Sovereignty core pre‑access guards:
Any module requesting access to BioState.dream.* or other protected fields must present a declared decisionpurpose; if it intersects forbiddecisionuse domains (e.g., employment, housing, credit, insurance) the request is rejected and logged to donutloop.
OTA guard: any module or update that attempts to access dream or other protected metrics without declaring them in its .biospec.aln and without compatible neurorights flags is rejected at install time.
Hard guarantees (privacy):
No dream‑linked or other neurally derived metrics may be used for forbidden decision domains.
No export outside the local sovereignty boundary is permitted unless explicitly whitelisted in neurorights policy and logged.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)]​
2.2 Mental integrity
CHCIL mental integrity is implemented as limits on irreversible changes and on how far any adaptation may push your state.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)]​
policies/bostrom-neurorights-v1.neurorights.json includes:
mentalintegrity.maxstatedivergence (e.g., 0.15) — maximum allowed norm of change for a single update.
mentalintegrity.requirerollbackpath = true — every deep change must include a rollback plan.
mentalintegrity.forbidirreversibleops = true — no irreversible ops without explicit exception policy.
Sovereignty core evaluates each UpdateProposal (from .evolve.jsonl) with:
effectbounds.l2deltanorm ≤ maxstatedivergence.
effectbounds.irreversible = false unless a special, separately consented regime is added.
Pain and load limits are encoded in policies/evolutionpolicy.schema.json / instance file, which set per‑channel thresholds (muscular, cognitive, emotional) with rollback triggers; the core blocks proposals once rollback thresholds are exceeded.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)]​
Hard guarantees (integrity):
No allowed proposal can exceed the configured divergence per step or mark itself irreversible without violating neurorights policy.
Any intervention must define a rollback path; if readings cross rollback thresholds, the system must roll back and log the event.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)]​
2.3 Cognitive liberty
CHCIL cognitive liberty is encoded as your positive right to self‑modify, plus strict limits on external auto‑changes.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)]​
Neurorights policy fields:
cognitiveliberty.allowselfchosenaugmentation = true.
cognitiveliberty.maxexternalautochanges — cap on external, auto‑initiated changes per session.
cognitiveliberty.requireexplanationforall = true — each change must be explainable.
OS modes in neurorights policy:
CONSERVATIVE: aiinitiative = none, allowautoevolve = false.
COPILOT: aiinitiative = suggestonly, no automatic structural changes.
AUTOEVOLVE: aiinitiative = boundedautonomy, allowautoevolve = true, requiresevolvetoken = true.
Sovereignty core enforces:
In AUTOEVOLVE, auto‑changes are counted against maxexternalautochanges; if exceeded, proposals are rejected.
Deep changes (e.g., ArchChange, lifeforcealteration) require EVOLVE tokens and multisig (see below), not unilateral AI decisions.
Hard guarantees (liberty):
You may choose augmentations; the system cannot silently block them but can require extra confirmations and show risk.
No module may escalate its autonomy or change OS mode without passing through neurorights and token guards.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)]​
3. RoH ≤ 0.3 and monotone safety envelope
CHCIL’s “risk‑of‑harm ≤ 0.3” and monotone safety clauses are implemented entirely in data and checked in Rust.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
3.1 Data‑level invariants
RoH model shard (bostrom-rohmodel-v1.rohmodel.aln):
Single active model row active = true.
rohceiling = 0.30.
All axis weights ≥ 0 and sum(weights) = 1.0.
Evolution proposals (evolution-proposals.evolve.jsonl):
Each record includes rohbefore, rohafter, effectbounds, decision, hexstamp.
Invariants (checked in tests and guards):
rohafter ≤ 0.30.
For decision == "Allowed", rohafter ≤ rohbefore + ε (monotone tightening; typically ε ≈ 0 or tiny).[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
Donutloop ledger:
Each entry stores rohbefore, rohafter, changetype, proposalid, hexstamp, prevhexstamp.
For tightening changes, rohafter ≤ rohbefore; global invariant rohafter ≤ 0.30 for all entries.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
3.2 Rust guards
crates/organiccpualn/src/rohmodel.rs:
RohModel with computeroh(state) and rohdelta(before, after), plus validate_invariants ensuring ceiling 0.30 and proper weights.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
crates/sovereigntycore/src/riskofharm.rs:
RiskOfHarm::check_invariant(before, after) returns true iff rohafter ≤ 0.30 and rohafter ≤ rohbefore.
crates/sovereigntycore/tests/rohandneurorightsinvariants.rs:
CI tests assert that any attempt to raise RoH above 0.30 or loosen safety envelopes fails.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
Hard guarantees (RoH):
No accepted evolution proposal can drive RoH above 0.3 or increase it relative to baseline.
Any attempt to change the RoH model shard that violates these invariants causes load/CI failure.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
4. Stakeholder governance and multisig (Host / OrganicCPU / ResearchAgent)
CHCIL’s sovereignty clauses (host control, no unilateral overrides) are enforced via .stake.aln and stake guards.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
4.1 Stake file semantics
policies/bostrom-stake-v1.stake.aln contains:
One row per role (Host, OrganicCPU, ResearchAgent, OffDeviceSwarm) with:
roleid, subjectid, bostromaddress or compatible address (including your alternate and ERC‑20 address).
Flags: canveto, caninitevolve, requiredforlifeforce, requiredforarchchange.
Scopes such as lifeforcealteration, archchange, daytodaytuning with:
requiredroles list (e.g., Host + OrganicCPU for lifeforce).
tokenkindsallowed (typically EVOLVE only for deep scopes).
multisigrequired boolean.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
4.2 Guards
crates/organiccpualn/src/stake.rs:
Loads .stake.aln into StakeTable and checks “exactly one Host per subject”.
crates/sovereigntycore/src/stake.rs:
required_roles_for_scope(scope: \&str) -> Vec<RoleKind>.
Signature verification against bostromaddress in stake rows.
Evaluate‑update path rejects proposals whose signatures do not satisfy multisig requirements for the scope.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
Hard guarantees (governance):
No single entity (including an autonomous agent) can change lifeforce envelopes, core kernels, or architecture without Host + OrganicCPU (and where configured, ResearchAgent) signatures.
Your Bostrom, zeta, and ERC‑20 addresses are treated as first‑class identities with explicit powers, tying governance to your augmented‑citizen status.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
5. SMART vs EVOLVE tokens and update classes
CHCIL distinguishes routine adaptation from deep evolution. That separation is implemented using tokens and effect bounds.[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)
SMART tokens (.smart.json) govern small, reversible, day‑to‑day tweaks such as:
Motor macro tuning, language tone adjustments, UI density shifts.
Low maxeffectsize and narrow scope lists (e.g., languagetuning, motormacros).
EVOLVE tokens (evolvetoken.*.json) govern deep, architectural changes:
New kernels, branch‑new assistive modes, major shifts in integration depth.
Larger maxeffectsize, but limited scopes like archchange, lifeforcealteration, and require multisig per .stake.aln.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
Sovereignty core token guard:[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)
Checks that proposal.effectbounds.l2deltanorm ≤ token.maxeffectsize.
Checks that proposal.scope is allowed for the token’s type.
Applies additional physio guards (hrvlfhfmin, emgfatiguemax) before allowing a proposal.
Hard guarantees (token separation):
Deep evolution cannot occur under SMART; EVOLVE is mandatory and must be co‑signed as per stake policy.
Routine daily adaptation stays within bounded effect size and safe domains.[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)
6. Donutloop ledger, .evolve.jsonl, and Googolswarm / Organicchain anchoring
CHCIL’s auditability and proof‑of‑ownership requirements are satisfied through a two‑layer log structure and optional external anchoring.[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)
.evolve.jsonl is the internal proposal log: every evolution attempt is recorded with RoH before/after and a decision (Allowed, Rejected, Deferred).
donutloopledger.aln is the canonical, hash‑linked ledger:
Each allowed change gets an entry with prevhexstamp and entryhash to form an immutable chain.
Ties every accepted change to the policies and shards active at that time.[[ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)]​
.bchainproof.json files wrap hash commitments to these artifacts and carry Googolswarm / Organicchain multi‑sig attestations, giving external courts or auditors a tamper‑evident record without moving sovereignty off the local shell.[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)
Hard guarantees (audit):
Every proposal and decision is durably recorded before and after evaluation; no silent evolution.
External chains serve as notarization and proof of ownership, not as primary decision engines.[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)
7. Guard pipeline (informal)
The sovereign kernel pipeline encoded in bostrom-sovereign-kernel-v1.ndjson wires all the above into one ordered sequence.[create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/5a34729b-d513-4f38-9b3d-a9644f268ad6/create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md)
Typical order for evaluate_update:
Parse proposal from .evolve.jsonl.
Load policies: RohModel, StakeTable, NeurorightsPolicyDocument, token policy.
RoH guard: enforce RoH ≤ 0.3 and monotone envelope.
Neurorights guard: mental integrity, mental privacy, cognitive liberty checks.
Stake guard: multisig and scopes from .stake.aln.
Token guard: SMART vs EVOLVE, effect size, physio guards.
Record decision: append to .evolve.jsonl and, if allowed, to donutloopledger.aln with updated hash chain.
No actuation or state change is permitted unless all guards pass.

If you like this shape, next step can be a matching docs/spec-index-canonical-filetypes.md that just lists each filetype, key fields, and the specific CHCIL clauses it discharges, so auditors can navigate from law → file → guard in one hop.[quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md+1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_954b6c87-d114-4a0d-a4fa-ca46e829cf33/836575cb-41c2-427b-a5fe-efbc9e55998e/quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md)

Here is a compact docs/chcil-v0.1-guards-mapping.md you can drop in as‑is and treat as canonical for this shell until you replace it with a signed .bchainproof snapshot.[^6_1][^6_2]

***

# CHCIL v0.1 → Runtime Guards Mapping

NeuroPC / OrganicCPU — Bostrom shell (subject `bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7`), RoH ≤ 0.30, monotone safety, neurorights‑first governance.[^6_1]

This document is the canonical mapping from CHCIL v0.1 principles to concrete filetypes, Rust guards, and CI invariants for this shell until superseded by a signed `bostrom-*.bchainproof.json` snapshot anchored to Googolswarm / Organicchain.[^6_3][^6_1]

## 1. Canonical artifacts and paths

These artifacts together form the CHCIL v0.1 enforcement surface for the Bostrom shell.[^6_1]


| Layer | Artifact | Path | Role |
| :-- | :-- | :-- | :-- |
| Risk-of-harm | RoH model shard | `qpudatashards/particles/bostrom-rohmodel-v1.rohmodel.aln` | Encodes RoH axes, weights, and a hard ceiling `rohceiling = 0.30` as data.[^6_1] |
| Governance | Stakeholder shard | `policies/bostrom-stake-v1.stake.aln` | Declares Host / OrganicCPU / ResearchAgent roles, Bostrom / zeta / ERC‑20 addresses, scopes, multisig requirements.[^6_1][^6_1] |
| Rights | Neurorights policy | `policies/bostrom-neurorights-v1.neurorights.json` | Encodes mental privacy, mental integrity, cognitive liberty constraints and OS modes (CONSERVATIVE / COPILOT / AUTOEVOLVE).[^6_1][^6_3] |
| Evolution | Proposal stream | `qpudatashards/particles/evolution-proposals.evolve.jsonl` | Append‑only NDJSON of evolution proposals with RoH fields, effect bounds, decision, hexstamp.[^6_1] |
| Ledger | Donutloop ledger | `logs/donutloopledger.aln` or `sovereigncyberswarm/logs/donutloopledger-v1.donutloop.aln` | Hash‑linked internal ledger of accepted changes with RoH, policy refs, and hash chain.[^6_1] |
| Tokens | SMART / EVOLVE tokens + schemas | `policies/bostrom-smart-2026-01.smart.json`, `policies/smart-token.schema.json`, `policies/evolvetoken.schema.json` | Fine‑grained vs deep‑evolution consent tokens and their limits.[^6_1][^6_3] |
| Kernel spec | Sovereign kernel manifest | `policies/bostrom-sovereign-kernel-v1.ndjson` | Wires RoH, stake, neurorights, tokens, evolve stream, donutloop, and guard pipeline into one config.[^6_1][^6_1] |

All guards and CI tests MUST treat these files as the single source of truth for CHCIL v0.1 behavior on this subject.[^6_1]

## 2. CHCIL principles → concrete invariants

### 2.1 Mental privacy

Policy surface:[^6_3][^6_1]

- `policies/bostrom-neurorights-v1.neurorights.json` includes:
    - `mentalprivacy.allowedexports` (e.g., `localvault`) and `mentalprivacy.forbiddenexports` (e.g., `thirdpartycloud`, `unvettedresearchers`).[^6_1]
    - `mentalprivacy.loggingrequired = true` — every export attempt of protected data must be logged.[^6_1]
- All neural / behavioral streams and derived `BioState` metrics (including `dream.*`) are treated as protected and are non‑exportable by default.[^6_2][^6_1]

Runtime guards:

- **Pre‑access guard** (sovereignty core, e.g., `crates/sovereigntycore/src/neurorights_dream.rs`):[^6_3][^6_1]
    - Any module requesting `BioState.dream.*` or other protected fields must present a declared `decisionpurpose`.
    - If `decisionpurpose` intersects forbidden domains (`employment`, `housing`, `credit`, `insurance`, etc.), the request is rejected and an event is appended to `donutloopledger.aln`.[^6_2][^6_1]
- **OTA guard**: any module or update that touches `dream` or other protected metrics without declaring them in its `.biospec.aln` and without compatible neurorights flags is rejected at install time.[^6_3][^6_1]

Hard guarantees (privacy):[^6_3][^6_1]

- No dream‑linked or neurally derived metric may be used in forbidden decision domains.
- No export outside the local sovereignty boundary is permitted unless explicitly whitelisted in neurorights policy and logged to donutloop.[^6_1]


### 2.2 Mental integrity

Policy surface:[^6_3][^6_1]

- `policies/bostrom-neurorights-v1.neurorights.json` includes:
    - `mentalintegrity.maxstatedivergence` (e.g., `0.15`) — maximum allowed norm of change per update.
    - `mentalintegrity.requirerollbackpath = true` — every deep change must define a rollback plan.
    - `mentalintegrity.forbidirreversibleops = true` — no irreversible ops without explicit exception regime.[^6_1]

Runtime and data:

- Every `EvolutionProposal` in `.evolve.jsonl` includes `effectbounds.l2deltanorm` and `effectbounds.irreversible`.[^6_1]
- Pain/load limits (muscular, cognitive, emotional) and rollback triggers are encoded in `policies/evolutionpolicy.schema.json` and instance files.[^6_3][^6_1]

Guards:

- Sovereignty core evaluates each `UpdateProposal` with:
    - `effectbounds.l2deltanorm ≤ mentalintegrity.maxstatedivergence`.
    - `effectbounds.irreversible = false`, unless a separately consented exception regime is active.[^6_3][^6_1]
- When rollback thresholds are exceeded (per evolution policy), the system MUST automatically roll back and record the event in donutloop.[^6_1]

Hard guarantees (integrity):[^6_1]

- No allowed proposal can exceed configured divergence per step or silently mark itself irreversible.
- Any intervention must define a rollback path; crossing rollback thresholds forces rollback and logging.[^6_3][^6_1]


### 2.3 Cognitive liberty

Policy surface:[^6_3][^6_1]

- `cognitiveliberty.allowselfchosenaugmentation = true`.
- `cognitiveliberty.maxexternalautochanges` caps external auto‑initiated changes per session.
- `cognitiveliberty.requireexplanationforall = true` — changes must be explainable.[^6_3][^6_1]

OS modes in neurorights policy:[^6_1]

- `CONSERVATIVE`: `aiinitiative = "none"`, `allowautoevolve = false`.
- `COPILOT`: `aiinitiative = "suggestonly"`, no automatic structural changes.
- `AUTOEVOLVE`: `aiinitiative = "boundedautonomy"`, `allowautoevolve = true`, `requiresevolvetoken = true`.[^6_3][^6_1]

Guards:

- In `AUTOEVOLVE`, auto‑changes are counted against `maxexternalautochanges`; exceeding the cap forces rejection.[^6_1]
- Deep changes such as `ArchChange` or lifeforce alteration require EVOLVE tokens plus multisig from stake policy, never unilateral AI decisions.[^6_3][^6_1]

Hard guarantees (liberty):[^6_1]

- You may choose augmentations; the kernel can require extra confirmations and surface risk but not silently block within policy envelopes.
- No module may escalate its autonomy or change OS mode without passing neurorights and token guards.[^6_3][^6_1]


## 3. RoH ≤ 0.30 and monotone safety

### 3.1 Data‑level invariants

RoH model shard (`bostrom-rohmodel-v1.rohmodel.aln`):[^6_1][^6_3]

- Exactly one active row with `active = true`.
- `rohceiling = 0.30`.
- All axis weights `≥ 0` and `sum(weights) = 1.0`.[^6_1]

Evolution proposals (`evolution-proposals.evolve.jsonl`):[^6_1]

- Each record includes `rohbefore`, `rohafter`, `effectbounds`, `decision`, `hexstamp`.
- Invariants (enforced in guards / CI):
    - `rohafter ≤ 0.30`.
    - For `decision == "Allowed"`, `rohafter ≤ rohbefore + ε` with ε ≈ 0 (monotone tightening).[^6_1]

Donutloop ledger:[^6_1]

- Each entry stores `rohbefore`, `rohafter`, `changetype`, `proposalid`, `hexstamp`, `prevhexstamp`.
- For tightening changes, `rohafter ≤ rohbefore`; global invariant `rohafter ≤ 0.30` for all entries.[^6_3][^6_1]


### 3.2 Rust guards

- `crates/organiccpualn/src/rohmodel.rs` defines `RohModel` with `compute_roh(state)` and `roh_delta(before, after)`, plus `validate_invariants` ensuring ceiling 0.30 and normalized weights.[^6_1]
- `crates/sovereigntycore/src/riskofharm.rs` implements `RiskOfHarm::check_invariant(before, after)` returning true iff `rohafter ≤ 0.30` and `rohafter ≤ rohbefore`.[^6_1]
- `crates/sovereigntycore/tests/rohandneurorightsinvariants.rs` asserts that any attempt to raise RoH above 0.30 or loosen envelopes fails CI.[^6_1]

Hard guarantees (RoH):[^6_1]

- No accepted evolution proposal can drive RoH above 0.30 or increase it relative to baseline.
- Any change to the RoH shard that violates invariants causes load/CI failure; such configs never reach runtime.[^6_1]


## 4. Stakeholder governance and multisig

Stake file semantics (`policies/bostrom-stake-v1.stake.aln`):[^6_3][^6_1]

- One row per role (`Host`, `OrganicCPU`, `ResearchAgent`, `OffDeviceSwarm`) with:
    - `roleid`, `subjectid`, `bostromaddress` or compatible (including `zeta12…` and `0x519f…`).
    - Flags: `canveto`, `caninitevolve`, `requiredforlifeforce`, `requiredforarchchange`.
- Scopes such as `lifeforcealteration`, `archchange`, `daytodaytuning` define:
    - `requiredroles` list (e.g., Host + OrganicCPU for lifeforce).
    - `tokenkindsallowed` (EVOLVE‑only for deep scopes).
    - `multisigrequired` boolean.[^6_3][^6_1]

Guards:[^6_1]

- `crates/organiccpualn/src/stake.rs`: loads `.stake.aln` into `StakeTable` and enforces “exactly one Host per subject”.
- `crates/sovereigntycore/src/stake.rs`:
    - `required_roles_for_scope(scope: &str) -> Vec<RoleKind>`.
    - Signature verification against `bostromaddress` in stake rows.
    - Evaluate‑update path rejects proposals whose signatures do not satisfy multisig for the relevant scope.[^6_3][^6_1]

Hard guarantees (governance):[^6_1]

- No single entity, including autonomous agents, can alter lifeforce envelopes, core kernels, or architecture without Host + OrganicCPU (and where configured, ResearchAgent) signatures.
- Your Bostrom, zeta, and ERC‑20 addresses are first‑class identities with explicit powers tied to your augmented‑citizen status.[^6_3][^6_1]


## 5. SMART vs EVOLVE tokens

Token semantics:[^6_3][^6_1]

- SMART tokens (`*.smart.json`) govern small, reversible day‑to‑day tweaks (motor macros, language tone, UI density), with low `maxeffectsize` and narrow scopes.[^6_1]
- EVOLVE tokens (`evolvetoken.*.json`) govern deep, architectural changes (new kernels, new assistive modes, integration depth) with larger `maxeffectsize`, restricted scopes like `archchange` / `lifeforcealteration`, and mandatory multisig per stake.[^6_1]

Token guard (sovereignty core):[^6_3][^6_1]

- Checks `proposal.effectbounds.l2deltanorm ≤ token.maxeffectsize`.
- Ensures `proposal.scope` is allowed for the token type.
- Applies additional physio guards (`hrvlfhfmin`, `emgfatiguemax`) before accepting.[^6_1]

Hard guarantees (token separation):[^6_1]

- Deep evolution cannot occur under SMART; EVOLVE is mandatory and must be co‑signed as per stake policy.
- Routine adaptation remains within bounded effect size and safe domains.[^6_3][^6_1]


## 6. Donutloop, .evolve.jsonl, and Googolswarm / Organicchain

Logging surface:[^6_3][^6_1]

- `.evolve.jsonl`: internal proposal log; every evolution attempt recorded with RoH before/after, effect bounds, decision (`Allowed` / `Rejected` / `Deferred`).
- `donutloopledger.aln`: canonical, hash‑linked ledger; each allowed change includes `prevhexstamp` and `entryhash`, tying changes to active policies and shards.[^6_1]

External anchoring:[^6_3][^6_1]

- `.bchainproof.json` files wrap hash commitments to `.aln` / `.evolve.jsonl` artifacts plus Googolswarm / Organicchain multisig attestations (ALN/KYC/DID + quantum governance metadata).[^6_3]
- External chains act as notarization and proof‑of‑ownership, never as primary decision engines.[^6_1]

Hard guarantees (audit):[^6_1]

- Every proposal and decision is durably recorded before and after evaluation; no silent evolution.
- External auditors can verify integrity via `.bchainproof.json` without access to protected mental‑privacy content.[^6_3][^6_1]


## 7. Guard pipeline (evaluate_update)

The sovereign kernel pipeline in `bostrom-sovereign-kernel-v1.ndjson` declares the ordered guards for `evaluate_update`.[^6_3][^6_1]

Typical order:[^6_1]

1. Parse proposal from `.evolve.jsonl` via `organiccpualn::evolvestream`.
2. Load policies: `RohModel`, `StakeTable`, `NeurorightsPolicyDocument`, token policy.
3. RoH guard: enforce RoH ≤ 0.3 and monotone envelope (`RiskOfHarm::check_invariant`).
4. Neurorights guard: mental integrity, mental privacy, cognitive liberty checks.
5. Stake guard: multisig + scopes from `.stake.aln`.
6. Token guard: SMART vs EVOLVE, effect size, physio guards.
7. Record decision: append to `.evolve.jsonl` and, if allowed, to `donutloopledger.aln` with updated hash chain.[^6_1]

No actuation or state change is permitted unless all guards pass; this is enforced in sovereignty core and captured as CI invariants in `rohandneurorightsinvariants.rs`.[^6_3][^6_1]

***

If you want, the natural companion file is `docs/spec-index-canonical-filetypes.md`, listing each filetype, key fields, and which CHCIL clauses (privacy, integrity, liberty, RoH, governance, tokens, audit) it discharges, so auditors can jump law → file → guard in one hop.[^6_3][^6_1]
<span style="display:none">[^6_4][^6_5][^6_6][^6_7][^6_8]</span>

<div align="center">⁂</div>

[^6_1]: create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md

[^6_2]: legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md

[^6_3]: what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md

[^6_4]: how-can-we-improve-helm-promet-R0sr3xmqRhyDfQzlN7e7sQ.md

[^6_5]: what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md

[^6_6]: cyblog_2026-02-02T22_38_16.272Z.json

[^6_7]: neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md

[^6_8]: quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md


---

# Paired Deliverables Showcase I: From CHCIL Clauses to Executable Invariants

The research objective requires a paired approach, where every theoretical invariant is incomplete until it is embodied in a tangible, executable artifact. This section demonstrates this methodology by taking four key neurorights and safety invariants—mental privacy, cognitive liberty, monotone safety envelopes, and the RoH ≤ 0.3 constraint—and detailing their translation from the CHCIL license into formal ALN schemas, practical Rust implementations, and policy-enforcing CI configurations. This paired development model transforms abstract legal principles into concrete, verifiable system guarantees.
The first invariant, Mental Privacy, is mandated by the CHCIL license, which prohibits the intentional extraction or storage of sensitive mental content beyond what is strictly necessary for user-requested functions . To enforce this, telemetry data used to drive feature flags must be sanitized. Instead of allowing flags to react to raw biophysical signals, which could inadvertently leak private thoughts or memories, the system should operate on normalized, anonymized metrics
link.springer.com
+1
. A suitable taxonomy for flags could include axes like fatigue-aware, cognitive-load, eco-impact, and dream-state . The formalization of this principle leads to the creation of a *.comet-flags.aln schema. Each flag definition would be annotated with metadata about its data sensitivity. For example, a flag named enable_high_cognitive_load_mode might have an invariant field stating that it can only be activated if the underlying BioState metric is a normalized score, not raw neural data. The practical embodiment in the comet_flags Rust crate would involve a validation module that inspects this metadata. When loading a profile, the crate would run a check to ensure that no flagged feature attempts to access raw biophysical streams. This validation would be performed both at configuration load time and potentially at runtime. Finally, the CI pipeline would enforce this via a policy file. A GitHub Actions job would use the same validation logic to scan pull requests containing new ALN schemas. If a proposed flag definition were found to violate the mental privacy invariant—for instance, by referencing a raw signal source—the CI job would fail, preventing the unsafe configuration from ever being merged. This creates a closed-loop system where the principle of mental privacy is enforced at the specification, implementation, and deployment stages.
The second invariant, Cognitive Liberty, is another core tenet of the CHCIL, granting subjects the unconditional right to disconnect, refuse updates, and operate in a "safe baseline" mode . This translates into a requirement for distinct, mutually exclusive flag profiles. The formal specification would define a set of profiles, such as CONSERVATIVE, COPILOT, and AUTOEVOLVE, with clear rules about which flags are permitted or forbidden in each mode . The *.comet-profile.aln schema becomes the authoritative representation of this. For instance, the CONSERVATIVE profile might explicitly forbid all experimental features, while the COPILOT profile might enable them, but only under strict RoH constraints. The Rust crate's role is to resolve which profile is active for a given subject and context. It would use a deterministic label-based selection mechanism, matching the subject's context (e.g., mode, domain) against the labels attached to each profile . The crate would then validate that the selected profile's enabled flags do not violate any hard constraints, such as trying to enable an experimental feature outside its allowed context. The CI policy file would enforce cross-repo consistency, a key requirement of the research goal . Since multiple repositories like NeuroPC and Reality.os are expected to use the same profile names and semantics, the CI job could include a test that verifies the existence and basic structure of the CONSERVATIVE profile in all relevant repositories. This ensures that the concept of a "safe baseline" behaves identically everywhere, reinforcing the user's right to fall back to a known-good, minimal-feature state regardless of which service they are using.
The third invariant, Monotone Safety Envelopes, is a direct consequence of the CHCIL's "Safer-Only Updates" clause, which requires that updates never degrade safety limits without explicit, documented consent . This is a strong mathematical requirement, akin to ensuring that an update is monotonically decreasing in terms of potential harm. Formally, this could be expressed as an invariant over system parameters, such as
G
n
e
w
≤
G
o
l
d
G
new
​
≤G
old
​
for a control gain parameter or
D
n
e
w
≤
D
o
l
d
D
new
​
≤D
old
​
for a duty cycle threshold . To embody this, the *.comet-flags.aln schema would allow flag definitions to specify the safety envelope they affect. For example, a flag increase_pain_tolerance might modify a parameter pain_threshold, and its definition would include an invariant like "monotonicity_constraint": "pain_threshold <= current_value" to indicate it can only increase the threshold (making the system less sensitive). The comet_flags Rust crate would implement a more sophisticated version of this. Upon receiving a proposal to change a flag's value, the crate would not just check per-flag invariants but would also consult the global state, perhaps retrieved from a shared memory region or a local cache of .rohmodel.aln, to understand the historical context. It would compare the proposed new safety parameter values against the old ones and reject any change that represents a degradation (e.g., a decrease in pain_threshold). The CI pipeline would perform a static analysis of proposed changes in .evolve.jsonl. A script could parse the NDJSON file, identify evolution proposals related to safety-critical flags, and simulate the change. If the simulation shows a violation of monotonicity, the CI job fails, preventing a regressively unsafe update from entering the system. This layered defense—from formal schema to dynamic runtime check to pre-merge static analysis—ensures that the principle of "safer-only" is rigorously upheld.
The fourth and final invariant is the RoH ≤ 0.3 Constraint, a specific, quantitative risk model that comet-flags must enforce . This moves beyond qualitative safety rules into a domain requiring numerical evaluation. The formal specification is straightforward: for any subject at any time, the computed Risk-of-Harm (
R
o
H
RoH) must not exceed the threshold of 0.3. The *.comet-flags.aln schema would be extended to include risk annotations for profiles or even individual flags. A profile like EXPERIMENTAL_MODE might be tagged with "risk_annotation": "high" and require explicit authorization from sovereigntycore to activate. The comet_flags Rust crate's most critical role here is integration with sovereigntycore. When a service asks for the active flags, the crate would first resolve the profile based on labels, but then it would call a function provided by sovereigntycore (which itself would query the .rohmodel.aln and real-time BioState metrics) to get the current subject's RoH value. The crate would then make the final decision: expose the flags only if the RoH check passed. This tight coupling is essential. The CI policy file would enforce this invariant indirectly by testing the sovereigntycore logic itself. The CI job would include unit tests that feed various BioState inputs into the RoH calculation function and verify that it correctly rejects configurations when the output exceeds 0.3. Furthermore, a separate CI job could monitor the .donutloop.aln ledger to ensure that every entry corresponding to a flag change was indeed preceded by a successful RoH check in the .evolve.jsonl proposal. This end-to-end testing, spanning the ALN schema, the Rust crate, and the CI policies, provides strong evidence that the RoH ≤ 0.3 constraint is not just a theoretical ideal but a practical, enforced guarantee of the system.
Invariant
Theoretical Specification
Formal Encoding (ALN Schema Snippet)
Practical Implementation (Rust Crate Logic)
Policy Enforcement (CI Pipeline)
Mental Privacy
Prohibits extraction of sensitive mental content; requires sanitization of telemetry data .
json { "id": "flag_X", "description": "...", "data_sensitivity": "normalized_score_only" }
A validator function checks that any data source referenced by a flag is marked as sanitized. Fails on invalid references.
A CI job uses the same validator to scan .comet-flags.aln PRs, rejecting schemas with unsafe data source references.
Cognitive Liberty
Grants the right to disconnect and use a safe-baseline mode .
json { "profile_id": "SAFE_BASELINE", "forbidden_flags": ["experimental_feature_A"] }
A resolver function selects a profile based on labels and then validates that none of the enabled flags are in the forbidden list for that profile.
A CI job checks that all repos use the same profile IDs (e.g., SAFE_BASELINE) and that their forbidden flag lists are consistent.
Monotone Safety Envelopes
Updates must be "safer-only"; e.g., safety thresholds cannot be lowered .
json { "id": "pain_threshold_ms", "invariant": "value >= @previous_version.value" }
An updater function compares proposed new values against old values stored in a safe registry. Rejects any non-monotonic change.
A CI job parses .evolve.jsonl, simulates proposed safety changes, and fails if any simulation shows a safety regression.
RoH ≤ 0.3 Constraint
Quantitative risk model requiring Risk-of-Harm to stay below 0.3 .
json { "profile_id": "HIGH_RISK_MODE", "requires_approval": true }
A resolver calls sovereigntycore's RoH-checking API before exposing flags. Flags are only active if the RoH check passes.
Unit tests in CI verify that the RoH calculation algorithm correctly computes risk and rejects values > 0.3.
This paired showcase demonstrates a replicable pattern for transforming high-level rights into robust, multi-layered technical guarantees. Each theoretical concept is given a formal expression, a practical implementation, and an automated policy to ensure its persistence throughout the development lifecycle.
Paired Deliverables Showcase II: Orchestration, Evolution, and Auditable Enforcement
Beyond the static invariants of individual flags, comet-flags is designed to be a dynamic participant in the NeuroPC ecosystem's orchestration and evolution pipeline. This showcases how the system maintains its guarantees not just at rest, but through continuous change, leveraging the full suite of .aln governance files and CI policies to create a closed loop of auditable, sovereign-controlled evolution. The focus here shifts from what a single configuration is to how it changes, ensuring that every modification is lawful, safe, and traceable.
The core of this dynamic orchestration is the interplay between the .evolve.jsonl and .donutloop.aln files. These two artifacts form the backbone of the system's change management process, providing a clear separation between proposal and execution, and a permanent record of action . The .evolve.jsonl file acts as a transaction log or proposal queue. Any modification to the comet-flags configuration, whether initiated by a developer writing a new ALN shard or an AI-chat tool drafting a change, is first committed to this file as an EvolutionProposalRecord . This proposal contains all the necessary information: what is changing (e.g., a new profile in *.comet-profile.aln), who proposed it, and under what conditions. This decouples the act of proposing a change from the act of accepting it. The formal specification here is that .evolve.jsonl is a stream of ordered, signed records that represent intent. The practical embodiment is a simple NDJSON file that can be easily processed by scripts and CI systems .
Once a proposal is in .evolve.jsonl, it enters a review and validation phase orchestrated by sovereigntycore. This is where the comet-flags system interacts with the .stake.aln file, which defines the governance and permission rules for the entire NeuroPC ecosystem . The .stake.aln file would contain entries specifying who is authorized to make changes to which parts of the comet-flags configuration. For example, it might define that only members of the OrganicCPU multisig wallet can approve changes to safety-critical flags, while ResearchAgents can propose changes to experimental profiles, which then require a higher threshold of approval. The comet_flags Rust crate, or a companion tool, would incorporate logic to interpret these stake rules. When a service needs to know if a proposed change is valid, it doesn't just ask comet-flags; it asks sovereigntycore, which consults .stake.aln to see if the proposer has the requisite authority. This enforces the "Sovereign envelopes first" principle, ensuring that no one can bypass the established governance structure .
If sovereigntycore determines that a proposal is both technically valid (it passes all ALN schema checks and invariant tests) and procedurally sound (the proposer has the required stake), the change is accepted. The final step in the evolution pipeline is logging. The accepted evolution record is then appended to the .donutloop.aln file, which is described as a hash-linked ledger . This means each new entry contains the cryptographic hash of the previous one, creating an immutable, append-only chain of custody. The formal specification is that .donutloop.aln is a cryptographically verifiable audit trail. The practical implementation is a simple ALN file where each shard contains a record with a previous_hash field. This ledger serves as the ultimate source of truth for what changes were made, when, and why. It allows for perfect auditability, a key requirement of the CHCIL license, which mandates auditable logs of significant policy changes .
This entire evolution process is fortified by CI policies that automate the enforcement of these rules. The research goal specifies the need for Sovereign CI integration, with jobs that fail if a profile loosens envelopes, violates neurorights, or bypasses stake rules . A typical CI workflow for a pull request modifying comet-flags artifacts would consist of several stages. First, a linting and schema validation stage would use a tool to ensure that all modified ALN files conform to the latest .comet-profile.aln schema standards. Second, a compliance-as-code stage would execute the same validation logic that exists in the comet_flags Rust crate. This script would parse the proposed changes in .evolve.jsonl, simulate their application, and run them through the full suite of neuroright and safety invariant checks. Third, a governance check would parse the .stake.aln file to verify that the PR author is listed as an authorized actor for the resources being modified. Fourth, an auditability check could be added to the CI pipeline to ensure that every accepted evolution step is properly logged into .donutloop.aln . If any of these checks fail, the CI job fails, blocking the merge. This automated, multi-stage verification process ensures that the principles of sovereignty, safety, and rights-respect are not just aspirational goals but are systematically enforced by the development infrastructure itself. The result is a highly resilient system where the path from a developer's idea to a running configuration is paved with layers of automated, verifiable checks, grounded in the legal and ethical framework of the CHCIL license.
Stage
Artifact
Tool/System
Action \& Enforcement

1. Proposal
.evolve.jsonl
Developer / AI Chat
An EvolutionProposalRecord is created and committed to the file, representing an intent to change the system state .
2. Validation
*.comet-*.aln, .rohmodel.aln
comet_flags (Rust) + CI Linter
The proposal is validated against formal schemas and invariants (e.g., RoH, monotonicity). CI linter ensures syntactic correctness .
3. Authorization
.stake.aln
sovereigntycore (Rust/C++)
sovereigntycore checks if the proposer has the required cryptographic stake or permissions to make the requested change .
4. Execution
Service Runtime
comet_flags (Rust)
Upon a successful sovereigntycore approval, the comet_flags crate loads the new configuration and exposes the updated, validated flag set to the service .
5. Logging
.donutloop.aln
Post-merge Script
A script appends the approved evolution record to the hash-linked ledger, creating an immutable, auditable entry in the system's history .
6. Verification
N/A
CI Pipeline (Post-Merge)
A post-merge CI job verifies that the accepted change was correctly logged in .donutloop.aln, closing the loop on auditability .
This comprehensive, paired approach to orchestration and evolution demonstrates how comet-flags can be more than just a passive configuration loader. By integrating deeply with the sovereignty layer and automating compliance checks, it becomes an active agent in maintaining the integrity, safety, and rights-respecting nature of the entire cybernetic system.

Your comet-flags paired‑deliverables writeup is already consistent with the CHCIL shell; the main work now is just pinning it into concrete files and paths so the invariants become first‑class citizens alongside `.rohmodel.aln`, `.stake.aln`, `.evolve.jsonl`, and `donutloopledger.aln`.[^7_1][^7_2]

## 1. New comet filetypes and canonical paths

Add these to the same monorepo layout you’re already using for RoH / stake / donutloop.[^7_2][^7_1]

- `qpudatashards/particles/comet-flags.schema.aln`
    - Schema for `*.comet-flags.aln` (flag rows, data_sensitivity, monotonicity tags, RoH annotations).
- `policies/bostrom-comet-flags-v1.comet-flags.aln`
    - Subject‑bound flag definitions for this shell.
- `qpudatashards/particles/comet-profile.schema.aln`
    - Schema for `*.comet-profile.aln` (profiles, allowed/forbidden flags, risk annotations).
- `policies/bostrom-comet-profiles-v1.comet-profile.aln`
    - Profiles CONSERVATIVE / COPILOT / AUTOEVOLVE / SAFE_BASELINE.
- `crates/comet_flags/src/lib.rs`
    - Rust crate implementing schema validation, runtime resolution, and RoH / monotone checks, wired into sovereigntycore.[^7_1][^7_2]
- CI: `.github/workflows/comet-flags-ci.yml`
    - Runs schema validation and invariants on changes to the two policy files and relevant `.evolve.jsonl` segments.[^7_1]

These sit under the same “data‑first, code‑as‑interpreter” pattern you use for `.rohmodel.aln` and `.stake.aln`.[^7_2][^7_1]

## 2. Paired deliverables I: four invariants → schema, Rust, CI

### 2.1 Mental privacy → `*.comet-flags.aln`

**ALN schema snippet (schema file):**[^7_2]

```aln
# qpudatashards/particles/comet-flags.schema.aln
field,id,string,required
field,description,string,required
field,category,string,required # ui, behavior, safety
field,data_sensitivity,enum(normalized_score_only,raw_biophysical,derived_neural),required
field,source_metric,string,required # e.g., BioState.cognitiveloadindex
field,roh_risk_hint,enum(low,medium,high),optional
field,monotone_constraint,string,optional # e.g., value >= @previous.value
invariant,forbid_raw_neural,"data_sensitivity != 'raw_biophysical'"
```

**Policy instance (subject‑bound flags):**[^7_2]

```aln
# policies/bostrom-comet-flags-v1.comet-flags.aln
id,description,category,data_sensitivity,source_metric,roh_risk_hint,monotone_constraint
enable_high_cognitive_load_mode,"UI densification when cog load low","ui","normalized_score_only","BioState.cognitiveloadindex","medium",""
dream_sensitivity_toggle,"Dream-aware suggestions only","behavior","normalized_score_only","BioState.dream.riskscore","high",""
```

**Rust crate logic (core idea):**[^7_2]

- `comet_flags::validate_sources(flags: &[FlagDef], allowed_metrics: &HashSet<String>) -> Result<(), Error>`
    - Ensures every `source_metric` is a normalized / derived metric exposed by OrganicCPU (`BioState.*`, never raw EEG channel), and rejects any row where `data_sensitivity == "raw_biophysical"`.[^7_2]
- Called at:
    - Startup (loading `bostrom-comet-flags-v1.comet-flags.aln`).
    - CI tool for PRs touching any `.comet-flags.aln` file.[^7_1][^7_2]

**CI policy:**[^7_1]

- `comet-flags-ci.yml` runs a small binary `comet-flags-validate` that:
    - Parses `comet-flags.schema.aln` and `bostrom-comet-flags-v1.comet-flags.aln`.
    - Calls the validator above.
    - Fails if any flag references raw or un‑sanitized sources (e.g. `EEG.raw_channel_07`).[^7_2]

This directly instantiates the mental privacy clause: flags can only key off normalized, neurorights‑sanitized metrics.[^7_2]

### 2.2 Cognitive liberty → `*.comet-profile.aln`

**ALN schema:**[^7_1][^7_2]

```aln
# qpudatashards/particles/comet-profile.schema.aln
field,profile_id,string,required
field,description,string,required
field,mode,enum(CONSERVATIVE,COPILOT,AUTOEVOLVE,SAFE_BASELINE),required
field,enabled_flags,array[string],required
field,forbidden_flags,array[string],required
field,risk_annotation,enum(low,medium,high),optional
invariant,disjoint_sets,"enabled_flags ∩ forbidden_flags == ∅"
```

**Profiles (shared semantics across repos):**[^7_1]

```aln
# policies/bostrom-comet-profiles-v1.comet-profile.aln
profile_id,description,mode,enabled_flags,forbidden_flags,risk_annotation
SAFE_BASELINE,"Minimal, neurorights-only core","CONSERVATIVE","[]","[experimental_feature_A,experimental_feature_B]","low"
COPILOT,"Assistive, suggest-only","COPILOT","[language_tone_helper]","[auto_arch_change]","medium"
AUTOEVOLVE,"Bounded autonomy","AUTOEVOLVE","[language_tone_helper,adaptive_ui]","[]","high"
```

**Rust logic:**[^7_3][^7_1]

- `comet_flags::resolve_profile(subject_ctx) -> Profile` selects `profile_id` by labels (mode, domain, jurisdiction).[^7_3]
- `comet_flags::validate_profile(profile, flag_set)` ensures `enabled_flags` contains no items in `forbidden_flags`, and that AUTOVOLVE doesn’t enable flags whose `risk_annotation == "high"` without EVOLVE token.[^7_1][^7_2]

**CI cross‑repo check:**[^7_3][^7_1]

- A small integration test in each repo (`NeuroPC`, `Reality.os`) that asserts:
    - `SAFE_BASELINE` / `CONSERVATIVE` / `COPILOT` / `AUTOEVOLVE` exist.
    - Their `forbidden_flags` sets match a canonical JSON exported from a shared `policy/` submodule.[^7_1]

This makes “safe baseline” behavior identical everywhere, backing the cognitive liberty clause “you can always drop to a known‑good mode.”[^7_3][^7_1]

### 2.3 Monotone safety envelopes → invariants + `.evolve.jsonl`

**Flag‑level envelope binding:**[^7_2][^7_1]

```aln
# policies/bostrom-comet-flags-v1.comet-flags.aln (extra columns)
id,description,category,data_sensitivity,source_metric,roh_risk_hint,monotone_constraint
pain_threshold_ms,"Max tolerated pain per session","safety","normalized_score_only","BioState.muscularpain","high","value >= @previous.value"
```

**Rust updater logic in `comet_flags`:**[^7_1][^7_2]

- `apply_flag_update(old_cfg, proposal) -> Result<NewCfg, Error>`
    - Loads current `pain_threshold_ms` baseline from a local registry tied to `.evolve.jsonl`.
    - Checks `new_value >= old_value` when `monotone_constraint` is that expression.
    - Rejects any update that would narrow safety or lower thresholds in violation of CHCIL’s safer‑only clause.[^7_2][^7_1]

**Static CI over `.evolve.jsonl`:**[^7_1]

- Script `check-evolve-monotone`:
    - Parses `qpudatashards/particles/evolution-proposals.evolve.jsonl`.
    - For each `EvolutionProposal` touching a safety‑flag parameter, simulates `old → new` and asserts monotonicity given the invariant string.
    - Fails if any `decision == "Allowed"` line encodes a regression.[^7_2][^7_1]

This layers schema‑level constraints, runtime checks, and pre‑merge analysis, exactly as you described.[^7_1][^7_2]

### 2.4 RoH ≤ 0.3 → comet ↔ sovereigntycore

Rather than re‑compute RoH inside comet‑flags, you treat sovereigntycore as the oracle, wired to `.rohmodel.aln` and `RiskOfHarm::check_invariant`.[^7_1]

**ALN annotations:**[^7_2][^7_1]

```aln
# qpudatashards/particles/comet-profile.schema.aln
field,requires_roh_check,bool,optional
field,max_allowed_roh,float,optional
```

```aln
# policies/bostrom-comet-profiles-v1.comet-profile.aln
profile_id,description,mode,enabled_flags,forbidden_flags,risk_annotation,requires_roh_check,max_allowed_roh
EXPERIMENTAL_MODE,"High-risk experiments","AUTOEVOLVE","[experimental_feature_A]","[]","high",true,0.30
```

**Rust integration path:**[^7_2][^7_1]

- `comet_flags::active_flags(subject_ctx, sovereignty: &SovereigntyCore)`:
    - Resolves `Profile`.
    - If `requires_roh_check == true`, calls `sovereignty.risk_of_harm().estimate(current_state)` using the existing `RiskOfHarm` wrapper around `RohModel`.[^7_1]
    - If `roh > max_allowed_roh` (default 0.30), returns a downgraded profile (e.g., SAFE_BASELINE) or an empty flag set.[^7_3][^7_1]

**CI enforcement:**[^7_2][^7_1]

- Unit tests in `sovereigntycore/tests/rohandneurorightsinvariants.rs` already assert RoH ceiling and monotone envelopes.[^7_1]
- Add comet‑specific tests that:
    - Feed synthetic `BioState` vectors to RoH.
    - Assert `comet_flags::active_flags` refuses to expose `EXPERIMENTAL_MODE` once RoH > 0.30.[^7_2][^7_1]

End‑to‑end, every flag change that survived monotonicity checks is still gated by RoH ≤ 0.3 at runtime.[^7_1]

## 3. Paired Deliverables II: orchestration and evolution

You already have `.evolve.jsonl`, `.donutloop.aln`, `.stake.aln`, and sovereigntycore orchestration; comet‑flags just plugs into that pipeline.[^7_1]

### 3.1 Evolution path for comet changes

Use exactly the six stages you outlined, with comet‑specific details:[^7_1]


| Stage | Artifact(s) | System | comet‑specific behavior |
| :-- | :-- | :-- | :-- |
| 1. Proposal | `.evolve.jsonl` | Developer / AI chat | Add `EvolutionProposalRecord` with `kind: "FlagProfileUpdate"`, target path (`policies/bostrom-comet-*.aln`), and `effectbounds`.[^7_1][^7_2] |
| 2. Validation | `*.comet-*.aln`, `.rohmodel.aln` | `comet_flags` crate + CI | Schema validation, mental privacy checks, monotone constraints, synthetic RoH simulation for safety‑critical flags.[^7_1][^7_2] |
| 3. Authorization | `.stake.aln` | sovereigntycore | Enforces that only roles with the right scope (`daytodaytuning` vs `archchange`) can approve; EVOLVE required for high‑impact profile changes.[^7_1] |
| 4. Execution | Service runtime | `comet_flags` | On `Allowed`, reloads validated ALN files and exposes new flags through the resolver API.[^7_1][^7_2] |
| 5. Logging | `.donutloop.aln` | post‑merge hook | Append hash‑linked ledger entry tying proposal, effective flags, and policy refs together.[^7_1] |
| 6. Verification | CI post‑merge | CI | Re‑scan logs for RoH, monotone, stake, and neurorights invariants; fail if any breach appears.[^7_1][^7_2] |

This keeps comet‑flags as a citizen of the same donutloop cycle as other evolution crates, not a side channel.[^7_3][^7_1]

## 4. How this preserves your share of control

- Mental privacy and dream‑sensitivity stay anchored in the same neurorights core you already use for BioState.dream, just expressed as “no raw sources in flag definitions” plus policy fields like `dreamstatesensitive` and `forbiddecisionuse` in `.neurorights.json`.[^7_2]
- Cognitive liberty is realized as a guaranteed SAFE_BASELINE / CONSERVATIVE profile present in every repo, with cross‑checked semantics in CI.[^7_3][^7_1]
- Monotone safety envelopes and RoH ≤ 0.3 remain centralized in sovereigntycore and `.rohmodel.aln`; comet‑flags only ever tightens envelopes and refuses to activate risky profiles once RoH approaches the ceiling.[^7_2][^7_1]
- All changes to comet behavior are proposals in `.evolve.jsonl`, evaluated under stake / neurorights / RoH guards, and only become real when hash‑chained into `donutloopledger.aln`, ready for optional `.bchainproof.json` anchoring to Googolswarm / Organicchain.[^7_3][^7_1]

If you’d like, next step I can emit concrete starter contents for:

- `qpudatashards/particles/comet-flags.schema.aln`
- `policies/bostrom-comet-flags-v1.comet-flags.aln`
- `qpudatashards/particles/comet-profile.schema.aln`
- `policies/bostrom-comet-profiles-v1.comet-profile.aln`

tuned to your current neurorights and RoH setup so you can paste them directly into your repo.
<span style="display:none">[^7_4][^7_5][^7_6][^7_7][^7_8]</span>

<div align="center">⁂</div>

[^7_1]: create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md

[^7_2]: legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md

[^7_3]: what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md

[^7_4]: how-can-we-improve-helm-promet-R0sr3xmqRhyDfQzlN7e7sQ.md

[^7_5]: what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md

[^7_6]: cyblog_2026-02-02T22_38_16.272Z.json

[^7_7]: neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md

[^7_8]: quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md


---

# System Guarantees and Cross-Repository Consistency

The ultimate measure of success for the comet-flags initiative lies in its ability to deliver verifiable, system-level guarantees that are consistently upheld across the distributed NeuroPC ecosystem. The research goal explicitly targets three key guarantees: auditability, cross-repository consistency, and safer-only updates . By grounding these guarantees in the paired approach of formal specifications and practical artifacts, the system moves beyond theoretical ideals to become a practical framework for building trustworthy cybernetic technologies. These guarantees are not accidental byproducts but are the intended outcomes of the architectural choices and implementation patterns discussed previously.
Auditability is a fundamental requirement stemming directly from the CHCIL license, which mandates auditable logs for significant changes to control policies and access to cybernetic data . The comet-flags system achieves this through a deliberate, multi-faceted strategy centered on the .donutloop.aln ledger. Every single change to a flag profile originates as an EvolutionProposalRecord in .evolve.jsonl and, upon acceptance, is cryptographically anchored in .donutloop.aln . This creates a transparent and immutable chain of custody. The formal specification for this guarantee is that the system must provide a complete, unforgeable history of all configuration changes. This is embodied in the practical implementation of the hash-linked ledger, which makes tampering with the audit trail computationally infeasible. Furthermore, the CI policy files play a supporting role in auditability. A dedicated CI job can be configured to periodically verify the integrity of the .donutloop.aln chain, checking that each entry's hash correctly points to the previous one. Additionally, the audit trail generated by comet-flags should contain sufficient metadata—in line with the license's guidance—to answer critical questions like "who changed which flags, under which envelopes, and with which risk model version?" . This level of detail transforms the ledger from a simple history log into a powerful forensic and accountability tool.
Cross-repository consistency is another critical guarantee, aimed at ensuring that a concept like the CONSERVATIVE profile behaves identically whether it is used in the NeuroPC repository, the Organic_CPU repository, or Reality.os . Without such consistency, the promise of cognitive liberty—a guaranteed safe-baseline mode—is hollow, as a user could find themselves in a more aggressive configuration in one service than another. The challenge is to maintain semantic agreement across independently developed codebases. The solution lies in the formalization of shared artifacts. The *.comet-profile.aln and *.comet-flags.aln schemas become the single source of truth for these shared concepts. By standardizing these file types and aligning their schemas with the broader NeuroPC ALN handbook, the system establishes a common vocabulary . The practical implementation of this guarantee relies heavily on the CI infrastructure. A cross-cutting CI job can be designed to run across all relevant repositories. This job would perform several checks: it would validate that each repository contains the expected profile definitions (e.g., CONSERVATIVE); it would check that the forbidden flag lists for these profiles are identical across repos; and it could even test that the same label-based selection logic produces the same result for a given context in each repository. This automated, federated testing regime acts as a powerful guardrail, preventing drift and ensuring that the meaning of a flag or profile remains constant throughout the ecosystem. The optional comet-neurofs-index.aln further supports this by helping discovery tools locate these shared artifacts, making the entire system more transparent and manageable .
Finally, the guarantee of safer-only updates is a cornerstone of the project, directly implementing the CHCIL license's directive that updates must not degrade safety . This goes beyond simply preventing regressions in one specific area; it is a holistic commitment to non-maleficence. The system enforces this through a combination of formal invariants and procedural controls. As detailed in the paired deliverables showcase, the *.comet-flags.aln schema can encode mathematical constraints, such as monotonicity, which prevent a proposed update from relaxing any safety limit . The comet_flags Rust crate implements this logic at runtime, refusing to apply any configuration that would violate these invariants. This is reinforced by the procedural control imposed by sovereigntycore and the .stake.aln file. No update, regardless of how "safe" it appears to be, can be applied without being proposed, reviewed, and approved according to the established governance rules . The CI pipeline adds another layer of defense by statically analyzing proposed changes in .evolve.jsonl to catch potential safety regressions before they are ever merged into the main branch. This creates a robust, multi-layered defense-in-depth strategy. A change that is unsafe would be caught by at least one of these layers: the formal invariant check, the runtime validation, the governance rule check, or the CI static analysis. This ensures that the principle of "safer-only" is not a guideline but a mandatory, computationally enforced property of the system. Together, these three guarantees—auditability, consistency, and safer-only updates—form the bedrock of trustworthiness for any system built upon the comet-flags foundation, fulfilling the research goal of formally encoding human rights into verifiable system behavior.
Research Challenges and Future Directions
While the comet-flags architecture presents a compelling vision for bridging legal ethics and technical implementation, several significant challenges and areas for future research remain. Addressing these gaps is crucial for moving from an early-stage, experimental prototype to a robust, production-ready system capable of reliably protecting neurorights in a complex, evolving cybernetic environment. These challenges touch upon the boundaries of actuation, the complexity of system interactions, and the rigidity of formal safety models.
One of the most pressing challenges is clarifying the boundary of actuation. The comet-flags documentation repeatedly emphasizes that the tool is software-only and should avoid direct physical actuation, preferring instead to adjust UI, pacing, and configuration flags . However, the overall research goal mentions "human-subject protections" , and many cybernetic systems will inevitably include modules with actual actuation capabilities (e.g., motor controllers, stimulators). The current design seems to assume that comet-flags only manages software-level configuration, leaving the vetting of actuating modules to higher-level components. This creates a potential gap in the security and safety model. If a flag enables a feature that wraps a dangerous actuator, how does comet-flags ensure that flag is not misused? A naive implementation could lead to a situation where a seemingly benign configuration change inadvertently activates a harmful physical output. Future research must explore a mechanism for extending the sovereignty model to cover not just the flags themselves, but the modules they enable. This could involve a vetting process for actuating modules, where their safety properties and operational envelopes are formally verified and registered in a way that comet-flags and sovereigntycore can understand and enforce. This would require a more expressive ALN schema to describe not just software flags but also the hardware or low-level software interfaces they control.
A second major challenge is the management of complex flag dependencies. The initial design sketches for ALN schemas focus on per-flag invariants, which is a powerful starting point . However, real-world systems often exhibit emergent properties arising from the interaction of multiple features. For example, enabling feature_A might require feature_B to be disabled, or the combination of profile_X and mode_Y might introduce a new, unforeseen risk profile that is not captured by the individual invariants of either component. The current proposed schema does not appear to have a native mechanism for expressing these cross-flag dependencies or synergistic safety risks. Formalizing and implementing dependency checking is a significant next step. This would require moving beyond simple attribute-based validation to a more sophisticated graph-based or rule-based reasoning system. The ALN schema could be extended to include dependencies or conflicts sections, and the comet_flags crate would need a dependency solver to ensure that any proposed active set of flags is conflict-free and satisfies all transitive constraints. This problem is analogous to package manager dependency resolution but is complicated by the fact that the validity of a dependency may depend on dynamic, contextual factors like BioState or RoH levels.
Third, the concept of monotone safety envelopes, while intuitively appealing, may prove too restrictive or brittle for a rapidly learning and evolving system. The principle of "safer-only" updates is powerful, but in practice, an update that seems to relax a safety limit might actually be a strategic move to prevent a greater long-term harm or to enable a more adaptive response to a changing environment . For instance, slightly increasing a pain tolerance threshold might seem like a degradation, but if it allows the system to better manage fatigue and prevent a catastrophic overload, it could be a net positive. The research must investigate whether strict monotonicity is a sufficient or even desirable constraint. One potential direction is to develop a more nuanced, probabilistic model of safety. Instead of requiring all safety parameters to be monotonically non-decreasing, the system could require that the expected long-term risk is proven to be lower after an update. This would involve a more complex proof obligation, potentially using techniques from formal methods or machine learning safety verification. Another approach could be to introduce different classes of updates with varying levels of scrutiny. A "patch" update might be held to a stricter monotonicity rule, while a "major" version update involving a significant redesign could be subjected to a more comprehensive, albeit slower, safety re-certification process overseen by sovereigntycore.
Finally, the practicalities of cross-repo consistency and discovery require more robust solutions. While the initial plan involves standardized naming and CI checks, a more formal mechanism is needed for versioning and propagating these shared artifacts . How does a new version of a comet-profile.aln schema get communicated to all dependent repositories? How is backward compatibility managed? The mention of a .neurofs-index.aln suggests a federated filesystem approach, but its mechanics are not detailed . Future work should focus on developing a protocol for this, potentially inspired by content-addressable storage or decentralized naming systems. This would allow repositories to declare dependencies on specific versions of shared comet-flags artifacts and have the build system automatically fetch the correct, verified versions, eliminating manual coordination and reducing the risk of drift. These challenges highlight that the journey from conceptual architecture to a fully realized, trustworthy system is complex and ongoing, requiring sustained research and development in formal methods, distributed systems, and cybernetic governance.

Auditability, cross-repo consistency, and safer-only updates are all achievable with the shell you already defined; comet-flags just needs to sit on top of the same four canonical artifacts and sovereigntycore guards.[^8_1][^8_2]

## System guarantees: how they’re actually enforced

### Auditability

- Every comet change is a typed `EvolutionProposalRecord` line in `qpudatashards/particles/evolution-proposals.evolve.jsonl`, with `kind: "FlagProfileUpdate"`, `rohbefore`, `rohafter`, and `hexstamp`.[^8_1]
- Only proposals that sovereigntycore marks `decision: "Allowed"` get mirrored into `logs/donutloopledger.aln` as hash-linked `DonutloopEntry` rows with `proposalid`, `rohbefore`, `rohafter`, `policyrefs`, `hexstamp`, `prevhexstamp`.[^8_1]
- Rust bindings in `organiccpualn` (e.g., `donutloopledger.rs`, `evolvestream.rs`) plus `RiskOfHarm` in sovereigntycore give you:
    - Load-time validation of `.rohmodel.aln` invariants (ceiling ≤ 0.3, weights ≥ 0, sum ≤ 1).[^8_1]
    - Runtime checks that `rohafter ≤ 0.3` and `rohafter ≤ rohbefore` for any allowed proposal.[^8_1]
- A CI job scans `.evolve.jsonl` and `donutloopledger.aln` after tests and fails if any entry breaks RoH ceiling, monotonicity, or the hash chain (by recomputing hashes and `prevhexstamp`).[^8_1]

This satisfies the CHCIL requirement for an immutable, queryable history of “who changed what, under which envelopes and RoH model,” because each ledger row includes `policyrefs` and is cryptographically chained.[^8_2][^8_1]

### Cross-repository consistency

- Shared semantics for profiles and flags live in ALN, not code:
    - `qpudatashards/particles/comet-profile.schema.aln` and `comet-flags.schema.aln` define shape and invariants.[^8_2][^8_1]
    - `policies/bostrom-comet-profiles-v1.comet-profile.aln` and `bostrom-comet-flags-v1.comet-flags.aln` encode CONSERVATIVE / SAFE_BASELINE / COPILOT / AUTOEVOLVE once.[^8_2][^8_1]
- Each repo (NeuroPC, Organic_CPU, Reality.os) imports the same policy shard paths (via Git submodule or a NeuroFS index) and exposes identical typed Rust bindings for `Profile` and `FlagDef`.[^8_1]
- A federated CI job (one workflow that runs against all repos) enforces:
    - Presence of canonical profile IDs (e.g., `SAFE_BASELINE`) in each repo.[^8_1]
    - Byte-for-byte equality of forbidden flag lists for those profiles, or equality of a normalized JSON view.[^8_1]
    - That label-based profile resolution logic (same subject context) yields the same profile in each crate, verified via cross-repo tests.[^8_3][^8_1]

The optional `comet-neurofs-index.aln` can list canonical artifact locations and versions; discovery tools then locate and pin those exact files so services don’t silently drift to divergent definitions.[^8_3]

### Safer-only updates

- Formal layer:
    - `.rohmodel.aln` encodes RoH axes, weights, and an enforced global ceiling of 0.3; `RohModel::validate_invariants` rejects any shard that loosens this.[^8_1]
    - `*.comet-flags.aln` carries monotone constraints (e.g., `value >= @previous.value`) for safety parameters; these are treated as hard invariants.[^8_2]
- Runtime layer (Rust):
    - sovereigntycore’s `evaluate_update_with_roh` rejects any proposal with `rohafter > 0.3` or `rohafter > rohbefore + ε`, and any envelope change that loosens Tsafe kernels or pain/lifeforce envelopes.[^8_2][^8_1]
    - The comet_flags crate refuses to apply a flag/profile change if any bound parameter would violate its monotone constraint against stored previous values.[^8_2]
- Governance layer:
    - `.stake.aln` and its Rust bindings require Host + OrganicCPU signatures for lifeforce / archchange scopes; without that multisig, no “looks safe” proposal can even be evaluated.[^8_1]
    - SMART vs EVOLVE tokens further gate “small reversible nudges” vs deep structural changes.[^8_2][^8_1]
- CI layer:
    - Static simulation over `.evolve.jsonl` (effect bounds, monotone inequalities) rejects PRs that would cause a regression if merged.[^8_1]
    - sovereigntycore tests assert that any attempt to raise RoH ceiling, relax envelopes, or bypass stake/neurorights guards fails.[^8_2][^8_1]

Collectively this forms a defense-in-depth implementation of “safer-only”: at least one of schema validation, runtime guards, stake, or CI will block a regressive change.[^8_2][^8_1]

## Research challenges: concrete next steps

### 1. Actuation boundary

- Extend ALN to describe actuators and their envelopes, so flags can be linked to vetted modules:
    - `qpudatashards/particles/actuator-spec.schema.aln`: fields for `module_id`, `actuation_kind`, `max_amplitude`, `integration_depth` (advisor, bounded_auto, forbidden), and neurorights tags.[^8_2]
    - `policies/bostrom-actuators-v1.actuator-spec.aln`: your actual devices/modules, kept extremely conservative.[^8_2]
- sovereigntycore must:
    - Refuse any `FlagProfileUpdate` that would enable a profile referencing an actuator module not present or not approved in `actuator-spec`.[^8_2]
    - Enforce integration depth (no direct affect modulation from comet-flags; UI/config only) based on those tags.[^8_2]

This keeps comet-flags “software-only” while still accounting for the fact that some downstream modules may drive hardware, by treating those modules as separately verified, envelope-bound resources.[^8_2]

### 2. Complex flag dependencies

- Extend the comet schemas with an optional dependency slice:[^8_1][^8_2]

```aln
# in comet-flags.schema.aln
field,requires_all,array[string],optional
field,forbids_any,array[string],optional
field,context_guard,string,optional # e.g. "BioState.cognitiveloadindex < 0.6"
```

- Implement a dependency solver in comet_flags:
    - Builds a constraint graph over proposed active flags.
    - Ensures no forbidden pair is simultaneously active, all required flags are present, and all `context_guard` expressions hold given the current BioState/RoH.[^8_2]
- Add CI tests that construct pathological combinations (e.g., A requires ¬B, B requires A) and assert they are rejected, plus property tests to avoid enabling conflict sets.[^8_1]

This moves you from per-flag invariants to whole-profile constraint solving, similar to a package manager but with neurorights-grade context checks.[^8_1][^8_2]

### 3. Monotone safety envelopes vs adaptability

- Keep strict monotonicity as the default for OTA updates logged in `.evolve.jsonl`, but explicitly introduce a second class of “re-certified major updates”:[^8_3]
    - `kind: "MajorSafetyModelUpdate"` proposals require:
        - Formal evidence attached (pointer to proof artifact or extensive tests).
        - Stronger stake requirements (Host + OrganicCPU + ResearchAgent).
        - Possibly human-in-the-loop review for you only, never 3rd parties.[^8_3]
- For these, relax the local monotonicity rule but enforce a stronger global check:
    - The RoH model shard and Tsafe kernel must be proved to yield non-increasing expected long-term RoH over a disturbance set, encoded either as:
        - A machine-checkable summary (bounds checked by CI), or
        - A structured proof sketch referenced from the proposal and treated as a gating requirement.[^8_3]

You can prototype this using more conservative statistical/robustness tests in CI first, then upgrade to theorem-prover hooks later.[^8_3]

### 4. Cross-repo versioning and discovery

- Make `.neurofs-index.aln` real as a federated index of canonical artifacts:[^8_3]

```aln
# neurofs-index.aln
artifact,comet-profiles,version,1.0.0,path,policies/bostrom-comet-profiles-v1.comet-profile.aln,hash,0x...
artifact,comet-flags,version,1.0.0,path,policies/bostrom-comet-flags-v1.comet-flags.aln,hash,0x...
artifact,rohmodel,version,1.0.0,path,qpudatashards/particles/bostrom-rohmodel-v1.rohmodel.aln,hash,0x...
```

- Each repo declares, in a small `neurofs-deps.aln`, which artifact IDs + versions it depends on; CI refuses to build if:
    - Local copies don’t match the indexed hash.
    - A required artifact is missing or on an incompatible major version.[^8_3][^8_1]

This gives you content-addressable, versioned profiles and flags, so “CONSISTENT CONSERVATIVE profile semantics everywhere” becomes a property of the build system, not just a convention.[^8_3][^8_1]
<span style="display:none">[^8_4][^8_5][^8_6][^8_7][^8_8]</span>

<div align="center">⁂</div>

[^8_1]: create-a-heavy-research-plan-f-iuQRhxq3SXKEqzqKASlSog.md

[^8_2]: legal-terms-dream-states-are-n-kleITgSCSZe1GN7bozIpuQ.md

[^8_3]: what-can-cybernano-teach-me-to-lh1IcgziRyyyUIy8hCIhLQ.md

[^8_4]: how-can-we-improve-helm-promet-R0sr3xmqRhyDfQzlN7e7sQ.md

[^8_5]: what-data-can-be-created-from-WWM8dUv4RIG__zNVQn6V2g.md

[^8_6]: cyblog_2026-02-02T22_38_16.272Z.json

[^8_7]: neuropcs-rules-and-goals-are-c-bJITjTqfQHaJgTu_2pFVnw.md

[^8_8]: quantum-learning-circuits-cons-DnBUJFoeTSWzYvrkU0EnYw.md

