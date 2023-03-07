use reqwest;
use reqwest_middleware;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap};
use serde::{Deserialize, Serialize};

use anyhow::{Result, anyhow};

pub struct Client {
    http_client: reqwest_middleware::ClientWithMiddleware,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct OpenAIRequest {
    #[serde(rename = "model")]
    model: String,
    #[serde(rename = "messages")]
    messages: Vec<Message>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            http_client: reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
                // .with(http_cache_reqwest::Cache(http_cache_reqwest::HttpCache {
                //     mode: http_cache_reqwest::CacheMode::Default,
                //     manager: http_cache_reqwest::CACacheManager::default(),
                //     options: None,
                // }))
                .build(),
        }
    }

    fn create_message(&self, messages: Vec<Message>) -> anyhow::Result<String> {
        // let mut messages:Vec<Message> = vec![];
        // messages.push(Message {
        //     role: "user".to_string(),
        //     content: prompt.clone(),
        // });

        let data = OpenAIRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages,
        };

        return Ok(serde_json::to_string(&data).unwrap());
    }

    pub async fn async_query(
        &self,
        prompt: Vec<Message>,
        key: &String,
    ) -> Result<String> {
        let message = self.create_message(prompt)?;

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, format!("Bearer {}", key).parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let response = self.http_client
            .post("https://api.openai.com/v1/chat/completions")
            .headers(headers)
            .body(message)
            .send()
            .await?;

        return self.handle_response(response.json::<serde_json::Value>().await?);
    }

    fn handle_response(self: &Self, response: serde_json::Value) -> Result<String> {
        match response["error"].as_object() {
            None => {},
            Some(_) => {
                return Err(anyhow!("received an error from openAI: {}", response["error"]["message"].as_str().unwrap()));
            }
        }

        let answer = response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap();

        Ok(answer.to_string())
    }
}