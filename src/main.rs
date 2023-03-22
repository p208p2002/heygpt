use colored::Colorize;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::exit;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    role: String,
    content: String,
}

struct ChatManager {
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatGptRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatGptResponseUsage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatGptResponseChoicesMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatGptResponseChoices {
    message: ChatGptResponseChoicesMessage,
    finish_reason: String,
    index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatGptResponse {
    id: String,
    object: String,
    created: usize,
    model: String,
    usage: ChatGptResponseUsage,
    choices: Vec<ChatGptResponseChoices>,
}

impl ChatManager {
    fn new() -> ChatManager {
        ChatManager {
            messages: Vec::new(),
        }
    }
}

fn call_chat_gpt(messages: &Vec<Message>) -> String {
    // api endpoint
    let endpoint = String::from("https://api.openai.com/v1/chat/completions");

    // get openai api key from HOME_PATH
    let mut openai_api_key;
    let mut path = PathBuf::from(std::env::var("HOME").unwrap());
    path.push(".chatgpt_token");
    // let contents = fs::read_to_string(path).expect("Should have been able to read ~/.chatgpt_token");
    let contents = fs::read_to_string(path);
    let contents = match contents {
        Ok(context) => context,
        Err(err) => panic!("~/.chatgpt_token not exist => {err}"),
    };
    openai_api_key = contents.clone();
    openai_api_key = openai_api_key.trim().to_string();

    let playload = ChatGptRequest {
        model: String::from("gpt-3.5-turbo"),
        messages: messages.to_vec(),
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(180))
        .build()
        .expect("client build failed");

    let response = client
        .post(&endpoint)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .json(&playload)
        .send()
        .unwrap();

    let response_result: ChatGptResponse = serde_json::from_str(&response.text().unwrap()).unwrap();
    let response_content = response_result.choices[0]
        .message
        .content
        .trim()
        .to_string();
    return response_content;
}

fn main() {
    // quick chat
    let quick_chat;
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        quick_chat = args[1].clone();
        let messages = vec![Message {
            role: String::from("user"),
            content: quick_chat,
        }];
        let response_content = call_chat_gpt(&messages);
        println!("{response_content}");
        exit(0);
    }

    // go into a chat loop
    let user_label = "[👦]:".blue();
    let chat_gpt_label = "[🤖]:".red();
    // let endpoint = String::from("https://api.openai.com/v1/chat/completions");
    let mut cm: ChatManager = ChatManager::new();

    loop {
        print!("{user_label} ");
        std::io::stdout().flush().unwrap();
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        // user
        let new_message = Message {
            role: String::from("user"),
            content: user_input,
        };
        cm.messages.push(new_message);

        let response_content = call_chat_gpt(&cm.messages);

        println!("{chat_gpt_label} {response_content}");

        // save bot response
        let new_message = Message {
            role: String::from("assistant"),
            content: String::from(response_content),
        };

        cm.messages.push(new_message);
    }
}
