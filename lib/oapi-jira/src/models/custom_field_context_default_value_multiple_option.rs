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

/// CustomFieldContextDefaultValueMultipleOption : The default value for a multi-select custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueMultipleOption {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The list of IDs of the default options.
    #[serde(rename = "optionIds")]
    pub option_ids: Vec<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomFieldContextDefaultValueMultipleOption {
    /// The default value for a multi-select custom field.
    pub fn new(context_id: String, option_ids: Vec<String>, r#type: String) -> CustomFieldContextDefaultValueMultipleOption {
        CustomFieldContextDefaultValueMultipleOption {
            context_id,
            option_ids,
            r#type,
        }
    }
}

