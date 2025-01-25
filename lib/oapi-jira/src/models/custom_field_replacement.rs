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

/// CustomFieldReplacement : Details about the replacement for a deleted version.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldReplacement {
    /// The ID of the custom field in which to replace the version number.
    #[serde(rename = "customFieldId", skip_serializing_if = "Option::is_none")]
    pub custom_field_id: Option<i64>,
    /// The version number to use as a replacement for the deleted version.
    #[serde(rename = "moveTo", skip_serializing_if = "Option::is_none")]
    pub move_to: Option<i64>,
}

impl CustomFieldReplacement {
    /// Details about the replacement for a deleted version.
    pub fn new() -> CustomFieldReplacement {
        CustomFieldReplacement {
            custom_field_id: None,
            move_to: None,
        }
    }
}

