use serde::{Serialize, Deserialize};
use confy::{store, ConfyError};
use std::io::Write;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    endpoint: String,
    model: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            endpoint: "http://localhost/api/generate".to_string(),
            model: "deepseek-r1:1.5b".to_string()
        }
    }
}

impl Config{
    pub fn get_endpoint(&self) -> String {
        self.endpoint.clone()
    }
    pub fn set_endpoint(&mut self, endpoint: String) -> Result<(), ConfyError> {
        self.endpoint = endpoint;
        store(crate::PROJECT_NAME, "Config", &self)
    }
    pub fn get_model(&self) -> String {
        self.model.clone()
    }
    pub fn set_model(&mut self, model: String) -> Result<(), ConfyError> { 
        self.model = model;
        store(crate::PROJECT_NAME, "Config", &self)
    }
    pub fn config_init(&mut self) -> Result<(), ConfyError> {
        let config_path = self.get_configuration_file_path();
        println!("Configuration file path: {:?}", config_path);

        let templates_path = self.get_templates_file_path();
        println!("Templates file path: {:?}", templates_path);

        if !templates_path.exists() {
            std::fs::create_dir(&templates_path).expect("create_dir call failed");
        }
        
        for entry in templates_path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                println!("{:?}", entry.path());
            }
        }

        let mut endpoint_response = String::new();
        if self.endpoint == "" {
            print!("API endpoint:");
        } else {
            print!("API endpoint [{}]:", self.endpoint);
        }
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut endpoint_response).unwrap();
        if endpoint_response.trim() != "" {
            self.endpoint = endpoint_response.trim().to_string();
            store(crate::PROJECT_NAME, "Config", &self)?
        }
        let mut model_response = String::new();
        if self.model == "" {
            print!("LLM model:");
        } else {
            print!("LLM model [{}]:", self.model);
        }
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut model_response).unwrap();
        if model_response.trim() != "" {
            self.model = model_response.trim().to_string();
            store(crate::PROJECT_NAME, "Config", &self)?
        }
        Ok(())
    }
    
    pub fn get_configuration_file_path(&self) -> PathBuf {
        confy::get_configuration_file_path(crate::PROJECT_NAME, "Config").unwrap()
    }
    pub fn get_configuration_path(&self) -> PathBuf {
        let mut path = confy::get_configuration_file_path(crate::PROJECT_NAME, "Config").unwrap();
        path.pop();
        path
    }

    pub fn get_templates_file_path(&self) -> PathBuf {
        let mut path = self.get_configuration_path();
        path.push("templates");
        path
    }
}

pub fn config_get() -> Config {
   let config = match confy::load(crate::PROJECT_NAME, "Config"){
        Ok(config) => config,
        Err(_) => Config::default(),
   };
   return config;
}
