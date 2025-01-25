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

/// IssueTransition : Details of an issue transition.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTransition {
    /// Expand options that include additional transition details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// Details of the fields associated with the issue transition screen. Use this information to populate `fields` and `update` in a transition request.
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, models::FieldMetadata>>,
    /// Whether there is a screen associated with the issue transition.
    #[serde(rename = "hasScreen", skip_serializing_if = "Option::is_none")]
    pub has_screen: Option<bool>,
    /// The ID of the issue transition. Required when specifying a transition to undertake.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the transition is available to be performed.
    #[serde(rename = "isAvailable", skip_serializing_if = "Option::is_none")]
    pub is_available: Option<bool>,
    /// Whether the issue has to meet criteria before the issue transition is applied.
    #[serde(rename = "isConditional", skip_serializing_if = "Option::is_none")]
    pub is_conditional: Option<bool>,
    /// Whether the issue transition is global, that is, the transition is applied to issues regardless of their status.
    #[serde(rename = "isGlobal", skip_serializing_if = "Option::is_none")]
    pub is_global: Option<bool>,
    /// Whether this is the initial issue transition for the workflow.
    #[serde(rename = "isInitial", skip_serializing_if = "Option::is_none")]
    pub is_initial: Option<bool>,
    #[serde(rename = "looped", skip_serializing_if = "Option::is_none")]
    pub looped: Option<bool>,
    /// The name of the issue transition.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Details of the issue status after the transition.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<models::StatusDetails>,
}

impl IssueTransition {
    /// Details of an issue transition.
    pub fn new() -> IssueTransition {
        IssueTransition {
            expand: None,
            fields: None,
            has_screen: None,
            id: None,
            is_available: None,
            is_conditional: None,
            is_global: None,
            is_initial: None,
            looped: None,
            name: None,
            to: None,
        }
    }
}

