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

/// JexpEvaluateCtxJqlIssues : The JQL specifying the issues available in the evaluated Jira expression under the `issues` context variable. Not all issues returned by the JQL query are loaded, only those described by the `nextPageToken` and `maxResults` properties. This bean will be replacing JexpJqlIssues bean as part of new `evaluate` endpoint
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JexpEvaluateCtxJqlIssues {
    /// The maximum number of issues to return from the JQL query. max results value considered may be lower than the number specific here.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The token for a page to fetch that is not the first page. The first page has a `nextPageToken` of `null`. Use the `nextPageToken` to fetch the next page of issues.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// The JQL query, required to be bounded. Additionally, `orderBy` clause can contain a maximum of 7 fields
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl JexpEvaluateCtxJqlIssues {
    /// The JQL specifying the issues available in the evaluated Jira expression under the `issues` context variable. Not all issues returned by the JQL query are loaded, only those described by the `nextPageToken` and `maxResults` properties. This bean will be replacing JexpJqlIssues bean as part of new `evaluate` endpoint
    pub fn new() -> JexpEvaluateCtxJqlIssues {
        JexpEvaluateCtxJqlIssues {
            max_results: None,
            next_page_token: None,
            query: None,
        }
    }
}

