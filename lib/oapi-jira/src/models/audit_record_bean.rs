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

/// AuditRecordBean : An audit record.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditRecordBean {
    /// The list of items associated with the changed record.
    #[serde(rename = "associatedItems", skip_serializing_if = "Option::is_none")]
    pub associated_items: Option<Vec<models::AssociatedItemBean>>,
    /// Deprecated, use `authorAccountId` instead. The key of the user who created the audit record.
    #[serde(rename = "authorKey", skip_serializing_if = "Option::is_none")]
    pub author_key: Option<String>,
    /// The category of the audit record. For a list of these categories, see the help article [Auditing in Jira applications](https://confluence.atlassian.com/x/noXKM).
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The list of values changed in the record event.
    #[serde(rename = "changedValues", skip_serializing_if = "Option::is_none")]
    pub changed_values: Option<Vec<models::ChangedValueBean>>,
    /// The date and time on which the audit record was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The description of the audit record.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The event the audit record originated from.
    #[serde(rename = "eventSource", skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,
    /// The ID of the audit record.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "objectItem", skip_serializing_if = "Option::is_none")]
    pub object_item: Option<Box<models::AssociatedItemBean>>,
    /// The URL of the computer where the creation of the audit record was initiated.
    #[serde(rename = "remoteAddress", skip_serializing_if = "Option::is_none")]
    pub remote_address: Option<String>,
    /// The summary of the audit record.
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

impl AuditRecordBean {
    /// An audit record.
    pub fn new() -> AuditRecordBean {
        AuditRecordBean {
            associated_items: None,
            author_key: None,
            category: None,
            changed_values: None,
            created: None,
            description: None,
            event_source: None,
            id: None,
            object_item: None,
            remote_address: None,
            summary: None,
        }
    }
}

