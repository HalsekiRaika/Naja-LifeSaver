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

const ALIVE_COMMAND: &str = "!alive";

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
        let ctx_http = Arc::clone(&ctx.http);
        match ev {
            Event::Ready(a) => {
                Instant::t_name("Login").out("Info", yansi::Color::Cyan, format!("Connected to {}", a.ready.user.name));
                let mut scd  = JobScheduler::new();
                let logger   = Logger::new(Some("Report"));
                Instant::t_name("Scheduler").out("Info", yansi::Color::Cyan, format!("Build Schedule Task."));
                scd.add(Job::new_async("* 0/59 * * * * *", move |_uuid, _lock| {
                    let ctx_http = ctx_http.clone();
                    let logger   = logger.clone();
                    Box::pin(async move {
                        logger.info("fire!");
                        let report = *REPORT_CHANNEL;
                        on_report(ctx_http, report).await
                    })
                }).unwrap());
                Instant::t_name("Scheduler").out("Info", yansi::Color::Cyan, format!("Built"));
                tokio::spawn(scd.start());
                return;
            },
            Event::Resumed(r) => {
                for trace in r.trace {
                    Instant::t_name("Connect Resume")
                        .out("Info", yansi::Color::Cyan, format!("Trace {}", trace.unwrap_or(String::from(""))));
                }
                return;
            },
            Event::MessageCreate(m) => {
                let channel = *REPORT_CHANNEL;
                if !m.message.author.bot && (m.message.content == ALIVE_COMMAND || m.message.channel_id == channel) {
                    if let Err(e) = channel.say(&*ctx_http.clone(), ":white_check_mark: Alive").await {
                        Instant::t_name("Cmd [Alive]").out("Error", yansi::Color::Red, "Cannot send cmd msg.")
                    }
                }
                return;
            },
            _ => return ()
        };
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
                println!("Cannot send message! -- {:?}", e);
            };
        }
    }
}