use chatgpt::prelude::*;
use inquire::{CustomUserError, Text};
use std::process::exit;

const ENV_VAR: &str = "CHATGPT_SESSION_TOKEN";

fn commands(_val: &str) -> Result<Vec<String>, CustomUserError> {
    let commands = ["new", "help", "quit", "exit"];
    Ok(commands.iter().map(|s| String::from(*s)).collect())
}

fn has_command(command: &str, user_string: &str) -> bool {
    if command.len() < user_string.len() {
        return false;
    }

    let substr = command.get(0..user_string.len()).unwrap();

    substr == user_string
}

#[tokio::main]
async fn main() -> chatgpt::Result<()> {
    let token: String = match std::env::var(ENV_VAR) {
        Ok(v) => v,
        _ => {
            panic!("Failed to read env-var \"{ENV_VAR}\"");
        }
    };

    let mut client = ChatGPT::new(token)?;
    client.refresh_token().await?;
    //let mut conversation = client.new_conversation();

    loop {
        let command = Text::new("").with_autocomplete(&commands).prompt().unwrap();

        if has_command("quit", &command) || has_command("exit", &command) {
            exit(0);
        }

        let response = client.send_message(command).await?;
        //let response = conversation.send_message(&client, command).await?;
        println!("answer: {}", response);
        //let mut response = conversation
        //    .send_message_streaming(&client, command)
        //    .await?;

        // while let Some(line) = response.next().await {
        //     let text = format!("{line:?}");
        //     termimad::print_inline(&text);
        // }
    }
}
