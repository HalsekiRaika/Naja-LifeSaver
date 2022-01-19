use std::sync::Arc;
use once_cell::sync::Lazy;
use serenity::client::{Context, EventHandler, RawEventHandler};
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::event::Event;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use tokio_cron_scheduler::{Job, JobScheduler};
use crate::Logger;

pub(crate) struct CmdHandler;
pub(crate) struct ReportHandler;

const STILL_ALIVE: &str = "!still-alive";

static REPORT_CHANNEL: Lazy<ChannelId> = Lazy::new(|| {
    ChannelId(dotenv::var("REPORT_CHANNEL").ok()
        .and_then(|c| c.parse::<u64>().ok())
        .unwrap())
});

#[serenity::async_trait]
impl EventHandler for CmdHandler {
    async fn message(&self, ctx: Context, msg: Message) {

    }

    async fn ready(&self, ctx: Context, ready: Ready) {

    }
}

#[allow(unused_must_use)]
#[serenity::async_trait]
impl RawEventHandler for ReportHandler {
    async fn raw_event(&self, ctx: Context, _ev: Event) {
        let mut scd  = JobScheduler::new();
        let logger   = Logger::new(Some("Report"));
        let ctx_http = Arc::clone(&ctx.http);
        scd.add(Job::new_async("0/5 * * * * * *", move |_uuid, _lock| {
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
    if let Err(e) = channel.say(&ctx_http, "fire! 三:fire:").await {
        println!("cannot send message! -- {:?}", e);
    };
}