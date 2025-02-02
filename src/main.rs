mod config;
mod cli;
mod ollama;
use reqwest::Error;


const PROJECT_NAME: &str = "hunnigan";


#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = config::config_get(); 
    cli::parse_cli(config).await?;
    Ok(())
}
