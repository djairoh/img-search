use serde::{Deserialize, Serialize};

/// This struct is used for the config variables.
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// The Saucenao api key to use
    pub api_key: String,
    /// how many results to return at maximum
    pub num_results: u32,
    /// the minimum similarity of returned results
    pub min_similarity: u8,
    /// level of logging to use
    pub rust_log: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api_key: "".to_string(),
            num_results: 15,
            min_similarity: 50,
            rust_log: "info".to_string(),
        }
    }
}
