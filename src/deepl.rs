use deepl_api::*;
use std::env;


pub struct Deepl {
    pub deepl: DeepL,
}

impl Deepl {
    pub fn new(key: Option<String>) -> Self {
        return match key {
            Some(key) => {
                Self {
                    deepl: DeepL::new(key),
                }
            },
            None => {
                Self {
                    deepl: DeepL::new(env::var("DEEPL_API_KEY").unwrap()),
                }
            }
        }
    }

    pub async fn translate(self: &Self, text: &String, source: Option<String>, target: String) -> Result<String> {

        let texts = TranslatableTextList {
            source_language: source,
            target_language: target,
            texts: vec!(text.clone()),
        };

        let translated = self.deepl.translate(None, texts).await.unwrap();

        let mut result: String = String::new();
        translated.iter().for_each(|t| {
            result.push_str(&t.text); result.push_str("\n");
        });

        return Ok(result);
    }
}
