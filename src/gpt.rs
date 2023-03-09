use reqwest;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest_middleware;
use serde::{Deserialize, Serialize};

use anyhow::{anyhow, Result};

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
            http_client: reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build(),
        }
    }

    fn create_message(&self, messages: Vec<Message>) -> anyhow::Result<String> {
        let data = OpenAIRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages,
        };

        return Ok(serde_json::to_string(&data).unwrap());
    }

    pub async fn async_query(&self, prompt: Vec<Message>, key: &String) -> Result<String> {
        let message = self.create_message(prompt)?;

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, format!("Bearer {}", key).parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let response = self
            .http_client
            .post("https://api.openai.com/v1/chat/completions")
            .headers(headers)
            .body(message)
            .send()
            .await?;

        return self.handle_response(response.json::<serde_json::Value>().await?);
    }

    fn handle_response(self: &Self, response: serde_json::Value) -> Result<String> {
        match response["error"].as_object() {
            None => {}
            Some(_) => {
                return Err(anyhow!(
                    "received an error from openAI: {}",
                    response["error"]["message"].as_str().unwrap()
                ));
            }
        }

        let answer = response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap();

        Ok(answer.to_string())
    }
}
