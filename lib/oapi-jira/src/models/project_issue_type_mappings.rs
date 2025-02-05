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

/// ProjectIssueTypeMappings : The project and issue type mappings.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectIssueTypeMappings {
    /// The project and issue type mappings.
    #[serde(rename = "mappings")]
    pub mappings: Vec<models::ProjectIssueTypeMapping>,
}

impl ProjectIssueTypeMappings {
    /// The project and issue type mappings.
    pub fn new(mappings: Vec<models::ProjectIssueTypeMapping>) -> ProjectIssueTypeMappings {
        ProjectIssueTypeMappings {
            mappings,
        }
    }
}

