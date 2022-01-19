use std::sync::Arc;
use once_cell::sync::Lazy;
use reqwest::Client;
use serenity::client::{Context, RawEventHandler};
use serenity::http::Http;
use serenity::model::event::Event;
use serenity::model::gateway::Activity;
use serenity::model::id::ChannelId;
use tokio_cron_scheduler::{Job, JobScheduler};
use crate::Logger;
use crate::logger::Instant;
use crate::service::TimeLine;

pub(crate) struct ReportHandler;

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| Client::new());
static REPORT_CHANNEL: Lazy<ChannelId> = Lazy::new(|| {
    ChannelId(dotenv::var("REPORT_CHANNEL").ok()
        .and_then(|c| c.parse::<u64>().ok())
        .unwrap())
});
static TWITTER_API: Lazy<String> = Lazy::new(|| {
    dotenv::var("SURVEILLANCE_TARGET").ok()
        .and_then(|s| Some(format!("https://api.twitter.com/2/tweets/search/recent?tweet.fields=created_at&query=from:{}", s)))
        .unwrap_or(String::from("Najaran3"))
});
static TWITTER_API_TOKEN: Lazy<String> = Lazy::new(|| {
    dotenv::var("BEARER_TOKEN").ok()
        .unwrap()
});
static TWEET_URL: Lazy<String> = Lazy::new(|| {
    dotenv::var("SURVEILLANCE_TARGET").ok()
        .and_then(|s| Some(format!("https://twitter.com/{}/status/", s)))
        .unwrap()
});

#[allow(unused_must_use)]
#[serenity::async_trait]
impl RawEventHandler for ReportHandler {
    async fn raw_event(&self, ctx: Context, ev: Event) {
        match ev {
            Event::Ready(a) => {
                Instant::t_name("Login").out("Info", yansi::Color::Cyan, format!("Connected to {}", a.ready.user.name));
                let a = Activity::listening("Waiting for Naja alive report...");
                ctx.set_activity(a).await;
            },
            _ => ()
        };
        let mut scd  = JobScheduler::new();
        let logger   = Logger::new(Some("Report"));
        let ctx_http = Arc::clone(&ctx.http);
        scd.add(Job::new_async("0/30 * * * * * *", move |_uuid, _lock| {
            let ctx_http = ctx_http.clone();
            let logger   = logger.clone();
            Box::pin(async move {
                logger.info("fire!");
                let report = *REPORT_CHANNEL;
                on_report(ctx_http, report).await
            })
        }).unwrap());
        scd.start().await;
    }
}

async fn on_report(ctx_http: Arc<Http>, channel: ChannelId) {
    let client = &*HTTP_CLIENT;
    let res = client.get(&*TWITTER_API)
        .bearer_auth(&*TWITTER_API_TOKEN)
        .send().await.unwrap()
        .json::<TimeLine>()
        .await.unwrap();
    for tweet in res.get_tweet() {
        if tweet.text.contains("#StillAliveNotify") {
            if let Err(e) = channel.say(&ctx_http, format!("生存報告！{}{}", &*TWEET_URL, tweet.id)).await {
                println!("cannot send message! -- {:?}", e);
            };
        }
    }
}