use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use super::alert::GetAlertCountsGroupedByPolicyResponse;
use super::filter::Query;

const PATH: &'static str = "config.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    // token: &'a str,
    pub query: Query,
    pub state: GetAlertCountsGroupedByPolicyResponse,
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
    query: Query,
    state: GetAlertCountsGroupedByPolicyResponse,
}

impl ConfigBuilder {
    pub fn query(mut self, value: Query) -> Self {
        self.query = value;
        self
    }

    pub fn state(mut self, value: GetAlertCountsGroupedByPolicyResponse) -> Self {
        self.state = value;
        self
    }

    pub fn build(self) -> Config {
        Config {
            query: self.query,
            state: self.state,
        }
    }
}
