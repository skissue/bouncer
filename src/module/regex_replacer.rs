use regex::Regex;

use super::Module;

pub struct RegexReplacerModule {
    rules: Vec<(Regex, String, String)>, // (pattern, replacement, description)
}

impl RegexReplacerModule {
    pub fn new() -> Self {
        let rules = vec![(
            Regex::new(r"^(https?://)(?:x\.com|twitter\.com)(/.*)?$").unwrap(),
            "${1}xcancel.com${2}".to_string(),
            "Redirect to xcancel.com".to_string(),
        )];
        Self { rules }
    }
}

impl Module for RegexReplacerModule {
    fn name(&self) -> &str {
        "Regex Replacer"
    }

    fn evaluate(&self, url: &str) -> Option<String> {
        let descriptions: Vec<&str> = self
            .rules
            .iter()
            .filter(|(pattern, replacement, _)| {
                let result = pattern.replace(url, replacement.as_str());
                result != url
            })
            .map(|(_, _, desc)| desc.as_str())
            .collect();

        if descriptions.is_empty() {
            None
        } else {
            Some(descriptions.join(", "))
        }
    }

    fn transform(&self, url: &str) -> Result<String, String> {
        let mut result = url.to_string();
        for (pattern, replacement, _) in &self.rules {
            result = pattern.replace(&result, replacement.as_str()).to_string();
        }
        Ok(result)
    }
}
