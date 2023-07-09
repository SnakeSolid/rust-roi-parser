mod data;

use self::data::ListParams;
use self::data::Reply;
use self::data::ReplyItem;
use crate::database::Database;
use std::convert::Infallible;

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
    info!("List");

    let items = no_fail!(
        "Failed to read initiatives",
        database.initiative_votes(params.id()).await
    );
    let items = items
        .into_iter()
        .filter_map(|item| ReplyItem::try_from(item).ok())
        .collect();

    Ok(warp::reply::json(&Reply::list(items)))
}
