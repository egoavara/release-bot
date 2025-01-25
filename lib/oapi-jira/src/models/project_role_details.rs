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

/// ProjectRoleDetails : Details about a project role.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectRoleDetails {
    /// Whether this role is the admin role for the project.
    #[serde(rename = "admin", skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    /// Whether this role is the default role for the project.
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /// The description of the project role.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the project role.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the project role.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the roles are configurable for this project.
    #[serde(rename = "roleConfigurable", skip_serializing_if = "Option::is_none")]
    pub role_configurable: Option<bool>,
    /// The scope of the role. Indicated for roles associated with [next-gen projects](https://confluence.atlassian.com/x/loMyO).
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<models::Scope>,
    /// The URL the project role details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The translated name of the project role.
    #[serde(rename = "translatedName", skip_serializing_if = "Option::is_none")]
    pub translated_name: Option<String>,
}

impl ProjectRoleDetails {
    /// Details about a project role.
    pub fn new() -> ProjectRoleDetails {
        ProjectRoleDetails {
            admin: None,
            default: None,
            description: None,
            id: None,
            name: None,
            role_configurable: None,
            scope: None,
            param_self: None,
            translated_name: None,
        }
    }
}

