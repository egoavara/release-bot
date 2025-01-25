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

/// IssueArchivalSyncRequest : List of Issue Ids Or Keys that are to be archived or unarchived
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueArchivalSyncRequest {
    #[serde(rename = "issueIdsOrKeys", skip_serializing_if = "Option::is_none")]
    pub issue_ids_or_keys: Option<Vec<String>>,
}

impl IssueArchivalSyncRequest {
    /// List of Issue Ids Or Keys that are to be archived or unarchived
    pub fn new() -> IssueArchivalSyncRequest {
        IssueArchivalSyncRequest {
            issue_ids_or_keys: None,
        }
    }
}

