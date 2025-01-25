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

/// WorkflowTransitionRulesUpdate : Details about a workflow configuration update request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTransitionRulesUpdate {
    /// The list of workflows with transition rules to update.
    #[serde(rename = "workflows")]
    pub workflows: Vec<models::WorkflowTransitionRules>,
}

impl WorkflowTransitionRulesUpdate {
    /// Details about a workflow configuration update request.
    pub fn new(workflows: Vec<models::WorkflowTransitionRules>) -> WorkflowTransitionRulesUpdate {
        WorkflowTransitionRulesUpdate {
            workflows,
        }
    }
}

