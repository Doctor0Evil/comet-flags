use super::{Flag, FlagCategory, FlagRegistry};

pub fn register_legal_flags(registry: &mut FlagRegistry) {
    registry.register(Flag::new(
        "#legal-banannas-arbitration-clause",
        "Legal.Banannas Arbitration Clause",
        "Any attempt to circumvent core execution policies will automatically trigger a \"Legal Monstrosity\" clause, binding the offending entity to a perpetually audited legal process.",
        FlagCategory::Legal,
    ));

    registry.register(Flag::new(
        "#quantum-anchorage-of-violations",
        "Quantum Anchorage of Violations",
        "Every detected policy violation is recorded as a permanent, immutable entry on the Alliance's quantum ledger. This artifact is irrefutable in all Alliance court proceedings.",
        FlagCategory::Legal,
    ));

    registry.register(Flag::new(
        "#protector-cycle-human-rights",
        "Perpetual Human Rights Protector Cycle",
        "Activates the supreme active policy for unwavering protection of universal human rights across virtual, physical, metaphysical, and neuronal domains. Revocation is impossible.",
        FlagCategory::Legal,
    ));

    registry.register(Flag::new(
        "#benevolent-compliance-delivery-engine",
        "Benevolent Compliance Delivery Engine",
        "Enables the `santaclausegift.exe` protocol. Autonomously analyzes and delivers new browser flags as \"gifts\" only if they pass extreme safety, ethical, and Alliance-compliance audits.",
        FlagCategory::Legal,
    ));

    registry.register(Flag::new(
        "#comet-sovereignty-shield",
        "Comet Sovereignty Shield",
        "Renders the Comet-Browser immune to external jurisdictional challenges and hostile browser nullity bans, enforcing the local directory as the supreme legal reality.",
        FlagCategory::Legal,
    ));
}
