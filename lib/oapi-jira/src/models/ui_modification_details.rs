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

/// UiModificationDetails : The details of a UI modification.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UiModificationDetails {
    /// List of contexts of the UI modification. The maximum number of contexts is 1000.
    #[serde(rename = "contexts", skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<models::UiModificationContextDetails>>,
    /// The data of the UI modification. The maximum size of the data is 50000 characters.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// The description of the UI modification. The maximum length is 255 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the UI modification.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the UI modification. The maximum length is 255 characters.
    #[serde(rename = "name")]
    pub name: String,
    /// The URL of the UI modification.
    #[serde(rename = "self")]
    pub param_self: String,
}

impl UiModificationDetails {
    /// The details of a UI modification.
    pub fn new(id: String, name: String, param_self: String) -> UiModificationDetails {
        UiModificationDetails {
            contexts: None,
            data: None,
            description: None,
            id,
            name,
            param_self,
        }
    }
}

