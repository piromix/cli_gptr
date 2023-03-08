use crate::file::read_config;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub openai_api_key: String,
    pub translate_source: String,
    pub translate_target: String,
    pub deepl_api_key: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        let config = read_config("gptr.json".to_string());
        if config.is_some() {
            let config = config.unwrap();
            let config: serde_json::Value = serde_json::from_str(&config).unwrap();
            let openai_api_key = config["openai_api_key"].as_str().unwrap().to_string();
            let translate_source = config["translate"]["source"].as_str().unwrap().to_string();
            let translate_target = config["translate"]["target"].as_str().unwrap().to_string();
            let deepl_api_key = config["deepl_api_key"]
                .as_str()
                .and_then(|s| Some(s.to_string()))
                .or_else(|| None);
            return Self {
                openai_api_key,
                translate_source,
                translate_target,
                deepl_api_key,
            };
        } else {
            Self {
                openai_api_key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"),
                translate_source: "ja".to_string(),
                translate_target: "en".to_string(),
                deepl_api_key: None,
            }
        }
    }
}
