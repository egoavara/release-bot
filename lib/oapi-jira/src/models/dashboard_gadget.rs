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

/// DashboardGadget : Details of a gadget.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardGadget {
    /// The color of the gadget. Should be one of `blue`, `red`, `yellow`, `green`, `cyan`, `purple`, `gray`, or `white`.
    #[serde(rename = "color")]
    pub color: Color,
    /// The ID of the gadget instance.
    #[serde(rename = "id")]
    pub id: i64,
    /// The module key of the gadget type.
    #[serde(rename = "moduleKey", skip_serializing_if = "Option::is_none")]
    pub module_key: Option<String>,
    /// The position of the gadget.
    #[serde(rename = "position")]
    pub position: Box<models::DashboardGadgetPosition>,
    /// The title of the gadget.
    #[serde(rename = "title")]
    pub title: String,
    /// The URI of the gadget type.
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl DashboardGadget {
    /// Details of a gadget.
    pub fn new(color: Color, id: i64, position: models::DashboardGadgetPosition, title: String) -> DashboardGadget {
        DashboardGadget {
            color,
            id,
            module_key: None,
            position: Box::new(position),
            title,
            uri: None,
        }
    }
}
/// The color of the gadget. Should be one of `blue`, `red`, `yellow`, `green`, `cyan`, `purple`, `gray`, or `white`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Color {
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "cyan")]
    Cyan,
    #[serde(rename = "purple")]
    Purple,
    #[serde(rename = "gray")]
    Gray,
    #[serde(rename = "white")]
    White,
}

impl Default for Color {
    fn default() -> Color {
        Self::Blue
    }
}

