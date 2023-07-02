// use serde_json::Value;
use notify_rust::{Notification, Timeout};
use session::Session;
use std::env;

mod alert;
mod config;
mod filter;
mod session;

use alert::{ListV2AlertResponse, V2AlertResponseItem};
use config::Config;
// use filter::{Query, TimeRangeKind, TimeRangeValueUnit};

#[tokio::main]
async fn main() {
    let api_endpoint =
        env::var("PRISMA_API_ENDPOINT").expect("Env var PRISMA_API_ENDPOINT is not defined");
    let access_key =
        env::var("PRISMA_ACCESS_KEY").expect("Env var PRISMA_ACCESS_KEY is not defined");
    let secret_key =
        env::var("PRISMA_SECRET_KEY").expect("Env var PRISMA_SECRET_KEY is not defined");

    let config = match Config::load() {
        Ok(config) => config,
        Err(err)
            if err
                .downcast_ref::<std::io::Error>()
                .and_then(|v| Some(v.kind() == std::io::ErrorKind::NotFound))
                .is_some() =>
        {
            Config::builder().build()
        }
        Err(err) => panic!("{err}"),
    };
    let session = Session::new(
        api_endpoint.as_str(),
        access_key.as_str(),
        secret_key.as_str(),
    );
    let session = session
        .login()
        .await
        .expect("Login failed, Unable to retrieve token from access key");
    let query = config.query;
    // let query = Query::builder()
    //     .time_range(
    //         TimeRangeKind::Relative,
    //         TimeRangeValueUnit::Hour,
    //         String::from("72000"),
    //     )
    //     .add_filter("timeRange.type", "=", "ALERT_OPENED")
    //     .add_filter("alert.status", "=", "open")
    //     .add_filter("policy.severity", "=", "high")
    //     .add_filter("policy.type", "=", "anomaly")
    //     .add_filter("policy.type", "=", "attack_path")
    //     .add_filter("policy.type", "=", "audit_event")
    //     .add_filter("policy.type", "=", "network")
    //     .add_filter("policy.type", "=", "config")
    //     .add_filter("policy.type", "=", "workload_vulnerability")
    //     .add_filter("policy.type", "=", "workload_incident")
    //     .build();
    let alert = alert::Alert::new(&session);
    // let values = alert.list_v2_alert::<serde_json::Value, Query>(&query).await;
    // dbg!(&values);
    let values: ListV2AlertResponse = alert.list_v2_alert(&query).await.unwrap();
    println!("{}", serde_json::to_string_pretty(&values).unwrap());

    let mut new_alerts: Vec<V2AlertResponseItem> = Vec::new();
    let mut config_builder = Config::builder().query(query);
    for item in values.items.into_iter() {
        config_builder = config_builder.add_alert(item.id.clone());
        if !config.alerts.iter().any(|v| v == &item.id) {
            // new alerts
            Notification::new()
                .summary("Prisma cloud Alert")
                .body(format!("{}", serde_json::to_string_pretty(&item).unwrap()).as_str())
                .icon("alert")
                .appname("prisma-cloud")
                .timeout(Timeout::Never) // this however is
                .show()
                .unwrap();
            new_alerts.push(item);
        }
    }
    if !new_alerts.is_empty() {
        println!(
            "NEW Alerts: {}",
            serde_json::to_string_pretty(&new_alerts).unwrap()
        );
    }
    config_builder.build().save().unwrap();
}
