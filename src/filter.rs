use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum TimeRangeKind {
    #[default]
    Relative,
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum TimeRangeValueUnit {
    #[default]
    Hour,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct TimeRangeValue {
    amount: String,
    unit: TimeRangeValueUnit,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TimeRange {
    #[serde(rename = "type")]
    kind: TimeRangeKind,
    value: TimeRangeValue,
}

impl Default for TimeRange {
    fn default() -> Self {
        Self {
            kind: TimeRangeKind::Relative,
            value: TimeRangeValue {
                amount: String::from("24"),
                unit: TimeRangeValueUnit::Hour,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    web_client: bool,
    detailed: bool,
    filters: Vec<HashMap<String, String>>,
    time_range: TimeRange,
}

#[derive(Default)]
pub struct QueryBuilder {
    web_client: bool,
    detailed: bool,
    filters: Vec<HashMap<String, String>>,
    time_range: TimeRange,
}

#[allow(dead_code)]
impl QueryBuilder {
    pub fn build(self) -> Query {
        let default = Query::default();
        Query {
            web_client: self.web_client,
            detailed: self.detailed,
            filters: if self.filters.is_empty() {
                default.filters
            } else {
                self.filters
            },
            time_range: self.time_range,
        }
    }

    pub fn web_client(mut self, value: bool) -> Self {
        self.web_client = value;
        self
    }

    pub fn detailed(mut self, value: bool) -> Self {
        self.detailed = value;
        self
    }

    pub fn add_filter(mut self, name: &str, operator: &str, value: &str) -> Self {
        self.filters.push(HashMap::from([
            ("name".to_string(), name.to_string()),
            ("operator".to_string(), operator.to_string()),
            ("value".to_string(), value.to_string()),
        ]));
        self
    }

    pub fn time_range(
        mut self,
        kind: TimeRangeKind,
        unit: TimeRangeValueUnit,
        value: String,
    ) -> Self {
        self.time_range = TimeRange {
            kind: kind,
            value: TimeRangeValue {
                amount: value,
                unit: unit,
            },
        };
        self
    }
}
