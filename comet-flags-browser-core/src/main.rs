mod flags;
mod config;
mod ui;

use crate::config::CometConfig;
use crate::flags::FlagRegistry;
use crate::ui::run_cli;

fn main() {
    let mut registry = FlagRegistry::new();
    registry.register_defaults();

    let mut config = CometConfig::load_or_default("comet-flags.json");
    run_cli(&mut registry, &mut config);
    config.save("comet-flags.json").expect("failed to save config");
}
