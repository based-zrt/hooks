use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::{env, path::Path};

use actix_web::{
    post,
    web::{self, Data},
    HttpResponse, Responder,
};
use anyhow::Result;
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;
use webhook::models::{Embed, Message};

use crate::types::Comment;
use crate::util::{extract_event_name, log_request, root_url};
use crate::{imgstore, types::JiraData, Clients};

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

    if body.contains("issue_property_set") {
        return HttpResponse::Accepted().finish();
    }

    if env::var("LOG_REQUESTS").unwrap_or("".to_string()) != "" {
        let _ = log_request(&body);
    }

    let j: JiraData = serde_json::from_str(&body).unwrap();

    let _ = clients
        .jira_client
        .send_message(&create_message(j).await.unwrap())
        .await;

    HttpResponse::Accepted().finish()
}

async fn create_message(data: JiraData) -> Result<Message> {
    let user = data.user.as_ref();
    let event_name = extract_event_name(&data.webhook_event);

    let mut msg: Message = Message::new();
    msg.username("Jira");

    let mut embed = Embed::new();
    embed.title(&event_name);

    if data.user.is_some() {
        embed
            .author(
                &user.unwrap().display_name,
                None,
                user.unwrap().avatar_urls.get("48x48").cloned(),
            )
            .url(&root_url(&user.unwrap().self_url));
    }

    if data.issue.is_some() {
        let host_url = env::var("HOST_URL").expect("Missing host url value");
        let issue_type_url = format!(
            "{}/{}",
            host_url,
            imgstore::store(&data.issue.as_ref().unwrap().fields.issue_type.icon_url).await?
        );
        let project_avatar_url = format!(
            "{}/{}",
            host_url,
            imgstore::store(
                data.issue
                    .as_ref()
                    .unwrap()
                    .fields
                    .project
                    .avatar_urls
                    .get("48x48")
                    .unwrap(),
            )
            .await?
        );
        decorate_issue_embed(&mut embed, &data, issue_type_url, project_avatar_url);
    }

    msg.embeds.push(embed);

    if data.comment.is_some() {
        let mut comment_embed = Embed::new();
        decorate_comment_embed(&mut comment_embed, data.comment.as_ref().unwrap());
        msg.embeds.push(comment_embed);
    }

    Ok(msg)
}

fn decorate_issue_embed(e: &mut Embed, data: &JiraData, issue_img: String, project_img: String) {
    let i = data.issue.as_ref().unwrap();
    let project_url = root_url(&i.self_url);
    let f = &i.fields;
    e.footer(&f.project.name, Some(project_img));

    match data.webhook_event.as_str() {
        "jira:issue_created" => {
            e.thumbnail(&issue_img)
                .description(
                    format!(
                        "[`{}`]({}) **{}**\n```\n{}\n```",
                        i.key,
                        project_url,
                        f.summary,
                        f.description.as_ref().unwrap_or(&"".to_string())
                    )
                    .as_str(),
                )
                .field("Type", &f.issue_type.name, false)
                .field("Priority", &f.priority.name, false)
                .color(ISSUE_CREATED);
        }
        "jira:issue_updated" | "comment_created" => {
            e.thumbnail(&issue_img)
                .description(format!("[`{}`]({}) **{}**\n", i.key, project_url, f.summary).as_str())
                .color(ISSUE_UPDATED);

            if data.changelog.is_some() {
                for item in &data.changelog.as_ref().unwrap().items {
                    e.field(&item.field, "", false)
                        .field(
                            "From",
                            item.from_string.as_ref().unwrap_or(&"(unknown)".to_string()),
                            true,
                        )
                        .field("To", item.to_string.as_ref().unwrap_or(&"(unknown)".to_string()), true);
                }
            }
        }
        "jira:issue_deleted" => {
            e.thumbnail(&issue_img)
                .description(format!("[`{}`]({}) **{}**\n", i.key, project_url, f.summary).as_str())
                .color(ISSUE_DELETED);
        }
        _ => {
            e.color(UNSPECIFIED_EVENT);
        }
    }
}

fn decorate_comment_embed(e: &mut Embed, c: &Comment) {
    e.author(
        c.author.display_name.as_str(),
        None,
        c.author.avatar_urls.get("48x48").cloned(),
    )
    .description(c.body.as_str());
}
