use std::fmt::Formatter;
use std::marker::PhantomData;
use std::str::FromStr;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use serde::de::{Error,Visitor};

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
    #[serde(deserialize_with = "must_string_contents")]
    #[serde(rename(deserialize = "text"))]
    text: TextContents
}

#[derive(serde::Deserialize, Clone)]
pub(crate) struct TextContents(pub String);

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

impl FromStr for TextContents {
    type Err = void::Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

pub(crate) fn must_string_contents<'de, T, D>(deserializer: D) -> Result<T, D::Error>
  where T: Deserialize<'de> + FromStr<Err = void::Void>,
        D: Deserializer<'de>
{
    struct MustStringContents<T>(PhantomData<fn() -> T>);
    impl<'de, T> Visitor<'de> for MustStringContents<T>
      where T: Deserialize<'de> + FromStr<Err = void::Void>
    {
        type Value = T;
        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("must string contents")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
          where E: Error
        {
            Ok(FromStr::from_str(v).unwrap())
        }
    }
    deserializer.deserialize_any(MustStringContents(PhantomData))
}