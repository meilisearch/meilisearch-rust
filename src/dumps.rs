use crate::{client::Client, errors::Error, request::*};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DumpStatus {
    Done,
    InProgress,
    Failed,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DumpInfo {
    pub uid: String,
    pub status: DumpStatus,
    pub error: Option<serde_json::Value>,
}

impl<'a> Client<'a> {
    pub async fn create_dump(&self) -> Result<DumpInfo, Error> {
        request::<(), DumpInfo>(
            &format!("{}/dumps", self.host),
            self.apikey,
            Method::Post(()),
            202,
        )
        .await
    }

    pub async fn get_dump_status(&self, dump_uid: &str) -> Result<DumpInfo, Error> {
        request::<(), DumpInfo>(
            &format!("{}/dumps/{}/status", self.host, dump_uid),
            self.apikey,
            Method::Get,
            200,
        )
        .await
    }
}
