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

/// PrioritySchemeWithPaginatedPrioritiesAndProjects : A priority scheme with paginated priorities and projects.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrioritySchemeWithPaginatedPrioritiesAndProjects {
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /// The ID of the default issue priority.
    #[serde(rename = "defaultPriorityId", skip_serializing_if = "Option::is_none")]
    pub default_priority_id: Option<String>,
    /// The description of the priority scheme
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the priority scheme.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// The name of the priority scheme
    #[serde(rename = "name")]
    pub name: String,
    /// The paginated list of priorities.
    #[serde(rename = "priorities", skip_serializing_if = "Option::is_none")]
    pub priorities: Option<Box<models::PageBeanPriorityWithSequence>>,
    /// The paginated list of projects.
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<Box<models::PageBeanProjectDetails>>,
    /// The URL of the priority scheme.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

impl PrioritySchemeWithPaginatedPrioritiesAndProjects {
    /// A priority scheme with paginated priorities and projects.
    pub fn new(id: String, name: String) -> PrioritySchemeWithPaginatedPrioritiesAndProjects {
        PrioritySchemeWithPaginatedPrioritiesAndProjects {
            default: None,
            default_priority_id: None,
            description: None,
            id,
            is_default: None,
            name,
            priorities: None,
            projects: None,
            param_self: None,
        }
    }
}

