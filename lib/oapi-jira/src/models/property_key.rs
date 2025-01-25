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

/// PropertyKey : Property key details.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyKey {
    /// The key of the property.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The URL of the property.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

impl PropertyKey {
    /// Property key details.
    pub fn new() -> PropertyKey {
        PropertyKey {
            key: None,
            param_self: None,
        }
    }
}

