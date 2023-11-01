use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;

use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder,
};
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;
use webhook::models::{Embed, Message};

use crate::{
    types::{JiraData, JiraIssue},
    Clients,
};

const ENV_KEY: &str = "JIRA_TOKEN";

const ISSUE_CREATED: &str = "6684530";
const ISSUE_UPDATED: &str = "16759909";
const ISSUE_DELETED: &str = "16738405";
const UNSPECIFIED_EVENT: &str = "6683903";

#[derive(Deserialize)]
struct Info {
    token: String,
}

#[post("/jira")]
async fn handle(info: web::Query<Info>, body: String, clients: Data<Arc<Clients>>) -> impl Responder {
    let env_token = match env::var(ENV_KEY) {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"error": "Missing environment value"})),
    };

    if env_token != info.token {
        return HttpResponse::Unauthorized().finish();
    }

    if env::var("LOG_REQUESTS").unwrap_or("".to_string()) != "" {
        let _ = log_request(&body);
    }

    let j: JiraData = serde_json::from_str(&body).unwrap();
    let _ = clients.jira_client.send_message(&create_message(j)).await;

    HttpResponse::Accepted().finish()
}

fn log_request(data: &String) -> std::io::Result<()> {
    let mut file = File::create(format!("request_{}.json", Utc::now().format("%m-%d_%H-%M-%S")))?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn root_url(url: &str) -> String {
    let idx = url.replace("https://", "").find('/').unwrap();
    url.chars().take(8 + idx).collect()
}

fn extract_event_name(event: &str) -> String {
    let mut chars: Vec<char> = event.replace('_', " ").replace("jira:", "").chars().collect();
    chars[0] = chars[0].to_uppercase().next().unwrap();
    chars.into_iter().collect()
}

fn create_message(data: JiraData) -> Message {
    let root_url = root_url(&data.user.self_url);
    let event_name = extract_event_name(&data.webhook_event);

    let mut msg: Message = Message::new();
    msg.username("Jira");

    let mut embed = Embed::new();
    embed
        .author(
            &data.user.display_name,
            None,
            data.user.avatar_urls.get("48x48").cloned(),
        )
        .title(&event_name)
        .url(&root_url);

    if data.issue.is_some() {
        decorate_issue_embed(&mut embed, data, root_url);
    }
    msg.embeds.push(embed);
    msg
}

fn decorate_issue_embed(e: &mut Embed, data: JiraData, root: String) {
    let i = data.issue.unwrap();
    let f = &i.fields;
    e.footer(&f.project.name, f.project.avatar_urls.get("48x48").cloned());

    match data.webhook_event.as_str() {
        "jira:issue_created" => {
            e.thumbnail(&f.issue_type.icon_url)
                .description(format!("[`{}`]({}) **{}**\n```\n{}\n```", i.key, root, f.summary, f.description).as_str())
                .field("Type", &f.issue_type.name, false)
                .field("Priority", &f.priority.name, false)
                .color(ISSUE_CREATED);
        }
        "jira:issue_updated" => {
            e.description(format!("[`{}`]({}) **{}**\n", i.key, root, f.summary).as_str())
                .color(ISSUE_UPDATED);

            for item in data.changelog.items {
                e.field(&item.field, "", false)
                    .field("From", &item.from_string, true)
                    .field("To", &item.to_string, true);
            }
        }
        "jira:issue_deleted" => {
            e.thumbnail(&f.issue_type.icon_url)
                .description(format!("[`{}`]({}) **{}**\n", i.key, root, f.summary).as_str())
                .color(ISSUE_DELETED);
        }
        _ => {}
    }
}
