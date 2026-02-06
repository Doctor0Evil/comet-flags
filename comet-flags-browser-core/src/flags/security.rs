use super::{Flag, FlagCategory, FlagRegistry};

pub fn register_security_flags(registry: &mut FlagRegistry) {
    registry.register(Flag::new(
        "#site-isolation-trial-opt-out",
        "Disable site isolation",
        "Disables site isolation features. Caution: this disables important mitigations for the Spectre CPU vulnerability.",
        FlagCategory::Security,
    ));

    registry.register(Flag::new(
        "#tracking-protection-3pcd",
        "Tracking Protection for 3PCD",
        "Enables the tracking protection UI and preferences for the third-party cookie phaseout.",
        FlagCategory::Security,
    ));

    registry.register(Flag::new(
        "#enterprise-file-obfuscation",
        "Enterprise File Obfuscation",
        "Enables temporary file obfuscation during download for enterprise users, preventing access before security verification is complete.",
        FlagCategory::Security,
    ));
}
