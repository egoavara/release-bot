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

/// SearchAutoCompleteFilter : Details of how to filter and list search auto complete information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchAutoCompleteFilter {
    /// Include collapsed fields for fields that have non-unique names.
    #[serde(rename = "includeCollapsedFields", skip_serializing_if = "Option::is_none")]
    pub include_collapsed_fields: Option<bool>,
    /// List of project IDs used to filter the visible field details returned.
    #[serde(rename = "projectIds", skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<i64>>,
}

impl SearchAutoCompleteFilter {
    /// Details of how to filter and list search auto complete information.
    pub fn new() -> SearchAutoCompleteFilter {
        SearchAutoCompleteFilter {
            include_collapsed_fields: None,
            project_ids: None,
        }
    }
}

