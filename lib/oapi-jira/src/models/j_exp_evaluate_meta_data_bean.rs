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

/// JExpEvaluateMetaDataBean : Contains information about the expression evaluation. This bean will be replacing `JiraExpressionEvaluationMetaDataBean` bean as part of new `evaluate` endpoint
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JExpEvaluateMetaDataBean {
    /// Contains information about the expression complexity. For example, the number of steps it took to evaluate the expression.
    #[serde(rename = "complexity", skip_serializing_if = "Option::is_none")]
    pub complexity: Option<Box<models::JiraExpressionsComplexityBean>>,
    /// Contains information about the `issues` variable in the context. For example, is the issues were loaded with JQL, information about the page will be included here.
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Box<models::JExpEvaluateIssuesMetaBean>>,
}

impl JExpEvaluateMetaDataBean {
    /// Contains information about the expression evaluation. This bean will be replacing `JiraExpressionEvaluationMetaDataBean` bean as part of new `evaluate` endpoint
    pub fn new() -> JExpEvaluateMetaDataBean {
        JExpEvaluateMetaDataBean {
            complexity: None,
            issues: None,
        }
    }
}

