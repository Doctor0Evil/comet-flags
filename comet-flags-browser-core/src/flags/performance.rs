use super::{Flag, FlagCategory, FlagRegistry};

pub fn register_performance_flags(registry: &mut FlagRegistry) {
    registry.register(Flag::new(
        "#ignore-gpu-blocklist",
        "Override software rendering list",
        "Overrides the built-in software rendering list and enables GPU-acceleration on unsupported system configurations.",
        FlagCategory::Performance,
    ));

    registry.register(Flag::new(
        "#disable-accelerated-2d-canvas",
        "Accelerated 2D canvas",
        "Enables the use of the GPU to perform 2D canvas rendering instead of using software rendering.",
        FlagCategory::Performance,
    ));

    registry.register(Flag::new(
        "#enable-gpu-rasterization",
        "GPU rasterization",
        "Use GPU to rasterize web content.",
        FlagCategory::Performance,
    ));
}
