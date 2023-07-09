use crate::database::Database;
use crate::database::InitiativeData;
use crate::roi::RoiClient;
use log::warn;
use std::future::Future;
use std::time::Duration;
use std::time::Instant;
use time::OffsetDateTime;

macro_rules! log_warning {
    ($message: expr, $value: expr) => {
        match $value {
            Ok(result) => result,
            Err(error) => {
                warn!("{}: {}", $message, error);

                return;
            }
        }
    };
    ($message: expr, $value: expr, unwrap) => {
        if let Err(error) = $value {
            warn!("{}: {}", $message, error);
        }
    };
}

async fn load_and_save_initiatives(client: &RoiClient, database: &Database, data: &InitiativeData) {
    let id = data.initiative_id();
    let initiative = log_warning!(
        format!("Failed to load initiative #{}", id),
        client.load(id).await
    );
    let timestamp = OffsetDateTime::now_utc().unix_timestamp();
    let positive = initiative.positive();
    let negative = initiative.negative();

    log_warning!(
        "Failed to save vote",
        database.save_vote(id, timestamp, positive, negative).await,
        unwrap
    );

    if initiative.is_archived() != data.is_archived() {
        log_warning!(
            "Failed to archive initiative",
            database.archive_initiative(id).await,
            unwrap
        );
    }
}

async fn update_active_initiatives(client: &RoiClient, database: &Database, interval: Duration) {
    info!("Update active initiatives");

    let initiatives = log_warning!(
        "Failed to read active initiatives",
        database.active_initiatives().await
    );

    for data in initiatives {
        load_and_save_initiatives(client, database, &data).await;

        tokio::time::sleep(interval).await;
    }
}

pub fn start(
    query_interval: u64,
    update_interval: u64,
    client: RoiClient,
    database: Database,
) -> impl Future<Output = ()> + Send + Sync {
    async move {
        let query_interval = Duration::from_secs(query_interval);
        let update_interval = Duration::from_secs(update_interval);

        loop {
            let start = Instant::now();

            update_active_initiatives(&client, &database, query_interval).await;

            let interval = start.elapsed();

            if update_interval > interval {
                let delta = update_interval - interval;

                tokio::time::sleep(delta).await;
            }
        }
    }
}
