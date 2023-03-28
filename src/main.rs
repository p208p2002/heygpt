use atty::Stream;
use colored::Colorize;
use dirs;
use reqwest::blocking::{Client,Response};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, result};
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path};
use std::process::exit;
use std::time::Duration;
use toml;

enum EndpointProvider {
    OpenAI(Endpoint),
    Azure (Endpoint)
}

struct Endpoint{
    url:String,
    token:String
}

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
    model: Option<String>,
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

impl EndpointProvider {
    fn from_open_ai(token:String) ->EndpointProvider{
        EndpointProvider::OpenAI(
            Endpoint {
                url:String::from("https://api.openai.com/v1/chat/completions"), 
                token:token 
            }
        )
    }

    fn from_azure(token:String,url:String) ->EndpointProvider{
        EndpointProvider::Azure(
            Endpoint {
                url, 
                token:token 
            }
        )
    }
   
    fn chat_completion(&self,messages: &Vec<Message>)->Result<Response,Error>{
        
        let client = Client::builder()
        .timeout(Duration::from_secs(180))
        .build()
        .expect("client build failed");
        
        let response:Result<Response,Error>;
        match self {
            EndpointProvider::OpenAI(endpoint)=>{
                let playload = ChatGptRequest {
                    model: Some(String::from("gpt-3.5-turbo")),
                    messages: messages.to_vec(),
                };

                response = client
                .post(&endpoint.url)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", endpoint.token))
                .json(&playload)
                .send();
            },
            EndpointProvider::Azure(endpoint)=>{
                let playload = ChatGptRequest {
                    model:None,
                    messages: messages.to_vec(),
                };
                response = client
                .post(&endpoint.url)
                .header("Content-Type", "application/json")
                .header("api-key", &endpoint.token)
                .json(&playload)
                .send();
            }
        };

        return response;

    }


}

impl ChatManager {
    fn new() -> ChatManager {
        ChatManager {
            messages: Vec::new(),
        }
    }
}

fn init_app_config() {
    //allow user create a app config
    let args: Vec<String> = env::args().collect();
    let home_dir = dirs::home_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
    let file_path = Path::new(&home_dir).join(".heygpt-config");

    if args.len() == 2 && args[1] == "init" {
        println!("Paste your opean ai token bellow:");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let user_input = user_input.trim();

        let os = env::consts::OS;
        let data = format!("openai_token = \"{user_input}\"\nsystem = \"plantform:{os}\"");
        fs::write(file_path, data).unwrap();
        exit(0)
    }

    else if args.len() == 2 && args[1] == "init-azure" {
        println!("Paste your azure token bellow:");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let user_input = user_input.trim();

        // set os
        let os = env::consts::OS;
        let mut data = format!("system = \"plantform:{os}\"\n");
        

        // set use azure
        data.push_str("use_azure=\"true\"\n");
        
        // set azure token
        let azure_token_format = format!("azure_token = \"{user_input}\"\n");
        data.push_str(&azure_token_format);

        // set azure base-url
        println!("\nPaste your azure endpoint bellow\n(https://<USER_COUNT>.openai.azure.com/openai/deployments/<MODEL_DEPLOY_NAME>/chat/completions?api-version=2023-03-15-preview):");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let user_input = user_input.trim();
        let azure_base_url_format = format!("azure_base_url = \"{user_input}\"\n");
        data.push_str(&azure_base_url_format);

        fs::write(file_path, data).unwrap();
        exit(0)
    } 
}

fn get_config()->HashMap<String, String>{
    let home_dir = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let file_path = Path::new(&home_dir).join(".heygpt-config");
    let toml_data = fs::read_to_string(file_path);
    let toml_data = match toml_data {
        Ok(data)=>data,
        Err(_err)=>{
            println!("Please run `hetgpt init` to configure api token");
            println!("Get the token from https://platform.openai.com/account/api-keys");
            exit(0)
        }
    };

    let config: HashMap<String, String> = toml::from_str(&toml_data).unwrap();
    return config;
}

fn read_pipe() -> String {
    // No pipe input detected
    if atty::is(Stream::Stdin) {
        return String::new();
    }

    // read from pipe
    let stdin = io::stdin();
    let mut stdin_lines = String::new();

    for line in stdin.lock().lines() {
        let mut line = line.expect("Could not read line from standard in");
        line.push_str("\n");
        stdin_lines.push_str(&line);
    }

    return stdin_lines;
}

fn call_chat_gpt(messages: &Vec<Message>) -> String {
    // api endpoint
    let config = get_config();

    // use azure or open ai
    let _default_use_azure = String::from("false");
    let use_azure = config.get("use_azure").unwrap_or(&_default_use_azure);

    // instance endpoint
    let token:String;
    let endpoint:EndpointProvider;
    if use_azure == "true"{
        token = config["azure_token"].clone();
        let url =config["azure_base_url"].clone();
        endpoint = EndpointProvider::from_azure(token,url);
    }
    else {
        token = get_config()["openai_token"].clone();
        endpoint = EndpointProvider::from_open_ai(token);
    }

    let response = endpoint.chat_completion(messages).unwrap();    
    let response_result: ChatGptResponse = serde_json::from_str(&response.text().unwrap()).unwrap();
    let response_content = response_result.choices[0]
        .message
        .content
        .trim()
        .to_string();
    return response_content;
}

fn main() {
    init_app_config();

    // read args
    let args: Vec<String> = env::args().collect();

    // stdin from pipe
    let std_lines = read_pipe();
    if std_lines.len() > 0 {
        let mut action: String = String::new();
        if args.len() > 1 {
            action = args[1].clone();
        }
        let content: String = format!("{action}\n{std_lines}");

        let messages = vec![Message {
            role: String::from("user"),
            content: content,
        }];
        let response_content = call_chat_gpt(&messages);
        println!("{response_content}");
        exit(0);
    }

    // quick chat
    let quick_chat;
    if args.len() >= 2 {
        quick_chat = args[1].clone();
        let sys_content = get_config()["system"].clone();
        let messages = vec![
            Message {
                role: String::from("system"),
                content: sys_content,
            },
            Message {
                role: String::from("user"),
                content: quick_chat,
            },
        ];
        let response_content = call_chat_gpt(&messages);
        println!("{response_content}");
        exit(0);
    }

    // go into a chat loop
    let user_label = "[👦]:".blue();
    let chat_gpt_label = "[🤖]:".red();

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
