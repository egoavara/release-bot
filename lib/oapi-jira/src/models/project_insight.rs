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

/// ProjectInsight : Additional details about a project.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectInsight {
    /// The last issue update time.
    #[serde(rename = "lastIssueUpdateTime", skip_serializing_if = "Option::is_none")]
    pub last_issue_update_time: Option<String>,
    /// Total issue count.
    #[serde(rename = "totalIssueCount", skip_serializing_if = "Option::is_none")]
    pub total_issue_count: Option<i64>,
}

impl ProjectInsight {
    /// Additional details about a project.
    pub fn new() -> ProjectInsight {
        ProjectInsight {
            last_issue_update_time: None,
            total_issue_count: None,
        }
    }
}

