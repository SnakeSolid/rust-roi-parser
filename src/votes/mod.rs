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
            let rows = make_hour_rows(start_timestamp, hours, &items);

            rows.into_iter()
                .filter_map(|item| ReplyItem::try_from(item).ok())
                .collect()
        }
        _ => Default::default(),
    };

    Ok(warp::reply::json(&Reply::list(result)))
}

fn make_hour_rows(from_timestamp: i64, hours: i64, items: &[VoteData]) -> Vec<HourRow> {
    let mut rows: Vec<_> = (0..hours)
        .map(|hour| HourRow::empty(from_timestamp + hour * HOUR_SECONDS))
        .collect();

    fill_hour_rows(from_timestamp, &items, &mut rows);

    rows
}

fn fill_hour_rows(from_timestamp: i64, items: &[VoteData], rows: &mut [HourRow]) {
    let mut has_value: Vec<_> = rows.iter().map(|_| false).collect();

    for item in items {
        if item.timestamp() > from_timestamp {
            let index = ((item.timestamp() - from_timestamp - 1) / HOUR_SECONDS) as usize;

            rows[index].merge(item.positive(), item.negative());
            has_value[index] = true;
        }
    }

    fill_head(&has_value, rows);
    make_monotonic(rows);
}

fn fill_head(has_value: &[bool], data: &mut [HourRow]) {
    match has_value.iter().position(|value| *value) {
        Some(0) => {}
        Some(start) => {
            let mut start = start;
            let row = data[start].clone();

            while start > 0 {
                start -= 1;

                data[start] = row.clone();
            }
        }
        None => {}
    }
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
