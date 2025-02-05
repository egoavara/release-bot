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
pub struct WorkflowUpdateValidateRequestBean {
    #[serde(rename = "payload")]
    pub payload: Box<models::WorkflowUpdateRequest>,
    #[serde(rename = "validationOptions", skip_serializing_if = "Option::is_none")]
    pub validation_options: Option<Box<models::ValidationOptionsForUpdate>>,
}

impl WorkflowUpdateValidateRequestBean {
    pub fn new(payload: models::WorkflowUpdateRequest) -> WorkflowUpdateValidateRequestBean {
        WorkflowUpdateValidateRequestBean {
            payload: Box::new(payload),
            validation_options: None,
        }
    }
}

