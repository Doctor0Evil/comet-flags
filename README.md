# comet-flags

`comet-flags` is a small, sovereign-friendly feature-flag and profile-description toolkit designed for evolving distributed systems without forking Docker images, Helm charts, or entire repositories.[file:4][file:7] It fits into the wider OrganicCPU / NeuroPC ecosystem as a **declarative switchboard** for turning optional capabilities on or off across services, languages, and environments, while keeping neurorights and autonomy constraints enforceable at the config layer.[file:4][file:5][file:7]

The goal is to make it easy to say “these features, in this context, for this subject” as data, not ad‑hoc code.

---

## Status

This repository is early-stage and experimental. Expect breaking changes while the flag grammar, ALN schemas, and Rust bindings are still being aligned with the NeuroPC sovereign-kernel specs (.evolve.jsonl, .rohmodel.aln, .stake.aln, .donutloop.aln, .neurofs-index.aln).[file:4][file:7]

---

## Core ideas

`comet-flags` is intended to support:

- **Profile‑driven feature bundles**  
  Describe feature sets per subject, environment, or deployment mode (e.g. `baseline`, `copilot`, `autoevolve`) as data, not code-conditionals.[file:4][file:6]

- **Hard, label‑based selection**  
  Use multi-axis labels (mode, domain, risk level, language, region) to select flags deterministically, suitable for CI and sovereignty-core checks.[file:4][file:6]

- **Language-agnostic configs**  
  Keep the flag grammar in neutral formats (ALN/JSON/NDJSON), so Rust, Java, Kotlin, and Mojo services can all consume the same flag sets.[file:4][file:7]

- **Sovereign envelopes first**  
  Flags can express “allowed only under SMART token”, “requires EVOLVE”, or “disallowed when RoH ≥ 0.3”, matching the existing neurorights and Tsafe patterns.[file:4][file:5]

- **AI‑chat compatible wiring**  
  AI tools are expected to propose flag/profile changes as `.evolve.jsonl` entries; sovereigntycore decides what actually becomes active.[file:4][file:7]

---

## How comet-flags fits into the stack

`comet-flags` is designed to sit alongside existing NeuroPC / OrganicCPU filetypes, not replace them:[file:4][file:5][file:6]

- `.evolve.jsonl` – evolution proposals (including “change flag profile X for subject Y”).  
- `.rohmodel.aln` – risk-of-harm model that gates which flag profiles are allowed at all.  
- `.stake.aln` – who can change which flag bundles (Host, OrganicCPU, ResearchAgent).  
- `.donutloop.aln` – hash-linked ledger of accepted changes, including flag flips.  
- `.neurofs-index.aln` – index that tells tools where the live comet-flag configs reside.

A typical workflow:

1. A service or AI-chat client asks: “Which profile is active for subject X in mode `COPILOT`?”  
2. `comet-flags` resolves the active profile from ALN/JSON shards and labels.  
3. Sovereigntycore checks RoH, stake, neurorights; if safe, the resulting flag set is exposed as read-only inputs to services.  
4. Any change to a profile is expressed as an `EvolutionProposalRecord` in `.evolve.jsonl` and, if accepted, logged into `.donutloop.aln`.[file:4][file:6]

---

## Planned filetypes

The repository is expected to standardize a small set of comet-specific filetypes, all compatible with existing ALN/NDJSON patterns:[file:4][file:6][file:7]

- `*.comet-profile.aln`  
  Declarative profiles: named bundles of flags with labels (mode, domain, risk band, language, jurisdiction).

- `*.comet-flags.aln`  
  Canonical flag catalog: each flag has an ID, description, default, and invariants (e.g., “never true if RoH ≥ 0.3”).  

- `comet-profiles.evolve.jsonl`  
  Evolution stream of profile changes, suitable for sovereigntycore guards and donutloop logging.

- `comet-neurofs-index.aln`  
  Optional index entry referencing where comet-related shards live, for AI-chat and CI discovery (integrated into the global `.neurofs-index.aln`).

Exact schemas will track the broader NeuroPC ALN handbook and sovereign-kernel spec.[file:4][file:6]

---

## Intended use-cases

While the code and schemas are still forming, the design targets:

- **Service meshes & wasmCloud / wadm‑style orchestration**  
  Use profiles to gate optional services, logging, or experimental modules per subject or environment, without cloning charts or manifests.[file:4][file:6]

- **NeuroPC / OrganicCPU modes**  
  Switch between `CONSERVATIVE`, `COPILOT`, and `AUTOEVOLVE` behavior by flipping profiles, with neurorights and RoH invariants checked at the boundary.[file:4][file:5][file:7]

- **AI-assisted configuration**  
  Allow AI-chat tools to draft changes (“enable X for language-cowriter in sandbox only”), while sovereigntycore and stake rules decide what is actually accepted.[file:4][file:7]

- **Cross‑repo consistency**  
  Keep the same comet profile names and semantics across `NeuroPC`, `Organic_CPU`, `Reality.os`, and related repos, so flags behave identically everywhere.[file:4][file:6][file:7]

---

## Roadmap (high-level)

Planned milestones (subject to change):[file:4][file:6][file:7]

1. **Schema sketches**  
   - Draft `*.comet-flags.aln` and `*.comet-profile.aln` schemas aligned with the ALN handbook.  
   - Add entries into `.neurofs-index.aln` so tools can discover these shards.

2. **Rust bindings**  
   - Provide a `comet_flags` Rust crate that loads/validates ALN profiles and exposes typed flag-sets for services.  
   - Wire invariant checks (RoH ≤ 0.3, neurorights constraints, stake gating) via existing `organiccpualn` and `sovereigntycore` crates.

3. **Sovereign CI integration**  
   - GitHub Actions that fail if a profile loosens envelopes, violates neurorights, or bypasses stake multisig.  
   - Tests to ensure every accepted evolution step is logged into `.donutloop.aln`.

4. **AI-chat interfaces**  
   - NDJSON/JSON contracts so AI tools can propose profile changes and query effective flags safely, without direct access to raw biophysical metrics.

---

## Contributing

This repository follows the same neurorights and CHCIL principles used across the NeuroPC / OrganicCPU ecosystem:[file:5][file:7]

- No designs that exclude or downgrade augmented citizens.  
- No direct actuation; flags remain software-only controls.  
- All structural changes should be representable as `.evolve.jsonl` proposals and pass RoH and neurorights guards.

Issues and pull requests are welcome once the initial schemas and crate layout land. When in doubt, prefer:

- Declarative data over ad-hoc code conditionals.  
- Monotone tightening of safety envelopes over any form of loosening.  
- Rust + ALN-first implementations, with Java/Kotlin/Mojo bindings where needed.[file:4][file:7]

---

## License

The intended license is aligned with the Cybernetic Human–Computer Interface License (CHCIL) used elsewhere in the NeuroPC ecosystem, prioritizing neurorights and augmented-citizen autonomy.[file:7]  
See the repository’s `LICENSE` file once published for the authoritative terms.

---

_neuropc-tag 0xNP09_
