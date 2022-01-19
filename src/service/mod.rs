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
        .event_handler(report_handler::CmdHandler)
        .raw_event_handler(report_handler::ReportHandler)
        .await.expect("Cannot create client.");

    if let Err(e) = client.start().await {
        logger.error(&format!("Cannot start client! {:?}", e))
    }
}