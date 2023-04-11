use chrono::{DateTime, Utc};
use regex::Regex;
use std::error::Error;
use uuid::Uuid;

#[derive(Debug)]
pub struct Message {
    pub id: Uuid,
    pub username: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl Message {
    pub fn new(username: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            content,
            created_at: Utc::now(),
        }
    }
    pub fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        let re = Regex::new(
            r#"Message\s*\{\s*id:\s*(?P<id>[\da-fA-F-]+),\s*username:\s*"(?P<username>[^"]+)",\s*content:\s*"(?P<content>[^"]+)",\s*created_at:\s*(?P<created_at>[\dTZ:.-]+)\s*\}"#,
        )?;

        if let Some(captures) = re.captures(s) {
            let id = Uuid::parse_str(&captures["id"])?;
            let username = captures["username"].to_string();
            let content = captures["content"].to_string();
            let created_at =
                DateTime::parse_from_rfc3339(&captures["created_at"])?.with_timezone(&Utc);

            Ok(Self {
                id,
                username,
                content,
                created_at,
            })
        } else {
            Err("Failed to parse message string".into())
        }
    }
}
