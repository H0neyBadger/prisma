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

    pub async fn list_v2_alert<T, Q>(&self, query: &Q) -> Result<T, Error>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        let uri = "v2/alert";
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
    severity: PolicySeverity,
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
pub enum PolicySeverity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AlertStatus {
    Open,
    Dismissed,
    Snoozed,
    Resolved,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CloudType {
    All,
    Aws,
    Azure,
    Gcp,
    AlibabaCloud,
    Oci,
    Ibm,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListV2AlertResponse {
    pub total_rows: u32,
    pub items: Vec<V2AlertResponseItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct V2AlertResponseItem {
    pub id: String,
    pub status: String,
    pub reason: String,
    pub first_seen: u64,
    pub last_seen: u64,
    pub alert_time: u64,
    pub policy_id: String,
    // todo
    pub resource: V2AlertResponseItemResource,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct V2AlertResponseItemResource {
    pub rrn: Option<String>,
    pub id: String,
    pub name: String,
    pub account: String,
    pub account_id: String,
    pub region: String,
    pub region_id: String,
    pub resource_type: String,
    pub resource_api_name: String,
    pub cloud_service_name: String,
    pub url: Option<String>,
    pub data: Value,
    pub cloud_type: CloudType,
    #[serde(flatten)]
    pub other: Value,
}
