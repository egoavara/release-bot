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

/// WorkflowStatusAndPort : The status reference and port that a transition is connected to.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowStatusAndPort {
    /// The port the transition is connected to this status.
    #[serde(rename = "port", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub port: Option<Option<i32>>,
    /// The reference of this status.
    #[serde(rename = "statusReference", skip_serializing_if = "Option::is_none")]
    pub status_reference: Option<String>,
}

impl WorkflowStatusAndPort {
    /// The status reference and port that a transition is connected to.
    pub fn new() -> WorkflowStatusAndPort {
        WorkflowStatusAndPort {
            port: None,
            status_reference: None,
        }
    }
}

