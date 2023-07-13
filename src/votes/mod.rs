mod data;

use self::data::HourlyParams;
use self::data::ListParams;
use self::data::Reply;
use self::data::ReplyItem;
use crate::database::Database;
use crate::database::VoteData;
use crate::votes::data::HourRow;
use std::convert::Infallible;

const HOUR_SECONDS: i64 = 3_600;

macro_rules! no_fail {
    ($message:expr, $callback:expr) => {
        match $callback {
            Ok(result) => result,
            Err(error) => {
                warn!("{}: {}", $message, error);

                let message = format!("{}", error);

                return Ok(warp::reply::json(&Reply::error(message)));
            }
        }
    };
}

pub async fn list(params: ListParams, database: Database) -> Result<impl warp::Reply, Infallible> {
    info!("List: id = {}", params.id());

    let items = no_fail!(
        "Failed to read votes",
        database.initiative_votes(params.id()).await
    );
    let items = items
        .into_iter()
        .filter_map(|item| ReplyItem::try_from(item).ok())
        .collect();

    Ok(warp::reply::json(&Reply::list(items)))
}

pub async fn hourly(
    params: HourlyParams,
    database: Database,
) -> Result<impl warp::Reply, Infallible> {
    info!("Hourly: id = {}, hours = {}", params.id(), params.hours());

    let items = no_fail!(
        "Failed to read votes",
        database.initiative_votes(params.id()).await
    );
    let min_timestamp = items.iter().map(VoteData::timestamp).min();
    let max_timestamp = items.iter().map(VoteData::timestamp).max();
    let result = match (min_timestamp, max_timestamp) {
        (Some(min_timestamp), Some(max_timestamp)) => {
            let max_hours = (max_timestamp - min_timestamp) / HOUR_SECONDS;
            let hours = (params.hours() as i64).min(max_hours);
            let mut rows: Vec<_> = (0..hours)
                .map(|hour| HourRow::empty(max_timestamp - (hours - hour + 1) * HOUR_SECONDS))
                .collect();

            for item in items {
                let hours_ago = (max_timestamp - item.timestamp()) / HOUR_SECONDS;

                if hours_ago < hours {
                    let index = (hours - hours_ago - 1) as usize;

                    rows[index].merge(item.positive(), item.negative());
                }
            }

            // Set skipped hours to previous values.
            for index in 0..hours as usize - 1 {
                let positive = rows[index].positive();
                let negative = rows[index].negative();

                rows[index + 1].merge(positive, negative);
            }

            rows.into_iter()
                .filter_map(|item| ReplyItem::try_from(item).ok())
                .collect()
        }
        _ => Default::default(),
    };

    Ok(warp::reply::json(&Reply::list(result)))
}
