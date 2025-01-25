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

/// Attachment : Details about an attachment.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Attachment {
    /// Details of the user who added the attachment.
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<models::UserDetails>>,
    /// The content of the attachment.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The datetime the attachment was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The file name of the attachment.
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// The ID of the attachment.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The MIME type of the attachment.
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// The URL of the attachment details response.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The size of the attachment.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// The URL of a thumbnail representing the attachment.
    #[serde(rename = "thumbnail", skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
}

impl Attachment {
    /// Details about an attachment.
    pub fn new() -> Attachment {
        Attachment {
            author: None,
            content: None,
            created: None,
            filename: None,
            id: None,
            mime_type: None,
            param_self: None,
            size: None,
            thumbnail: None,
        }
    }
}

