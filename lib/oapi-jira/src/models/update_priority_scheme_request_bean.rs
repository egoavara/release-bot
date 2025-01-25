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

/// UpdatePrioritySchemeRequestBean : Details of a priority scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePrioritySchemeRequestBean {
    /// The default priority of the scheme.
    #[serde(rename = "defaultPriorityId", skip_serializing_if = "Option::is_none")]
    pub default_priority_id: Option<i64>,
    /// The description of the priority scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Instructions to migrate the priorities of issues.  `in` mappings are used to migrate the priorities of issues to priorities used within the priority scheme.  `out` mappings are used to migrate the priorities of issues to priorities not used within the priority scheme.   *  When **priorities** are **added** to the priority scheme, no mapping needs to be provided as the new priorities are not used by any issues.  *  When **priorities** are **removed** from the priority scheme, issues that are using those priorities must be migrated to new priorities used by the priority scheme.           *  An `in` mapping must be provided for each of these priorities.  *  When **projects** are **added** to the priority scheme, the priorities of issues in those projects might need to be migrated to new priorities used by the priority scheme. This can occur when the current scheme does not use all the priorities in the project(s)' priority scheme(s).           *  An `in` mapping must be provided for each of these priorities.  *  When **projects** are **removed** from the priority scheme, the priorities of issues in those projects might need to be migrated to new priorities within the **Default Priority Scheme** that are not used by the priority scheme. This can occur when the **Default Priority Scheme** does not use all the priorities within the current scheme.           *  An `out` mapping must be provided for each of these priorities.  For more information on `in` and `out` mappings, see the child properties documentation for the `PriorityMapping` object below.
    #[serde(rename = "mappings", skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Box<models::PriorityMapping>>,
    /// The name of the priority scheme. Must be unique.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The priorities in the scheme.
    #[serde(rename = "priorities", skip_serializing_if = "Option::is_none")]
    pub priorities: Option<models::UpdatePrioritiesInSchemeRequestBean>,
    /// The projects in the scheme.
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<models::UpdateProjectsInSchemeRequestBean>,
}

impl UpdatePrioritySchemeRequestBean {
    /// Details of a priority scheme.
    pub fn new() -> UpdatePrioritySchemeRequestBean {
        UpdatePrioritySchemeRequestBean {
            default_priority_id: None,
            description: None,
            mappings: None,
            name: None,
            priorities: None,
            projects: None,
        }
    }
}

