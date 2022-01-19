use crate::logger::Logger;

mod logger;
mod service;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let logger = Logger::new(Some("Main"));
    logger.info("Do you think Naja is still alive? He's probably alive, and he's tough.");

    service::run().await;
}
