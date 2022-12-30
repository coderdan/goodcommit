use dialoguer::{theme::ColorfulTheme, Select};
use edit::edit;
use git_busy::{check_diff_get_error, get_api_key, get_commit_messages, spawn_cmd};
use spinners::{Spinner, Spinners};
use std::env::args;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let diff = spawn_cmd("git", &["diff".to_string(), "--staged".to_string()]);

	// check if there is a diff at all
	let diff_error = check_diff_get_error(&diff);
	if !diff_error.is_empty() {
		println!("{}", diff_error);
		std::process::exit(exitcode::NOINPUT);
	}

	let api_key = get_api_key();
	if api_key.is_empty() {
		println!("Please set the GPT_API_KEY environment variable.\nGo to https://beta.openai.com/account/api-keys to create an account and the required key.");
		std::process::exit(exitcode::NOINPUT);
	}

	let args = args().collect::<Vec<String>>();
	let mut git_args: Vec<String> = vec!["commit".to_string()]; // all args to be passed on to `git commit`

	if !args.contains(&String::from("-m")) && !args.contains(&String::from("--message")) {
		let prompt =
			format!("This is a diff from a recent change in my code. Write a commit message for this diff.\n{}", diff);
		if prompt.len() > 15026 {
			println!("The diff is too large for this tool.\nStage less files or commit with \"git commit\" directly.");
			std::process::exit(exitcode::USAGE);
		}
		let mut sp = Spinner::new(Spinners::Dots, "Asking GPT3 to write the commit message".into());
		let client = reqwest::Client::new();
		let answer = get_commit_messages(&client, &api_key, "text-davinci-003", &prompt, 0.8, 200, 3).await?;
		sp.stop();
		print!("\x1b[2K\n\x1b[2F\n"); // to clear the spinner and erase the line

		let items: Vec<String> = answer
			.choices
			.iter()
			.map(|choice| {
				choice.text.replace('"', "`").replace("Commit:", "").replace("Commit message:", "").trim().to_string()
			})
			.collect();
		let selection = Select::with_theme(&ColorfulTheme::default())
			.with_prompt("Which commit message do you prefer?")
			.items(&items)
			.default(0)
			.report(false)
			.max_length(50)
			.clear(true)
			.interact()?;

		// open your default editor as git would do
		let msg = edit(items[selection].replace('"', "\\\""))?;
		git_args.push("--message".into());
		git_args.push(msg.trim().to_string());
	}

	// we ignore the first argument as that's the path to the binary
	for item in args.iter().skip(1) {
		git_args.push(item.to_string());
	}
	let output = spawn_cmd("git", &git_args);

	println!("\n{}", output);
	Ok(())
}
