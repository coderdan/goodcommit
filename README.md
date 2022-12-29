```sh
   ██████╗  ██╗ ████████╗     ██████╗  ██╗   ██╗ ███████╗ ██╗   ██╗
  ██╔════╝  ██║ ╚══██╔══╝     ██╔══██╗ ██║   ██║ ██╔════╝ ╚██╗ ██╔╝
  ██║  ███╗ ██║    ██║        ██████╔╝ ██║   ██║ ███████╗  ╚████╔╝
  ██║   ██║ ██║    ██║        ██╔══██╗ ██║   ██║ ╚════██║   ╚██╔╝
  ╚██████╔╝ ██║    ██║        ██████╔╝ ╚██████╔╝ ███████║    ██║
   ╚═════╝  ╚═╝    ╚═╝        ╚═════╝   ╚═════╝  ╚══════╝    ╚═╝
```

<p align="center"><img src="https://raw.githubusercontent.com/coderdan/goodcommit/main/assets/git-busy.png" alt="git busy cli interface"></p>
<p align="center">Let the computer write the commit message for you</p>

## What it does

This tool sends the output of `git diff --staged` to the GPT3 API and asks the model to write a commit message based off of the diff.
It then allows you to choose from three suggestions and you will be able to edit before it calls `git commit`.

If you supply a `-m` or `--message` flag GPT3 will not be consulted and instead your supplied message will be used for the commit.

## Requirements

You will need an API key for GPT3 as an environment variable called `GPT_API_KEY`.
Go to https://beta.openai.com/account/api-keys and create an account and the required key.

💡 _Large diffs won't work well so keep them small_

## Install

```sh
cargo install git-busy
```

## Usage

```
Usage: git busy [<options>] [--] <pathspec>...
```

💡 _Any arguments and flags are passed on to `git commit`._

## License
Licensed under the [GNU GPL-3.0-or-later](https://github.com/coderdan/goodcommit/blob/main/LICENSE).