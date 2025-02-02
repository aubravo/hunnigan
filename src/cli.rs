use clap::{Parser, Subcommand};
use crate::ollama;
use crate::config;
use reqwest::Error;

#[derive(Parser, Debug)]
#[command(name=crate::PROJECT_NAME)]
#[command(about="👩💻 An intelligence agent providing assistance directly where you need it.")]
pub struct Cli {
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    pub command: Command,

    #[arg(short, long, default_value="")]
    model: Option<String>,

    #[arg(short, long, default_value="")]
    endpoint: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Ask{
        prompt: String,
    },
    Config{
        #[command(subcommand)]
        command: ConfigSubcommand
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigSubcommand {
    Get{
        parameter: String,
    },
    Set{
        parameter: String,
        value: String
    },
    Init{}
}

pub fn get_parsed_cli() -> Cli {
    Cli::parse()
}

pub async fn parse_cli(mut config: config::Config) -> Result<(), Error> {
    let args = get_parsed_cli();
    
    match args.command {
        Command::Ask {prompt} => println!("{}", termimad::inline(&ollama::ask_command(prompt, config).await)),
        Command::Config {command} => {
            match command {
                ConfigSubcommand::Get { parameter } => {
                    match parameter{
                        val if val == "model".to_string() => println!("{}",config.get_model()),
                        val if val == "endpoint".to_string() => println!("{}", config.get_endpoint()),
                        _ => panic!("Parameter was not recognized!"),
                    }
                },
                ConfigSubcommand::Set { parameter, value } => {
                     match parameter{
                        val if val == "model".to_string() => config.set_model(value).expect("Something went wrong when setting the model!"),
                        val if val == "endpoint".to_string() => config.set_endpoint(value).expect("Something went wrong when setting the endpoint!"),
                        _ => panic!("Parameter was not recognized!"),
                    }                  
                },
                ConfigSubcommand::Init {} => config.config_init().expect("Something went wrong when initializing the configuration!"),
            }
        },
    };
    Ok(())
}
