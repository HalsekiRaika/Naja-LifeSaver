use tokio_cron_scheduler::{Job, JobScheduler};
use yansi::Color;
use crate::Logger;
use crate::logger::Instant;

pub async fn run() {
    let mut scd = JobScheduler::new();
    scd.add(Job::new_async("* * 0/23 * * * *", |_i, _j| Box::pin(async {
        Instant::t_name("t-api").out("Info", Color::Cyan, "exec!")
    })).unwrap());

    scd.start().await;
}

async fn req() {
}