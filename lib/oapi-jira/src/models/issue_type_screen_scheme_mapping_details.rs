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

/// IssueTypeScreenSchemeMappingDetails : A list of issue type screen scheme mappings.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeScreenSchemeMappingDetails {
    /// The list of issue type to screen scheme mappings. A *default* entry cannot be specified because a default entry is added when an issue type screen scheme is created.
    #[serde(rename = "issueTypeMappings")]
    pub issue_type_mappings: Vec<models::IssueTypeScreenSchemeMapping>,
}

impl IssueTypeScreenSchemeMappingDetails {
    /// A list of issue type screen scheme mappings.
    pub fn new(issue_type_mappings: Vec<models::IssueTypeScreenSchemeMapping>) -> IssueTypeScreenSchemeMappingDetails {
        IssueTypeScreenSchemeMappingDetails {
            issue_type_mappings,
        }
    }
}

