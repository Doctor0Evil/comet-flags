use super::{Flag, FlagCategory, FlagRegistry};

pub fn register_webapi_flags(registry: &mut FlagRegistry) {
    registry.register(Flag::new(
        "#enable-javascript-harmony",
        "Experimental JavaScript",
        "Enable web pages to use experimental JavaScript features.",
        FlagCategory::WebApi,
    ));

    registry.register(Flag::new(
        "#prompt-api-for-gemini-nano",
        "Prompt API for Gemini Nano",
        "Enables the exploratory Prompt API, allowing natural language instructions to a built-in large language model (Gemini Nano).",
        FlagCategory::WebApi,
    ));

    registry.register(Flag::new(
        "#web-machine-learning-neural-network",
        "Enables WebNN API",
        "Enables the Web Machine Learning Neural Network (WebNN) API.",
        FlagCategory::WebApi,
    ));
}
