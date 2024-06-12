use std::fs;

use colored::Colorize;
use serde::{Deserialize, Serialize};
use sqlx::{Connection, PgConnection};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    default: String,
    connection: Vec<ConnectionProfile>,
}

impl Config {
    pub fn read_config() -> Self {
        let path = std::env::var("TUSKCONFIG").unwrap_or_else(|_| {
            let mut path = dirs::home_dir().expect("Could not retrieve home directory");
            path.push(".tusk/config.toml");
            path.to_str().unwrap().to_owned()
        });

        // Read the file into a string
        let config_content = fs::read_to_string(path).expect("Could not read the config file");

        // Deserialize the string into Config
        toml::from_str(&config_content).expect("Could not deserialize the config file")
    }
    pub fn get(self, name: Option<String>) -> ConnectionProfile {
        let name = name.unwrap_or(self.default);

        self.connection
            .into_iter()
            .find(|p| p.name.eq(&name))
            .expect("Connection Profile does not exist.")
    }
    pub fn list_connection_profiles(&self) {
        println!(" Profiles\n --------");
        for profile in &self.connection {
            if profile.name.eq(&self.default) {
                println!("*{} (default)", profile.name.bold().green());
            } else {
                println!(" {}", profile.name);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionProfile {
    name: String,
    user: String,
    password: String,
    host: String,
    port: u16,
    database: String,
}

impl ConnectionProfile {
    fn url(&self) -> String {
        format!(
            "postgresql://{user}:{password}@{host}:{port}/{database}",
            user = self.user,
            password = self.password,
            host = self.host,
            port = self.port,
            database = self.database,
        )
    }
    pub async fn connect(self) -> PgConnection {
        PgConnection::connect(&self.url()).await.unwrap()
    }
}
