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

/// WebhooksExpirationDate : The date the refreshed webhooks expire.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhooksExpirationDate {
    /// The expiration date of all the refreshed webhooks.
    #[serde(rename = "expirationDate")]
    pub expiration_date: i64,
}

impl WebhooksExpirationDate {
    /// The date the refreshed webhooks expire.
    pub fn new(expiration_date: i64) -> WebhooksExpirationDate {
        WebhooksExpirationDate {
            expiration_date,
        }
    }
}

