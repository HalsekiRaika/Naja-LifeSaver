use chrono::{DateTime, Utc};

mod report_handler;

static DISCORD_BOT_TOKEN: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(||
    dotenv::var("DISCORD_BOT_TOKEN").ok()
        .unwrap()
);

pub async fn run() {
    let logger = crate::Logger::new(Some("Service"));
    logger.info("Start Service!");
    logger.info("Build Serenity Client framework");
    let frame = serenity::framework::StandardFramework::new();

    logger.info("Build Discord Client (Serenity)");
    let mut client = serenity::Client::builder(&*DISCORD_BOT_TOKEN)
        .framework(frame)
        .raw_event_handler(report_handler::ReportHandler)
        .await.expect("Cannot create client.");

    if let Err(e) = client.start().await {
        logger.error(&format!("Cannot start client! {:?}", e))
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct TimeLine {
    #[serde(rename(deserialize = "data"))]
    data: Tweets,
    #[allow(dead_code)]
    #[serde(rename(deserialize = "meta"))]
    meta: MetaData
}

#[derive(serde::Deserialize, Clone)]
#[serde(transparent)]
pub(crate) struct Tweets(Vec<Tweet>);

#[derive(serde::Deserialize, Clone)]
pub(crate) struct Tweet {
    #[allow(dead_code)]
    #[serde(rename(deserialize = "created_at"))]
    created_at: DateTime<Utc>,
    #[serde(rename(deserialize = "id"))]
    id: String,
    #[serde(rename(deserialize = "text"))]
    text: String
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub(crate) struct MetaData {
    #[serde(rename(deserialize = "newest_id"))]
    newest_id: String,
    #[serde(rename(deserialize = "oldest_id"))]
    oldest_id: String,
    #[serde(rename(deserialize = "result_count"))]
    result_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "next_token"))]
    next_token: Option<String>
}

impl TimeLine {
    pub fn get_tweet(&self) -> Vec<Tweet> {
        self.data.clone().0
    }
}