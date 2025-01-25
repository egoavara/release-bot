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

/// IssueLinkType : This object is used as follows:   *  In the [ issueLink](#api-rest-api-3-issueLink-post) resource it defines and reports on the type of link between the issues. Find a list of issue link types with [Get issue link types](#api-rest-api-3-issueLinkType-get).  *  In the [ issueLinkType](#api-rest-api-3-issueLinkType-post) resource it defines and reports on issue link types.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueLinkType {
    /// The ID of the issue link type and is used as follows:   *  In the [ issueLink](#api-rest-api-3-issueLink-post) resource it is the type of issue link. Required on create when `name` isn't provided. Otherwise, read only.  *  In the [ issueLinkType](#api-rest-api-3-issueLinkType-post) resource it is read only.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The description of the issue link type inward link and is used as follows:   *  In the [ issueLink](#api-rest-api-3-issueLink-post) resource it is read only.  *  In the [ issueLinkType](#api-rest-api-3-issueLinkType-post) resource it is required on create and optional on update. Otherwise, read only.
    #[serde(rename = "inward", skip_serializing_if = "Option::is_none")]
    pub inward: Option<String>,
    /// The name of the issue link type and is used as follows:   *  In the [ issueLink](#api-rest-api-3-issueLink-post) resource it is the type of issue link. Required on create when `id` isn't provided. Otherwise, read only.  *  In the [ issueLinkType](#api-rest-api-3-issueLinkType-post) resource it is required on create and optional on update. Otherwise, read only.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the issue link type outward link and is used as follows:   *  In the [ issueLink](#api-rest-api-3-issueLink-post) resource it is read only.  *  In the [ issueLinkType](#api-rest-api-3-issueLinkType-post) resource it is required on create and optional on update. Otherwise, read only.
    #[serde(rename = "outward", skip_serializing_if = "Option::is_none")]
    pub outward: Option<String>,
    /// The URL of the issue link type. Read only.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

impl IssueLinkType {
    /// This object is used as follows:   *  In the [ issueLink](#api-rest-api-3-issueLink-post) resource it defines and reports on the type of link between the issues. Find a list of issue link types with [Get issue link types](#api-rest-api-3-issueLinkType-get).  *  In the [ issueLinkType](#api-rest-api-3-issueLinkType-post) resource it defines and reports on issue link types.
    pub fn new() -> IssueLinkType {
        IssueLinkType {
            id: None,
            inward: None,
            name: None,
            outward: None,
            param_self: None,
        }
    }
}

