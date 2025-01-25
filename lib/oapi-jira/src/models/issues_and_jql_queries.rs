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

/// IssuesAndJqlQueries : List of issues and JQL queries.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssuesAndJqlQueries {
    /// A list of issue IDs.
    #[serde(rename = "issueIds")]
    pub issue_ids: Vec<i64>,
    /// A list of JQL queries.
    #[serde(rename = "jqls")]
    pub jqls: Vec<String>,
}

impl IssuesAndJqlQueries {
    /// List of issues and JQL queries.
    pub fn new(issue_ids: Vec<i64>, jqls: Vec<String>) -> IssuesAndJqlQueries {
        IssuesAndJqlQueries {
            issue_ids,
            jqls,
        }
    }
}

