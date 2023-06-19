use reqwest::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use super::session::Session;

pub struct Alert<'a> {
    session: &'a Session<'a>,
}

impl<'a> Alert<'a> {
    pub fn new(session: &'a Session<'a>) -> Self {
        Self { session: session }
    }
    pub async fn get_alert_counts_grouped_by_policy<T, Q>(&self, query: &Q) -> Result<T, Error>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        // https://pan.dev/prisma-cloud/api/cspm/post-alerts-grouped/
        let uri = "alert/policy";
        self.session.post(uri, query).await
    }
}

pub type GetAlertCountsGroupedByPolicyResponse = Vec<GetAlertCountsGroupedByPolicyResponseItem>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAlertCountsGroupedByPolicyResponseItem {
    alert_count: u32,
    policy: GetAlertCountsGroupedByPolicyResponseItemPolicy,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAlertCountsGroupedByPolicyResponseItemPolicy {
    policy_id: String,
    severity: GetAlertCountsGroupedByPolicyResponseItemPolicySeverity,
    name: String,
    description: String,
    recommendation: String,
    cloud_type: String,
    policy_type: String,
    compliance_metadata: Vec<GetAlertCountsGroupedByPolicyResponseItemPolicyComplianceMetadata>,
    created_by: String,
    created_on: u64,
    last_modified_by: String,
    last_modified_on: u64,
    remediable: bool,
    remediation: Option<Value>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum GetAlertCountsGroupedByPolicyResponseItemPolicySeverity {
    High,
    Medium,
    Low,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAlertCountsGroupedByPolicyResponseItemPolicyComplianceMetadata {
    standard_name: String,
    standard_description: String,
    system_default: bool,
    section_id: String,
    section_description: String,
}
