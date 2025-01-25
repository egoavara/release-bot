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

/// JqlFunctionPrecomputationUpdateRequestBean : List of pairs (id and value) for precomputation updates.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlFunctionPrecomputationUpdateRequestBean {
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<models::JqlFunctionPrecomputationUpdateBean>>,
}

impl JqlFunctionPrecomputationUpdateRequestBean {
    /// List of pairs (id and value) for precomputation updates.
    pub fn new() -> JqlFunctionPrecomputationUpdateRequestBean {
        JqlFunctionPrecomputationUpdateRequestBean {
            values: None,
        }
    }
}

