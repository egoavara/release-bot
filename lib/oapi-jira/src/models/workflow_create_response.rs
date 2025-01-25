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

/// WorkflowCreateResponse : Details of the created workflows and statuses.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowCreateResponse {
    /// List of created statuses.
    #[serde(rename = "statuses", skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<models::JiraWorkflowStatus>>,
    /// List of created workflows.
    #[serde(rename = "workflows", skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<models::JiraWorkflow>>,
}

impl WorkflowCreateResponse {
    /// Details of the created workflows and statuses.
    pub fn new() -> WorkflowCreateResponse {
        WorkflowCreateResponse {
            statuses: None,
            workflows: None,
        }
    }
}

