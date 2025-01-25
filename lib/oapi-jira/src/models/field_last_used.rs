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

/// FieldLastUsed : Information about the most recent use of a field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldLastUsed {
    /// Last used value type:   *  *TRACKED*: field is tracked and a last used date is available.  *  *NOT\\_TRACKED*: field is not tracked, last used date is not available.  *  *NO\\_INFORMATION*: field is tracked, but no last used date is available.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// The date when the value of the field last changed.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl FieldLastUsed {
    /// Information about the most recent use of a field.
    pub fn new() -> FieldLastUsed {
        FieldLastUsed {
            r#type: None,
            value: None,
        }
    }
}
/// Last used value type:   *  *TRACKED*: field is tracked and a last used date is available.  *  *NOT\\_TRACKED*: field is not tracked, last used date is not available.  *  *NO\\_INFORMATION*: field is tracked, but no last used date is available.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "TRACKED")]
    Tracked,
    #[serde(rename = "NOT_TRACKED")]
    NotTracked,
    #[serde(rename = "NO_INFORMATION")]
    NoInformation,
}

impl Default for Type {
    fn default() -> Type {
        Self::Tracked
    }
}

