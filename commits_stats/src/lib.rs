use std::collections::HashMap;
use chrono::{DateTime, Datelike, Utc};

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut result = HashMap::new();

    for commit in data.members() {
        if let Some(login) = commit["author"]["login"].as_str() {
            *result.entry(login.to_string()).or_insert(0) += 1;
        }
    }

    result
}

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut result = HashMap::new();

    for commit in data.members() {
        if let Some(date) = commit["commit"]["author"]["date"].as_str() {
            if let Ok(datetime) = DateTime::parse_from_rfc3339(date) {
                let datetime: DateTime<Utc> = datetime.with_timezone(&Utc);

                let week = datetime.iso_week();
                let key = format!("{}-W{}", week.year(), week.week());

                *result.entry(key).or_insert(0) += 1;
            }
        }
    }

    result
}