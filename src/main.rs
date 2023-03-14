use std::io::*;
use openai::{chat::*, models::ModelID};
use owo_colors::OwoColorize;
use termimad;

#[tokio::main]
async fn main() {
    let stdin = std::io::stdin();

    let mut messages: Vec<ChatCompletionMessage> = vec![];

    do_prompt();
    for prompt in stdin.lock().lines() {
        let prompt = match prompt {
            Ok(prompt) => prompt,
            Err(..) => continue
        };

        let message = ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: prompt,
            name: None
        };

        messages.push(message);
        
        // Make openai request
        let response = ChatCompletionBuilder::default()
            .model(ModelID::Gpt3_5Turbo)
            .messages(messages.clone())
            .create()
            .await;

        let (response_message, token_count) = match response {
            Ok(response) => match response {
                Ok(response) => (response.choices[0].message.content.clone(), response.usage.unwrap().total_tokens),
                Err(e) => {
                    println!("An error occurred: {:#?}", e.message.red());
                    do_prompt();
                    continue;
                }
            },
            Err(e) => {
                println!("An error occurred: {:#?}", e.red());
                do_prompt();
                continue;
            }
        };

        let cost_cents = token_count as f64 * 0.0002;
        
        // Display response
        termimad::print_inline(&response_message);
        print!("\n");
        print!("\n");

        println!("(Cost: {:.4} cents)", cost_cents.blue());
        do_prompt();
    }
}

fn do_prompt() {
    print!("{}", "Prompt => ".blue());
    std::io::stdout().lock().flush().unwrap();
}
