use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize)]
struct GenerateTextRequest<'a> {
	model: &'a str,
	prompt: &'a str,
	temperature: f32,
	max_tokens: usize,
	n: usize,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
	pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct GenerateTextResponse {
	pub choices: Vec<Choice>,
}

pub async fn get_commit_messages<'a>(
	client: &Client,
	api_key: &str,
	model: &'a str,
	prompt: &'a str,
	temperature: f32,
	max_tokens: usize,
	n: usize,
) -> Result<GenerateTextResponse, Error> {
	let request_body = GenerateTextRequest {
		model,
		prompt,
		temperature,
		max_tokens,
		n,
	};

	client
		.post("https://api.openai.com/v1/completions")
		.header("Authorization", format!("Bearer {}", api_key))
		.json(&request_body)
		.send()
		.await?
		.json()
		.await
}

pub fn spawn_cmd(cmd: &str, args: &[String]) -> String {
	let output = Command::new(cmd).args(args).output().expect("Failed to execute process");

	String::from_utf8(output.stdout).expect("")
}

#[test]
fn test_spawn_cmd() {
	let output = spawn_cmd("echo", &["test".to_string()]);
	assert_eq!(output.trim(), String::from("test"));
}
