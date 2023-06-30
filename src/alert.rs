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
    status: String,
    reason: String,
    first_seen: u64,
    last_seen: u64,
    alert_time: u64,
    policy_id: String,
    // todo
    resource: V2AlertResponseItemResource,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct V2AlertResponseItemResource {
    rrn: Option<String>,
    id: String,
    account: String,
    account_id: String,
    region: String,
    region_id: String,
    // resource_type: String,
    resource_api_name: String,
    cloud_service_name: String,
    url: Option<String>,
    #[serde(flatten)]
    data: V2AlertResponseItemResourceKind,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
#[serde(tag = "resourceType")]
pub enum V2AlertResponseItemResourceKind {
    Instance(V2AlertResponseItemResourceInstanceData),
    #[serde(other)]
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct V2AlertResponseItemResourceDataTag {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct V2AlertResponseItemResourceInstanceData {
    tags: Option<Vec<V2AlertResponseItemResourceDataTag>>,
    state: V2AlertResponseItemResourceInstanceDataState,
    vpc_id: String,
    image_id: String,
    subnet_id: String,
    instance_id: String,
    private_dns_name: String,
    private_ip_address: String,
    iam_instance_profile: V2AlertResponseItemResourceInstanceDataIamIstanceProfile,
    cloud_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct V2AlertResponseItemResourceInstanceDataState {
    code: u16,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct V2AlertResponseItemResourceInstanceDataIamIstanceProfile {
    id: String,
    arn: String,
}
