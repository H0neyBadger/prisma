use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use super::filter::Query;

const PATH: &'static str = "config.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub token: Option<String>,
    pub query: Query,
    pub alerts: Vec<String>,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let file = File::open(PATH)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    pub fn save(self) -> Result<(), Box<dyn Error>> {
        let file = File::create(PATH)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &self)?;
        writer.flush()?;
        Ok(())
    }
}

#[derive(Default)]
pub struct ConfigBuilder {
    token: Option<String>,
    query: Query,
    alerts: Vec<String>,
}

impl ConfigBuilder {
    pub fn query(mut self, value: Query) -> Self {
        self.query = value;
        self
    }

    pub fn add_alert(mut self, value: String) -> Self {
        self.alerts.push(value);
        self
    }

    pub fn token(mut self, value: Option<String>) -> Self {
        self.token = value;
        self
    }

    pub fn build(self) -> Config {
        Config {
            token: self.token,
            query: self.query,
            alerts: self.alerts,
        }
    }
}
