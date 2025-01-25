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

/// PageOfCreateMetaIssueTypes : A page of CreateMetaIssueTypes.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageOfCreateMetaIssueTypes {
    #[serde(rename = "createMetaIssueType", skip_serializing_if = "Option::is_none")]
    pub create_meta_issue_type: Option<Vec<models::IssueTypeIssueCreateMetadata>>,
    /// The list of CreateMetaIssueType.
    #[serde(rename = "issueTypes", skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Vec<models::IssueTypeIssueCreateMetadata>>,
    /// The maximum number of items to return per page.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The index of the first item returned.
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// The total number of items in all pages.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl PageOfCreateMetaIssueTypes {
    /// A page of CreateMetaIssueTypes.
    pub fn new() -> PageOfCreateMetaIssueTypes {
        PageOfCreateMetaIssueTypes {
            create_meta_issue_type: None,
            issue_types: None,
            max_results: None,
            start_at: None,
            total: None,
        }
    }
}

