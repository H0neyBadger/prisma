use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum TimeRangeKind {
    #[default]
    Relative,
}
#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum TimeRangeValueUnit {
    #[default]
    Hour,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct TimeRangeValue<'a> {
    amount: &'a str,
    unit: TimeRangeValueUnit,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct TimeRange<'a> {
    #[serde(rename = "type")]
    kind: TimeRangeKind,
    value: TimeRangeValue<'a>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Query<'a> {
    web_client: bool,
    detailed: bool,
    filters: Vec<HashMap<&'a str, &'a str>>,
    time_range: TimeRange<'a>,
}

impl<'a> Query<'a> {
    pub fn builder() -> QueryBuilder<'a> {
        QueryBuilder::default()
    }
}

#[derive(Default)]
pub struct QueryBuilder<'a> {
    web_client: bool,
    detailed: bool,
    filters: Vec<HashMap<&'a str, &'a str>>,
    time_range: TimeRange<'a>,
}

#[allow(dead_code)]
impl<'a> QueryBuilder<'a> {
    pub fn build(self) -> Query<'a> {
        Query {
            web_client: self.web_client,
            detailed: self.detailed,
            filters: self.filters,
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

    pub fn add_filter(mut self, name: &'a str, operator: &'a str, value: &'a str) -> Self {
        self.filters.push(HashMap::from([
            ("name", name),
            ("operator", operator),
            ("value", value),
        ]));
        self
    }

    pub fn time_range(
        mut self,
        kind: TimeRangeKind,
        unit: TimeRangeValueUnit,
        value: &'a str,
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
