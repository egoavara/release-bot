/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-7976c7d8afd785633bfb479e9cd673542daba37d
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachmentArchive {
    #[serde(rename = "entries", skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<models::AttachmentArchiveEntry>>,
    #[serde(rename = "moreAvailable", skip_serializing_if = "Option::is_none")]
    pub more_available: Option<bool>,
    #[serde(rename = "totalEntryCount", skip_serializing_if = "Option::is_none")]
    pub total_entry_count: Option<i32>,
    #[serde(rename = "totalNumberOfEntriesAvailable", skip_serializing_if = "Option::is_none")]
    pub total_number_of_entries_available: Option<i32>,
}

impl AttachmentArchive {
    pub fn new() -> AttachmentArchive {
        AttachmentArchive {
            entries: None,
            more_available: None,
            total_entry_count: None,
            total_number_of_entries_available: None,
        }
    }
}

