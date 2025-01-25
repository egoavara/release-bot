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

/// SecuritySchemeWithProjects : Details about an issue security scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecuritySchemeWithProjects {
    /// The default level ID of the issue security scheme.
    #[serde(rename = "defaultLevel", skip_serializing_if = "Option::is_none")]
    pub default_level: Option<i64>,
    /// The description of the issue security scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the issue security scheme.
    #[serde(rename = "id")]
    pub id: i64,
    /// The name of the issue security scheme.
    #[serde(rename = "name")]
    pub name: String,
    /// The list of project IDs associated with the issue security scheme.
    #[serde(rename = "projectIds", skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<i64>>,
    /// The URL of the issue security scheme.
    #[serde(rename = "self")]
    pub param_self: String,
}

impl SecuritySchemeWithProjects {
    /// Details about an issue security scheme.
    pub fn new(id: i64, name: String, param_self: String) -> SecuritySchemeWithProjects {
        SecuritySchemeWithProjects {
            default_level: None,
            description: None,
            id,
            name,
            project_ids: None,
            param_self,
        }
    }
}

