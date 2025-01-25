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

/// SearchAndReconcileResults : The result of a JQL search with issues reconsilation.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchAndReconcileResults {
    /// The list of issues found by the search or reconsiliation.
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<models::IssueBean>>,
    /// The ID and name of each field in the search results.
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<std::collections::HashMap<String, String>>,
    /// Continuation token to fetch the next page. If this result represents the last or the only page this token will be null. This token will expire in 7 days.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// The schema describing the field types in the search results.
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<std::collections::HashMap<String, models::JsonTypeBean>>,
}

impl SearchAndReconcileResults {
    /// The result of a JQL search with issues reconsilation.
    pub fn new() -> SearchAndReconcileResults {
        SearchAndReconcileResults {
            issues: None,
            names: None,
            next_page_token: None,
            schema: None,
        }
    }
}

