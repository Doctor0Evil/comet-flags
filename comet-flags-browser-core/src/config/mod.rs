use crate::flags::FlagRegistry;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct CometConfig {
    pub enabled_flags: Vec<String>,
}

impl CometConfig {
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        let path_ref = path.as_ref();
        if path_ref.exists() {
            match fs::read_to_string(path_ref) {
                Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Self::default()),
                Err(_) => Self::default(),
            }
        } else {
            Self::default()
        }
    }

    pub fn sync_from_registry(&mut self, registry: &FlagRegistry) {
        self.enabled_flags = registry
            .all()
            .iter()
            .filter(|f| f.enabled)
            .map(|f| f.id.clone())
            .collect();
    }

    pub fn apply_to_registry(&self, registry: &mut FlagRegistry) {
        for flag in registry.all_mut().iter_mut() {
            flag.enabled = self.enabled_flags.contains(&flag.id);
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let data = serde_json::to_string_pretty(self)
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        fs::write(path, data)
    }
}

impl Default for CometConfig {
    fn default() -> Self {
        Self {
            enabled_flags: Vec::new(),
        }
    }
}
