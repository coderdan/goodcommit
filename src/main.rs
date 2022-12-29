use dialoguer::{theme::ColorfulTheme, Select};
use edit::edit;
use git_busy::{get_commit_messages, spawn_cmd};
use std::env;
use std::env::args;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let is_nocolor = env::var("NO_COLOR").unwrap_or_else(|_| String::from("unset")) != String::from("unset");
	let diff = spawn_cmd("git", &["diff".to_string(), "--staged".to_string()]);

	if diff.len() == 0 {
		let flags: Vec<String> = if is_nocolor {
			vec!["status".into()]
		} else {
			vec!["-c".into(), "color.status=always".into(), "status".into()]
		};

		let output = spawn_cmd("git", &flags);
		println!("{}", output);
		std::process::exit(exitcode::NOINPUT);
	}

	let api_key = "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";
	let args = args().collect::<Vec<String>>();
	let mut git_args: Vec<String> = vec!["git".to_string(), "commit".to_string()]; // TODO: remove git

	if !args.contains(&String::from("-m")) && !args.contains(&String::from("--message")) {
		let prompt =
			format!("This is a diff from a recent change in my code. Write a commit message for this diff.\n{}", diff);
		let client = reqwest::Client::new();
		let answer = get_commit_messages(&client, api_key, "text-davinci-003", &prompt, 0.8, 200, 3).await?;

		let items: Vec<String> = answer
			.choices
			.iter()
			.map(|choice| choice.text.trim().to_string().replace("\"", "`").replace("Commit: ", ""))
			.collect();
		let selection = Select::with_theme(&ColorfulTheme::default())
			.with_prompt("Which commit message do you prefer?")
			.items(&items)
			.default(0)
			.report(false)
			.max_length(50)
			.clear(true)
			.interact()?;

		let msg = edit(items[selection].replace('"', "\\\""))?;
		git_args.push("--message".into());
		git_args.push(format!("\"{}\"", msg));
	}

	// passing through all flags to git commit
	for item in args.iter().skip(1) {
		git_args.push(item.to_string());
	}
	let output = spawn_cmd("echo", &git_args); // TODO: replace echo with git

	println!("\n{}", output);
	Ok(())
}
