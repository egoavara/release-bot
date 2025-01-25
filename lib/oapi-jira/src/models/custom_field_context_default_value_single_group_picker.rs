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

/// CustomFieldContextDefaultValueSingleGroupPicker : The default value for a group picker custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueSingleGroupPicker {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The ID of the the default group.
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomFieldContextDefaultValueSingleGroupPicker {
    /// The default value for a group picker custom field.
    pub fn new(context_id: String, group_id: String, r#type: String) -> CustomFieldContextDefaultValueSingleGroupPicker {
        CustomFieldContextDefaultValueSingleGroupPicker {
            context_id,
            group_id,
            r#type,
        }
    }
}

