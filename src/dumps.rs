//! The `dumps` module allows the creation of database dumps.
//! Dumps are `.dump` files that can be used to launch MeiliSearch.
//! Dumps are compatible between MeiliSearch versions.
//!
//! Creating a dump is also referred to as exporting it, whereas launching MeiliSearch with a dump is referred to as importing it.
//!
//! During a [dump export](Client::create_dump), all [indexes](crate::indexes::Index) of the current instance are exported—together with their documents and settings—and saved as a single `.dump` file.
//!
//! During a dump import, all indexes contained in the indicated `.dump` file are imported along with their associated [documents](crate::document::Document) and [settings](crate::settings::Settings).
//! Any existing [index](crate::indexes::Index) with the same uid as an index in the dump file will be overwritten.
//!
//! Dump imports are [performed at launch](https://docs.meilisearch.com/reference/features/configuration.html#import-dump) using an option.
//! [Batch size](https://docs.meilisearch.com/reference/features/configuration.html#dump-batch-size) can also be set at this time.

use crate::{client::Client, errors::Error, request::*};
use serde::Deserialize;

/// The status of a dump.\
/// Contained in [`DumpInfo`].
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DumpStatus {
    /// Dump creation is in progress.
    Done,
    /// Dump creation is in progress.
    InProgress,
    /// An error occured during dump process, and the task was aborted.
    Failed,
}

/// Limited informations about a dump.\
/// Can be obtained with [create_dump](Client::create_dump) and [get_dump_status](Client::get_dump_status) methods.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DumpInfo {
    pub uid: String,
    pub status: DumpStatus,
    pub error: Option<serde_json::Value>,
}

/// Dump related methods.\
/// See the [dumps](crate::dumps) module.
impl<'a> Client<'a> {
    /// Triggers a dump creation process.
    /// Once the process is complete, a dump is created in the [dumps directory](https://docs.meilisearch.com/reference/features/configuration.html#dumps-destination).
    /// If the dumps directory does not exist yet, it will be created.
    pub async fn create_dump(&self) -> Result<DumpInfo, Error> {
        request::<(), DumpInfo>(
            &format!("{}/dumps", self.host),
            self.apikey,
            Method::Post(()),
            202,
        )
        .await
    }

    /// Get the status of a dump creation process using [the uid](DumpInfo::uid) returned after calling the [dump creation method](Client::create_dump).
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

/// Alias for [create_dump](Client::create_dump).
pub async fn create_dump<'a>(client: &'a Client<'a>) -> Result<DumpInfo, Error> {
    client.create_dump().await
}

/// Alias for [get_dump_status](Client::get_dump_status).
pub async fn get_dump_status<'a>(client: &'a Client<'a>, dump_uid: &str) -> Result<DumpInfo, Error> {
    client.get_dump_status(dump_uid).await
}
