use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;

use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder,
};
use serde::Deserialize;
use serde_json::json;
use webhook::models::Message;

use crate::{types::JiraData, Clients};

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
async fn handle(
    info: web::Query<Info>,
    body: web::Json<JiraData>,
    raw_body: String,
    clients: Data<Arc<Clients>>,
) -> impl Responder {
    let env_token = match env::var(ENV_KEY) {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"error": "Missing environment value"})),
    };

    if env_token != info.token {
        return HttpResponse::Unauthorized().finish();
    }

    if env::var("DEBUG_REQUESTS").unwrap_or("".to_string()) == "" {
        let _ = log_request(raw_body);
    }

    let _ = clients.jira_client.send_message(&message(&body)).await;

    HttpResponse::Accepted().finish()
}

fn log_request(data: String) -> std::io::Result<()> {
    let mut file = File::open("last_request.json")?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn message(data: &web::Json<JiraData>) -> Message {
    let root_url = root_url(&data.user.self_url);
    let event_name = extract_event_name(&data.webhook_event);
    let mut msg: Message = Message::new();
    msg.username("Jira");
    msg.embed(|embed| {
        embed
            .author(
                &data.user.display_name,
                None,
                data.user.avatar_urls.get("48x48").cloned(),
            )
            .title(&event_name)
            .url(&root_url);

        if data.issue.is_some() {
            let i = data.issue.as_ref().unwrap();
            let f = &i.fields;
            embed
                .thumbnail(&f.issue_type.icon_url)
                .description(
                    format!(
                        "[`{}`]({}) **{}**\n```\n{}\n```",
                        i.key, root_url, f.summary, f.description
                    )
                    .as_str(),
                )
                .field("Type", &f.issue_type.name, false)
                .field("Priority", &f.priority.name, false)
                .footer(&f.project.name, f.project.avatar_urls.get("48x48").cloned());
        }

        match data.webhook_event.as_str() {
            "jira:issue_created" => embed.color(ISSUE_CREATED),
            "jira:issue_updated" => embed.color(ISSUE_UPDATED),
            "jira:issue_deleted" => embed.color(ISSUE_DELETED),
            _ => embed.color(UNSPECIFIED_EVENT),
        }
    });
    msg
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
