use service::tapi_listening_handler;
use crate::logger::Logger;

mod logger;
mod service;

#[tokio::main]
async fn main() {
    let logger = Logger::new(Some("Main"));
    logger.info("Do you think Naja is still alive? He's probably alive, and he's tough.");

    tapi_listening_handler::run().await;

}
