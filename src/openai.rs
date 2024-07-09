use anyhow::{anyhow, Result};
use reqwest::Response;
use serde::Deserialize;
use serde_json::json;

#[derive(Clone)]
pub struct OpenAiClient {
    api_key: String,
    endpoint: String,
    client: reqwest::Client,
}

impl OpenAiClient {
    pub fn new(api_key: String, endpoint: String) -> Self {
        Self {
            api_key,
            endpoint,
            client: reqwest::Client::new(),
        }
    }

    pub async fn create_chat(&self, prompt: String) -> Result<Response> {
        self.client
            .post(self.endpoint.as_str())
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
              "model": "gpt-3.5-turbo-1106",
              "messages": [
                {
                  "role": "system",
                  "content": "You are a helpful assistant."
                },
                {
                  "role": "user",
                  "content": prompt
                }
              ],
              "stream": true,
            }))
            .send()
            .await
            .map_err(|e| anyhow!(e))?
            .error_for_status()
            .map_err(|e| anyhow!(e))
    }
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub choices: Vec<ChatCompletionChunkChoice>,
    pub created: i64,
    pub model: String,
    pub server_tier: String,
    pub system_fingerprint: String,
    pub object: String,
    pub usage: ChatCompletionChunkUsage,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletionChunkChoice {
    pub delta: ChatCompletionChunkDelta,
    pub logprobs: ChatCompletionChunkLogprobs,
    pub finish_reason: Option<String>,
    pub index: i64,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletionChunkLogprobs {
    pub content: Vec<ChatCompletionChunkLogprobsContent>,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletionChunkLogprobsContent {
    pub token: String,
    pub logprob: f64,
    pub bytes: Vec<u8>,
    pub top_logprobs: Vec<ChatCompletionChunkLogprobsContent>,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletionChunkDelta {
    pub content: String,
    pub tool_calls: Vec<ChatCompletionChunkToolCall>,
    pub role: String,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletionChunkToolCall {
    index: i64,
    id: String,
    #[serde(rename = "type")]
    my_type: String,
    function: ChatCompletionChunkFunction,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletionChunkFunction {
    name: String,
    arguments: String,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletionChunkUsage {
    completion_tokens: i64,
    prompt_tokens: i64,
    total_tokens: i64,
}
