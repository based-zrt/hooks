use std::io::{prelude::*, BufReader};
use std::{env, fs::File, path::Path};

use actix_web::{get, web, HttpResponse, Responder};
use anyhow::{bail, Result};
use reqwest::StatusCode;
use serde_json::{json, Value};
use sha1::{Digest, Sha1};

#[get("/img/{hash}")]
async fn serve_image(path: web::Path<String>) -> impl Responder {
    let hash = path.into_inner();
    let filename = format!("img/{}", hash);
    if !Path::new(filename.as_str()).exists() {
        return HttpResponse::NotFound().finish();
    }

    let v: Value = serde_json::from_reader(BufReader::new(File::open("img/content_type.json").unwrap())).unwrap();

    let content_type = &v[&filename].as_str().unwrap();

    HttpResponse::build(StatusCode::OK)
        .content_type(content_type.to_string())
        .body(std::fs::read(&filename).unwrap())
}

pub async fn store(url: &String) -> Result<String> {
    let email = env::var("JIRA_EMAIL").unwrap_or("".to_string());
    let token = env::var("JIRA_REST_TOKEN").unwrap_or("".to_string());

    if email.is_empty() || token.is_empty() {
        bail!("image hosting variables not configured");
    }

    let client = reqwest::Client::new();
    let res = client.get(url).basic_auth(email, Some(token)).send().await?;

    let content_type = res.headers().get("content-type").unwrap().as_bytes().to_vec().clone();
    let bytes = res.bytes().await?;

    let mut hasher = Sha1::new();
    hasher.update(url);
    let filename = format!("img/{:X}", hasher.finalize());

    save_file(&filename, std::str::from_utf8(&content_type).unwrap(), &bytes)?;
    Ok(filename)
}

fn save_file(name: &str, ctype: &str, b: &web::Bytes) -> Result<()> {
    std::fs::create_dir_all(Path::new(name).parent().unwrap())?;

    let mut image_file = File::create(name)?;
    image_file.write_all(b)?;
    let filename = "img/content_type.json";

    if Path::new(filename).exists() {
        let mut v: Value = serde_json::from_reader(BufReader::new(File::open(filename)?))?;
        v.as_object_mut()
            .unwrap()
            .insert(name.to_string(), Value::String(ctype.to_string()));

        let mut file = File::create(filename)?;
        file.write_all(v.to_string().as_bytes())?;
    } else {
        let mut file = File::create(filename)?;
        file.write_all(json!({name: ctype}).to_string().as_bytes())?;
    }
    Ok(())
}
