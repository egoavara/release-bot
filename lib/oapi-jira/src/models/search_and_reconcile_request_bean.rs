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
pub struct SearchAndReconcileRequestBean {
    /// Use [expand](#expansion) to include additional information about issues in the response. Note that, unlike the majority of instances where `expand` is specified, `expand` is defined as a comma-delimited string of values. The expand options are:   *  `renderedFields` Returns field values rendered in HTML format.  *  `names` Returns the display name of each field.  *  `schema` Returns the schema describing a field type.  *  `transitions` Returns all possible transitions for the issue.  *  `operations` Returns all possible operations for the issue.  *  `editmeta` Returns information about how each field can be edited.  *  `changelog` Returns a list of recent updates to an issue, sorted by date, starting from the most recent.  *  `versionedRepresentations` Instead of `fields`, returns `versionedRepresentations` a JSON array containing each version of a field's value, with the highest numbered item representing the most recent version.  Examples: `\"names,changelog\"` Returns the display name of each field as well as a list of recent updates to an issue.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// A list of fields to return for each issue. Use it to retrieve a subset of fields. This parameter accepts a comma-separated list. Expand options include:   *  `*all` Returns all fields.  *  `*navigable` Returns navigable fields.  *  `id` Returns only issue IDs.  *  Any issue field, prefixed with a dash to exclude.  The default is `id`.  Examples:   *  `summary,comment` Returns the summary and comments fields only.  *  `*all,-comment` Returns all fields except comments.  Multiple `fields` parameters can be included in a request.  Note: By default, this resource returns IDs only. This differs from [GET issue](#api-rest-api-3-issue-issueIdOrKey-get) where the default is all fields.
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    /// Reference fields by their key (rather than ID). The default is `false`.
    #[serde(rename = "fieldsByKeys", skip_serializing_if = "Option::is_none")]
    pub fields_by_keys: Option<bool>,
    /// A [JQL](https://confluence.atlassian.com/x/egORLQ) expression. For performance reasons, this parameter requires a bounded query. A bounded query is a query with a search restriction.   *  Example of an unbounded query: `order by key desc`.  *  Example of a bounded query: `assignee = currentUser() order by key`.  Additionally, `orderBy` clause can contain a maximum of 7 fields.
    #[serde(rename = "jql", skip_serializing_if = "Option::is_none")]
    pub jql: Option<String>,
    /// The maximum number of items to return. Depending on search criteria, real number of items returned may be smaller. It returns max 5000 issues
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The token for a page to fetch that is not the first page. The first page has a `nextPageToken` of `null`. Use the `nextPageToken` to fetch the next page of issues.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// A list of up to 5 issue properties to include in the results. This parameter accepts a comma-separated list.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<String>>,
    /// Strong consistency issue ids to be reconciled with search results. Accepts max 50 ids. All issues must exist.
    #[serde(rename = "reconcileIssues", skip_serializing_if = "Option::is_none")]
    pub reconcile_issues: Option<Vec<i64>>,
}

impl SearchAndReconcileRequestBean {
    pub fn new() -> SearchAndReconcileRequestBean {
        SearchAndReconcileRequestBean {
            expand: None,
            fields: None,
            fields_by_keys: None,
            jql: None,
            max_results: None,
            next_page_token: None,
            properties: None,
            reconcile_issues: None,
        }
    }
}

