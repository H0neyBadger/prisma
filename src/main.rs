// use serde_json::Value;
use session::Session;
use std::env;

mod alert;
mod filter;
mod session;

use alert::GetAlertCountsGroupedByPolicyResponse;
use filter::{Query, TimeRangeKind, TimeRangeValueUnit};

#[tokio::main]
async fn main() {
    let api_endpoint = env::var("PRISMA_API_ENDPOINT").unwrap();
    let access_key = env::var("PRISMA_ACCESS_KEY").unwrap();
    let secret_key = env::var("PRISMA_SECRET_KEY").unwrap();
    let session = Session::new(
        api_endpoint.as_str(),
        access_key.as_str(),
        secret_key.as_str(),
    );
    let session = session
        .login()
        .await
        .expect("Login failed, Unable to retrieve token from access key");
    let query = Query::builder()
        .add_filter("timeRange.type", "=", "ALERT_OPENED")
        .add_filter("alert.status", "=", "open")
        .add_filter("policy.severity", "=", "high")
        .time_range(TimeRangeKind::Relative, TimeRangeValueUnit::Hour, "72000")
        .build();
    let alert = alert::Alert::new(&session);
    let values: GetAlertCountsGroupedByPolicyResponse = alert
        .get_alert_counts_grouped_by_policy(&query)
        .await
        .unwrap();
    println!("{}", serde_json::to_string_pretty(&values).unwrap());
}
