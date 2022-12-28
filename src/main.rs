use dialoguer::{theme::ColorfulTheme, Select};
use git_busy::{get_commit_messages, spawn_cmd};
use std::env::args;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let diff = spawn_cmd("git", &["diff".to_string(), "--staged".to_string()]);
    let prompt = format!("This is a diff from a recent change of a library. Explain what was changed in two short sentences.\n{}", diff);
    let api_key = "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";
    let args = args().collect::<Vec<String>>();
    let mut new_args: Vec<String> = vec!["git".to_string(), "commit".to_string()]; // TODO: remove git
    let client = reqwest::Client::new();

    if !args.contains(&String::from("-m")) && !args.contains(&String::from("--message")) {
        let answer =
            get_commit_messages(&client, api_key, "text-davinci-003", &prompt, 0.8, 200, 3).await?;

        let items: Vec<String> = answer
            .choices
            .iter()
            .map(|choice| choice.text.trim().to_string())
            .collect();
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which commit message do you prefer?")
            .items(&items)
            .default(0)
            .report(false)
            .clear(true)
            .interact()?;

        new_args.push("--message".into());
        new_args.push(format!("\"{}\"", items[selection].replace('"', "\\\"")));
    }

    // passing through all flags to git commit
    for item in args.iter().skip(1) {
        new_args.push(item.to_string());
    }
    let output = spawn_cmd("echo", &new_args); // TODO: replace echo with git

    println!("\n{}", output);
    Ok(())
}
