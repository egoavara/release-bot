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

/// IssueEntityProperties : Lists of issues and entity properties. See [Entity properties](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/) for more information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueEntityProperties {
    /// A list of entity property IDs.
    #[serde(rename = "entitiesIds", skip_serializing_if = "Option::is_none")]
    pub entities_ids: Option<Vec<i64>>,
    /// A list of entity property keys and values.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, models::JsonNode>>,
}

impl IssueEntityProperties {
    /// Lists of issues and entity properties. See [Entity properties](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/) for more information.
    pub fn new() -> IssueEntityProperties {
        IssueEntityProperties {
            entities_ids: None,
            properties: None,
        }
    }
}

