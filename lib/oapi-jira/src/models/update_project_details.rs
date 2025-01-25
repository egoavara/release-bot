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

/// UpdateProjectDetails : Details about the project.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateProjectDetails {
    /// The default assignee when creating issues for this project.
    #[serde(rename = "assigneeType", skip_serializing_if = "Option::is_none")]
    pub assignee_type: Option<AssigneeType>,
    /// An integer value for the project's avatar.
    #[serde(rename = "avatarId", skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    /// The ID of the project's category. A complete list of category IDs is found using the [Get all project categories](#api-rest-api-3-projectCategory-get) operation. To remove the project category from the project, set the value to `-1.`
    #[serde(rename = "categoryId", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    /// A brief description of the project.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the issue security scheme for the project, which enables you to control who can and cannot view issues. Use the [Get issue security schemes](#api-rest-api-3-issuesecurityschemes-get) resource to get all issue security scheme IDs.
    #[serde(rename = "issueSecurityScheme", skip_serializing_if = "Option::is_none")]
    pub issue_security_scheme: Option<i64>,
    /// Project keys must be unique and start with an uppercase letter followed by one or more uppercase alphanumeric characters. The maximum length is 10 characters.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// This parameter is deprecated because of privacy changes. Use `leadAccountId` instead. See the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. The user name of the project lead. Cannot be provided with `leadAccountId`.
    #[serde(rename = "lead", skip_serializing_if = "Option::is_none")]
    pub lead: Option<String>,
    /// The account ID of the project lead. Cannot be provided with `lead`.
    #[serde(rename = "leadAccountId", skip_serializing_if = "Option::is_none")]
    pub lead_account_id: Option<String>,
    /// The name of the project.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The ID of the notification scheme for the project. Use the [Get notification schemes](#api-rest-api-3-notificationscheme-get) resource to get a list of notification scheme IDs.
    #[serde(rename = "notificationScheme", skip_serializing_if = "Option::is_none")]
    pub notification_scheme: Option<i64>,
    /// The ID of the permission scheme for the project. Use the [Get all permission schemes](#api-rest-api-3-permissionscheme-get) resource to see a list of all permission scheme IDs.
    #[serde(rename = "permissionScheme", skip_serializing_if = "Option::is_none")]
    pub permission_scheme: Option<i64>,
    /// Previous project keys to be released from the current project. Released keys must belong to the current project and not contain the current project key
    #[serde(rename = "releasedProjectKeys", skip_serializing_if = "Option::is_none")]
    pub released_project_keys: Option<Vec<String>>,
    /// A link to information about this project, such as project documentation
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl UpdateProjectDetails {
    /// Details about the project.
    pub fn new() -> UpdateProjectDetails {
        UpdateProjectDetails {
            assignee_type: None,
            avatar_id: None,
            category_id: None,
            description: None,
            issue_security_scheme: None,
            key: None,
            lead: None,
            lead_account_id: None,
            name: None,
            notification_scheme: None,
            permission_scheme: None,
            released_project_keys: None,
            url: None,
        }
    }
}
/// The default assignee when creating issues for this project.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssigneeType {
    #[serde(rename = "PROJECT_LEAD")]
    ProjectLead,
    #[serde(rename = "UNASSIGNED")]
    Unassigned,
}

impl Default for AssigneeType {
    fn default() -> AssigneeType {
        Self::ProjectLead
    }
}

