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

/// PriorityId : The ID of an issue priority.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PriorityId {
    /// The ID of the issue priority.
    #[serde(rename = "id")]
    pub id: String,
}

impl PriorityId {
    /// The ID of an issue priority.
    pub fn new(id: String) -> PriorityId {
        PriorityId {
            id,
        }
    }
}

