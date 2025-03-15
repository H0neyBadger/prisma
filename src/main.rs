// use serde_json::Value;
use std::collections::HashMap;

use notify_rust::{Notification, Timeout};
use session::Session;
use std::env;

mod policy;
mod alert;
mod config;
mod filter;
mod session;

use policy::{V2PolicyResponseItem};
use alert::{ListV2AlertResponse, V2AlertResponseItem};
use config::Config;
// use filter::{Query, TimeRangeKind, TimeRangeValueUnit};
//

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
        config.token,
    );

    let session = session
        .login_or_refresh()
        .await
        .expect("Login failed, Unable to retrieve token from access key");
    let query = config.query;
    let policy = policy::Policy::new(&session);
    let alert = alert::Alert::new(&session);

    let policies: Vec<V2PolicyResponseItem> = policy.list_v2_policy().await.unwrap();
    let policies: HashMap<String, String> = policies
        .into_iter()
        .map(|v| (v.policy_id, v.name))
        .collect();

    let values: ListV2AlertResponse = alert.list_v2_alert(&query).await.unwrap();
    println!("{}", serde_json::to_string_pretty(&values).unwrap());

    let mut new_alerts: Vec<V2AlertResponseItem> = Vec::new();
    let mut config_builder = Config::builder().query(query).token(session.token);

    for item in values.items.into_iter() {
        config_builder = config_builder.add_alert(item.id.clone());
        if !config.alerts.iter().any(|v| v == &item.id) {
            // new alerts
            Notification::new()
                .summary(
                    format!(
                        "{} - {} ({})",
                        item.reason, item.id, item.resource.resource_type,
                    )
                    .as_str(),
                )
                .body(
                    format!(
                        "{}\n{} {} {}",
                        policies.get(&item.policy_id).unwrap_or(&item.policy_id),
                        item.resource.account_id,
                        item.resource.name,
                        item.resource
                            .url
                            .as_deref()
                            .unwrap_or(item.resource.region_id.as_str())
                    )
                    .as_str(),
                )
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
