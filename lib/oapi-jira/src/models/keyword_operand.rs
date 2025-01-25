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

/// KeywordOperand : An operand that is a JQL keyword. See [Advanced searching - keywords reference](https://confluence.atlassian.com/jiracorecloud/advanced-searching-keywords-reference-765593717.html#Advancedsearching-keywordsreference-EMPTYEMPTY) for more information about operand keywords.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeywordOperand {
    /// The keyword that is the operand value.
    #[serde(rename = "keyword")]
    pub keyword: Keyword,
}

impl KeywordOperand {
    /// An operand that is a JQL keyword. See [Advanced searching - keywords reference](https://confluence.atlassian.com/jiracorecloud/advanced-searching-keywords-reference-765593717.html#Advancedsearching-keywordsreference-EMPTYEMPTY) for more information about operand keywords.
    pub fn new(keyword: Keyword) -> KeywordOperand {
        KeywordOperand {
            keyword,
        }
    }
}
/// The keyword that is the operand value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Keyword {
    #[serde(rename = "empty")]
    Empty,
}

impl Default for Keyword {
    fn default() -> Keyword {
        Self::Empty
    }
}

