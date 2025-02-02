use reqwest::{Client, Error};
use tera::Tera;
use serde::{Serialize, Deserialize};
use zenity::spinner::MultiSpinner;
use crate::config;

#[derive(Debug, Serialize, Deserialize)]
struct OllamaPost {
    model: String,
    stream: bool,
    prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
    done_reason: String,
    context: Vec<i128>,
    total_duration: i128,
    prompt_eval_count: i128,
    prompt_eval_duration: i128,
    eval_count: i128,
    eval_duration: i128,
}

fn template_parser(context: tera::Context) -> String {
    let tera = Tera::new("templates/**/*").unwrap();
    tera.render("agent", &context).unwrap()
}

async fn get_response(ollama_post: OllamaPost, endpoint: String) -> Result<String, Error> {
    let client = Client::new();
    
    let spinner = MultiSpinner::default();
    spinner.set_text(&spinner.get_last(), "  Waiting for response...".to_string());
    let res: OllamaResponse = client.post(endpoint).json(&ollama_post).send().await?.json().await?;
    let position = res.response.find("</think>");
    let response = if position.is_some() {
        let real_position = position.unwrap() + "</think>".len();
        &res.response[real_position..].trim()
    } else {
        res.response.trim()
    };
    Ok(response.to_string())
}

pub async fn ask_command(prompt: String, config: config::Config) -> String {
    let mut context = tera::Context::new();
    context.insert("prompt", &prompt); 
    
    let rendered = template_parser(context);
    
    let post = OllamaPost {
        model: config.get_model(),
        stream: false,
        prompt: rendered,
    };

    get_response(post, config.get_endpoint()).await.unwrap()
}

