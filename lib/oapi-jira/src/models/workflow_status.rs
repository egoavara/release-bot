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

/// WorkflowStatus : Details of a workflow status.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowStatus {
    /// The ID of the issue status.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the status in the workflow.
    #[serde(rename = "name")]
    pub name: String,
    /// Additional properties that modify the behavior of issues in this status. Supports the properties `jira.issue.editable` and `issueEditable` (deprecated) that indicate whether issues are editable.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl WorkflowStatus {
    /// Details of a workflow status.
    pub fn new(id: String, name: String) -> WorkflowStatus {
        WorkflowStatus {
            id,
            name,
            properties: None,
        }
    }
}

