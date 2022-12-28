use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct GenerateTextRequest {
    model: &'static str,
    prompt: &'static str,
    temperature: f32,
    max_tokens: usize,
}

#[derive(Deserialize, Debug)]
struct Choice {
    text: String,
}

#[derive(Deserialize, Debug)]
struct GenerateTextResponse {
    //id: String,
    model: String,
    //prompt: String,
    //temperature: f32,
    //max_tokens: usize,
    choices: Vec<Choice>
}

pub async fn generate_text(
    client: &Client,
    api_key: &str,
    model: &'static str,
    prompt: &'static str,
    temperature: f32,
    max_tokens: usize,
) -> Result<String, Error> {
    let request_body = GenerateTextRequest {
        model,
        prompt,
        temperature,
        max_tokens,
    };

    let response: GenerateTextResponse = client
        .post("https://api.openai.com/v1/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?
        .json()
        .await?;

    Ok(response.choices[0].text.to_string())
}

