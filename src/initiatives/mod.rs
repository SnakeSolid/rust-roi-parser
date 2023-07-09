mod data;

use self::data::AddParams;
use self::data::DisableParams;
use self::data::EnableParams;
use self::data::RemoveParams;
use self::data::Reply;
use self::data::ReplyItem;
use crate::database::Database;
use crate::roi::RoiClient;
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

pub async fn list(database: Database) -> Result<impl warp::Reply, Infallible> {
    info!("List");

    let items = no_fail!(
        "Failed to read initiatives",
        database.all_initiatives().await
    );
    let items = items.into_iter().map(ReplyItem::from).collect();

    Ok(warp::reply::json(&Reply::list(items)))
}

pub async fn enable(
    params: EnableParams,
    database: Database,
) -> Result<impl warp::Reply, Infallible> {
    info!("Enable: parameters = {:?}", params);

    no_fail!(
        "Failed to enable initiative",
        database.enable_initiative(params.id()).await
    );

    Ok(warp::reply::json(&Reply::success()))
}

pub async fn disable(
    params: DisableParams,
    database: Database,
) -> Result<impl warp::Reply, Infallible> {
    info!("Disable: parameters = {:?}", params);

    no_fail!(
        "Failed to disable initiative",
        database.disable_initiative(params.id()).await
    );

    Ok(warp::reply::json(&Reply::success()))
}

pub async fn add(
    params: AddParams,
    client: RoiClient,
    database: Database,
) -> Result<impl warp::Reply, Infallible> {
    info!("Add: parameters = {:?}", params);

    let initiative = no_fail!("Failed to load initiative", client.load(params.id()).await);
    let id = initiative.id();
    let name = initiative.name();
    let is_archived = initiative.is_archived();

    no_fail!(
        "Failed to add initiative",
        database.create_initiative(id, name, is_archived, true).await
    );

    Ok(warp::reply::json(&Reply::success()))
}

pub async fn remove(
    params: RemoveParams,
    database: Database,
) -> Result<impl warp::Reply, Infallible> {
    info!("Remove: parameters = {:?}", params);

    no_fail!(
        "Failed to remove initiative",
        database.remove_initiative(params.id()).await
    );

    Ok(warp::reply::json(&Reply::success()))
}
