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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraColorField {
    #[serde(rename = "color")]
    pub color: Box<models::JiraColorInput>,
    #[serde(rename = "fieldId")]
    pub field_id: String,
}

impl JiraColorField {
    pub fn new(color: models::JiraColorInput, field_id: String) -> JiraColorField {
        JiraColorField {
            color: Box::new(color),
            field_id,
        }
    }
}

