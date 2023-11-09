use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraData {
    pub timestamp: i64,
    pub webhook_event: String,
    pub user: Option<User>,
    pub issue: Option<Issue>,
    pub issue_event_type_name: Option<String>,
    pub changelog: Option<Changelog>,
    pub comment: Option<Comment>,
    pub sprint: Option<Sprint>,
}

#[derive(Deserialize)]
pub struct Issue {
    #[serde(rename = "self")]
    pub self_url: String,
    pub id: String,
    pub key: String,
    pub fields: IssueFields,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueFields {
    pub creator: Option<User>,
    pub project: Project,
    pub summary: String,
    pub description: Option<String>,
    pub priority: IssuePriority,
    #[serde(rename = "issuetype")]
    pub issue_type: IssueType,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssuePriority {
    #[serde(rename = "self")]
    pub self_url: String,
    pub icon_url: String,
    pub name: String,
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueType {
    #[serde(rename = "self")]
    pub self_url: String,
    pub id: String,
    pub description: String,
    pub icon_url: String,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "self")]
    pub self_url: String,
    pub account_id: String,
    pub avatar_urls: HashMap<String, String>,
    pub display_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    #[serde(rename = "self")]
    pub self_url: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub avatar_urls: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct Changelog {
    pub id: String,
    pub items: Vec<ChangelogItem>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangelogItem {
    pub field: String,
    pub field_id: String,
    pub from_string: Option<String>,
    pub to_string: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    #[serde(rename = "self")]
    pub self_url: String,
    pub id: String,
    pub author: User,
    pub body: String,
    pub update_author: User,
    pub created: String,
    pub updated: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sprint {
    #[serde(rename = "self")]
    pub self_url: String,
    pub id: i32,
    pub state: String,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub goal: String,
}
