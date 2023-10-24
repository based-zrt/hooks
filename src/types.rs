use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraData {
    pub timestamp: i64,
    pub webhook_event: String,
    pub user: JiraUser,
    pub issue: Option<JiraIssue>,
    pub issue_event_type_name: Option<String>,
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
