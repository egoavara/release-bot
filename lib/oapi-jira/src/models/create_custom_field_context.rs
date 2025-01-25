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

/// CreateCustomFieldContext : The details of a created custom field context.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateCustomFieldContext {
    /// The description of the context.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the context.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The list of issue types IDs for the context. If the list is empty, the context refers to all issue types.
    #[serde(rename = "issueTypeIds", skip_serializing_if = "Option::is_none")]
    pub issue_type_ids: Option<Vec<String>>,
    /// The name of the context.
    #[serde(rename = "name")]
    pub name: String,
    /// The list of project IDs associated with the context. If the list is empty, the context is global.
    #[serde(rename = "projectIds", skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<String>>,
}

impl CreateCustomFieldContext {
    /// The details of a created custom field context.
    pub fn new(name: String) -> CreateCustomFieldContext {
        CreateCustomFieldContext {
            description: None,
            id: None,
            issue_type_ids: None,
            name,
            project_ids: None,
        }
    }
}

