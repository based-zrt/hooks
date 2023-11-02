use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraData {
    pub timestamp: i64,
    pub webhook_event: String,
    pub user: Option<JiraUser>,
    pub issue: Option<JiraIssue>,
    pub issue_event_type_name: Option<String>,
    pub changelog: JiraChangelog,
}

#[derive(Deserialize)]
pub struct JiraIssue {
    #[serde(rename = "self")]
    pub self_url: String,
    pub id: String,
    pub key: String,
    pub fields: JiraIssueFields,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraIssueFields {
    pub creator: JiraUser,
    pub project: JiraProject,
    pub summary: String,
    pub description: String,
    pub priority: JiraIssuePriority,
    #[serde(rename = "issuetype")]
    pub issue_type: JiraIssueType,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraIssuePriority {
    #[serde(rename = "self")]
    pub self_url: String,
    pub icon_url: String,
    pub name: String,
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraIssueType {
    #[serde(rename = "self")]
    pub self_url: String,
    pub id: String,
    pub description: String,
    pub icon_url: String,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraUser {
    #[serde(rename = "self")]
    pub self_url: String,
    pub account_id: String,
    pub avatar_urls: HashMap<String, String>,
    pub display_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraProject {
    #[serde(rename = "self")]
    pub self_url: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub avatar_urls: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct JiraChangelog {
    pub id: String,
    pub items: Vec<JiraChangelogItem>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraChangelogItem {
    pub field: String,
    pub field_id: String,
    pub from_string: Option<String>,
    pub to_string: Option<String>,
}
