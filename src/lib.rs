use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::env;
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

pub fn check_diff_get_error(diff: &str) -> String {
	if diff.is_empty() {
		let is_nocolor = env::var("NO_COLOR").unwrap_or_else(|_| String::from("unset")) != *"unset";
		let flags: Vec<String> = if is_nocolor {
			vec!["status".into()]
		} else {
			vec!["-c".into(), "color.status=always".into(), "status".into()]
		};

		spawn_cmd("git", &flags)
	} else {
		String::from("")
	}
}

#[test]
fn test_check_diff_get_error() {
	assert!(check_diff_get_error("").len() > 0);
	assert!(check_diff_get_error(".").len() == 0);
}

pub fn get_api_key() -> String {
	env::var("GPT_API_KEY").unwrap_or_else(|_| String::from(""))
}

#[cfg(test)]
mod lib {
	extern crate temp_env;
	use super::*;

	#[test]
	fn test_get_api_key_with_env_var() {
		temp_env::with_var_unset("GPT_API_KEY", || assert!(get_api_key().is_empty()));
	}

	#[test]
	fn test_get_api_key_without_env_var() {
		temp_env::with_var("GPT_API_KEY", Some("somekey"), || assert!(!get_api_key().is_empty()));
	}
}
