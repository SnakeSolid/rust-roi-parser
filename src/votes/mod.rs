mod data;

use self::data::HourRow;
use self::data::HourlyParams;
use self::data::ListParams;
use self::data::Reply;
use self::data::ReplyItem;
use crate::database::Database;
use crate::database::VoteData;
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
            let max_hours = (max_timestamp - min_timestamp - 1) / HOUR_SECONDS;
            let hours = (params.hours() as i64).min(max_hours);
            let start_timestamp = max_timestamp - hours * HOUR_SECONDS;
            let mut rows: Vec<_> = (0..hours)
                .map(|hour| HourRow::empty(start_timestamp + hour * HOUR_SECONDS))
                .collect();

println!("items = {:?}", items);


            for item in items {
                if item.timestamp() > start_timestamp {
                    let index = ((item.timestamp() - start_timestamp - 1) / HOUR_SECONDS) as usize;

                    rows[index].merge(item.positive(), item.negative());
                }
            }

println!("rows = {:?}", rows);


            make_monotonic(&mut rows);

            rows.into_iter()
                .filter_map(|item| ReplyItem::try_from(item).ok())
                .collect()
        }
        _ => Default::default(),
    };

    Ok(warp::reply::json(&Reply::list(result)))
}

fn make_monotonic(data: &mut [HourRow]) {
    let mut iter = data.iter_mut();

    if let Some(row) = iter.next() {
        let mut positive = row.positive();
        let mut negative = row.negative();

        while let Some(row) = iter.next() {
            row.merge(positive, negative);
            positive = row.positive();
            negative = row.negative();
        }
    }
}
