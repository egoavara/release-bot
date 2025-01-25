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
pub struct JiraMultiSelectComponentField {
    #[serde(rename = "bulkEditMultiSelectFieldOption")]
    pub bulk_edit_multi_select_field_option: BulkEditMultiSelectFieldOption,
    #[serde(rename = "components")]
    pub components: Vec<models::JiraComponentField>,
    #[serde(rename = "fieldId")]
    pub field_id: String,
}

impl JiraMultiSelectComponentField {
    pub fn new(bulk_edit_multi_select_field_option: BulkEditMultiSelectFieldOption, components: Vec<models::JiraComponentField>, field_id: String) -> JiraMultiSelectComponentField {
        JiraMultiSelectComponentField {
            bulk_edit_multi_select_field_option,
            components,
            field_id,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BulkEditMultiSelectFieldOption {
    #[serde(rename = "ADD")]
    Add,
    #[serde(rename = "REMOVE")]
    Remove,
    #[serde(rename = "REPLACE")]
    Replace,
    #[serde(rename = "REMOVE_ALL")]
    RemoveAll,
}

impl Default for BulkEditMultiSelectFieldOption {
    fn default() -> BulkEditMultiSelectFieldOption {
        Self::Add
    }
}

