pub fn log_request(data: &String) -> std::io::Result<()> {
    let _ = std::fs::create_dir_all(Path::new("requests/"));
    let mut file = File::create(format!("requests/request_{}.json", Utc::now().format("%m-%d_%H-%M-%S")))?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn root_url(url: &str) -> String {
    let idx = url.replace("https://", "").find('/').unwrap();
    url.chars().take(8 + idx).collect()
}

pub fn extract_event_name(event: &str) -> String {
    let mut chars: Vec<char> = event.replace('_', " ").replace("jira:", "").chars().collect();
    chars[0] = chars[0].to_uppercase().next().unwrap();
    chars.into_iter().collect()
}
