use clap::{App, Arg};
use dotenv::dotenv;

use std::io::{self, BufRead};

mod message;
use message::Message;

async fn send_msg(
    channel_id: &str,
    username: &str,
) -> Result<(), Box<dyn std::marker::Send + std::marker::Sync + std::error::Error>> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let pool = sqlx::postgres::PgPool::connect(&url).await?;
    let stdin = io::stdin();

    loop {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        if !input.trim().is_empty() {
            let content = input.trim();
            let message = Message::new(username.to_string(), content.to_string());

            let query = format!("NOTIFY {}, '{:?}'", channel_id, message);
            sqlx::query(&query).execute(&pool).await?;

            let formatted = message.created_at.format("%Y-%m-%d %H:%M:%S").to_string();
            println!("{} You :{}", formatted, message.content);
        }
    }
}

async fn read_msg(
    channel_id: &str,
    username: &str,
) -> Result<(), Box<dyn std::marker::Send + std::marker::Sync + std::error::Error>> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let mut listener = sqlx::postgres::PgListener::connect(&url).await?;

    println!("Listening to channel {} as {}", channel_id, username);
    listener.listen(channel_id).await?;

    loop {
        let notification = listener.recv().await?;

        let s = notification.payload();
        let message = Message::from_str(s).unwrap();
        // simple validation to avoid printing the message sent by the user
        if message.username == username {
            continue;
        }
        let formatted = message.created_at.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("{} {}: {}", formatted, message.username, message.content);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::marker::Send + std::marker::Sync + std::error::Error>> {
    let matches = App::new("Pg chat")
        .version("1.0")
        .author("Guilherme Beira <guilherme.vieira.beira@gmail.com>")
        .about("Chat using postgresql")
        .arg(
            Arg::with_name("channel-name")
                .short('c')
                .long("channel-name")
                .value_name("CHANNEL_NAME")
                .help("Sets the channel name")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("username")
                .short('u')
                .long("username")
                .value_name("USERNAME")
                .help("Sets the channel username")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let matches_clone = matches.clone();
    tokio::task::spawn_blocking(move || {
        let channel_name = matches_clone.value_of("channel-name").unwrap_or("default");
        let username = matches_clone.value_of("username").unwrap_or("default");
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(read_msg(channel_name, username))
    });

    let matches_clone = matches.clone();
    tokio::task::spawn_blocking(move || {
        let username = matches_clone.value_of("username").unwrap_or("default");
        let channel_name = matches_clone.value_of("channel-name").unwrap_or("default");
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(send_msg(channel_name, username))
    });

    Ok(())
}
