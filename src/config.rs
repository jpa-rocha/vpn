use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub vpn: String,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            vpn: "wobcom".to_string(),
        }
    }
}
