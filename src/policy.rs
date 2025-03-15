use reqwest::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::session::Session;

pub struct Policy<'a> {
    session: &'a Session<'a>,
}

impl<'a> Policy<'a> {
    pub fn new(session: &'a Session<'a>) -> Self {
        Self { session: session }
    }

    pub async fn list_v2_policy<T>(&self) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let uri = "v2/policy";
        self.session.get_with_query(uri, &[("slimView", "true"), ("detailedComplianceMappings", "false")]).await
    }
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct V2PolicyResponseItem {
    pub policy_id: String,
    pub name: String,
}
