pub mod legal;
pub mod performance;
pub mod security;
pub mod webapi;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FlagCategory {
    Legal,
    Performance,
    Security,
    WebApi,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Flag {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: FlagCategory,
    pub enabled: bool,
}

impl Flag {
    pub fn new<S: Into<String>>(
        id: S,
        title: S,
        description: S,
        category: FlagCategory,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
            category,
            enabled: false,
        }
    }
}

#[derive(Default)]
pub struct FlagRegistry {
    flags: Vec<Flag>,
}

impl FlagRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, flag: Flag) {
        self.flags.push(flag);
    }

    pub fn register_defaults(&mut self) {
        legal::register_legal_flags(self);
        performance::register_performance_flags(self);
        security::register_security_flags(self);
        webapi::register_webapi_flags(self);
    }

    pub fn all(&self) -> &[Flag] {
        &self.flags
    }

    pub fn all_mut(&mut self) -> &mut [Flag] {
        &mut self.flags
    }

    pub fn find_mut(&mut self, id: &str) -> Option<&mut Flag> {
        self.flags.iter_mut().find(|f| f.id == id)
    }
}
