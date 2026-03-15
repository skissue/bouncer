use serde::Deserialize;
use std::path::PathBuf;

const DEFAULT_ENABLED_MODULES: &[&str] =
    &["https", "tracking_cleaner", "unshorten", "regex_replacer"];

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    pub enabled_modules: Vec<String>,
    pub regex_replacer: RegexReplacerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled_modules: DEFAULT_ENABLED_MODULES
                .iter()
                .map(|s| s.to_string())
                .collect(),
            regex_replacer: RegexReplacerConfig::default(),
        }
    }
}

#[derive(Deserialize, Default)]
pub struct RegexReplacerConfig {
    #[serde(default)]
    pub rules: Vec<RegexRule>,
}

#[derive(Deserialize)]
pub struct RegexRule {
    pub pattern: String,
    pub replacement: String,
    pub description: String,
}

impl Config {
    pub fn load() -> Self {
        let path = config_path();
        let contents = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => return Config::default(),
        };
        match toml::from_str(&contents) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Warning: failed to parse config at {}: {}", path.display(), e);
                Config::default()
            }
        }
    }
}

fn config_path() -> PathBuf {
    let config_dir = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").expect("HOME not set");
            PathBuf::from(home).join(".config")
        });
    config_dir.join("bouncer").join("config.toml")
}
