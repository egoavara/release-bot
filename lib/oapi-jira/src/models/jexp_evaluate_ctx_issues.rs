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

/// JexpEvaluateCtxIssues : The JQL specifying the issues available in the evaluated Jira expression under the `issues` context variable. This bean will be replacing `JexpIssues` bean as part of new `evaluate` endpoint
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JexpEvaluateCtxIssues {
    /// The JQL query that specifies the set of issues available in the Jira expression.
    #[serde(rename = "jql", skip_serializing_if = "Option::is_none")]
    pub jql: Option<Box<models::JexpEvaluateCtxJqlIssues>>,
}

impl JexpEvaluateCtxIssues {
    /// The JQL specifying the issues available in the evaluated Jira expression under the `issues` context variable. This bean will be replacing `JexpIssues` bean as part of new `evaluate` endpoint
    pub fn new() -> JexpEvaluateCtxIssues {
        JexpEvaluateCtxIssues {
            jql: None,
        }
    }
}

