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

/// UserList : A paginated list of users sharing the filter. This includes users that are members of the groups or can browse the projects that the filter is shared with.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserList {
    /// The index of the last item returned on the page.
    #[serde(rename = "end-index", skip_serializing_if = "Option::is_none")]
    pub end_index: Option<i32>,
    /// The list of items.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::User>>,
    /// The maximum number of results that could be on the page.
    #[serde(rename = "max-results", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The number of items on the page.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// The index of the first item returned on the page.
    #[serde(rename = "start-index", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}

impl UserList {
    /// A paginated list of users sharing the filter. This includes users that are members of the groups or can browse the projects that the filter is shared with.
    pub fn new() -> UserList {
        UserList {
            end_index: None,
            items: None,
            max_results: None,
            size: None,
            start_index: None,
        }
    }
}

