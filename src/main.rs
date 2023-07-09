#[macro_use]
extern crate log;

mod database;
mod initiatives;
mod options;
mod roi;
mod votes;
mod worker;

use database::Database;
use options::Options;
use roi::RoiClient;
use std::convert::Infallible;
use std::error::Error;
use structopt::StructOpt;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    info!("Parsing options...");
    let options = Options::from_args();

    info!("Setup client...");
    let client = RoiClient::new(options.read_timeout())?;

    info!("Setup database...");
    let database = Database::new(options.database())?;

    info!("Start worker...");
    let worker = tokio::spawn(worker::start(
        options.query_interval(),
        options.update_interval(),
        client.clone(),
        database.clone(),
    ));

    info!("Create routes...");
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("public/initiatives.html"));
    let public = warp::get().and(warp::fs::dir("public"));

    let initiatives_list = warp::path!("api" / "initiatives" / "list")
        .and(warp::post())
        .and(with(database.clone()))
        .and_then(initiatives::list);
    let initiatives_add = warp::path!("api" / "initiatives" / "add")
        .and(warp::post())
        .and(warp::body::json())
        .and(with(client.clone()))
        .and(with(database.clone()))
        .and_then(initiatives::add);
    let initiatives_enable = warp::path!("api" / "initiatives" / "enable")
        .and(warp::post())
        .and(warp::body::json())
        .and(with(database.clone()))
        .and_then(initiatives::enable);
    let initiatives_disable = warp::path!("api" / "initiatives" / "disable")
        .and(warp::post())
        .and(warp::body::json())
        .and(with(database.clone()))
        .and_then(initiatives::disable);
    let initiatives_remove = warp::path!("api" / "initiatives" / "remove")
        .and(warp::post())
        .and(warp::body::json())
        .and(with(database.clone()))
        .and_then(initiatives::remove);
    let votes_list = warp::path!("api" / "votes" / "list")
        .and(warp::post())
        .and(warp::body::json())
        .and(with(database.clone()))
        .and_then(votes::list);

    let routes = index
        .or(initiatives_list)
        .or(initiatives_add)
        .or(initiatives_enable)
        .or(initiatives_disable)
        .or(initiatives_remove)
        .or(votes_list)
        .or(public);

    info!("Starting server...");
    warp::serve(routes)
        .run((options.address().clone(), options.port()))
        .await;
    worker.await?;

    Ok(())
}

fn with<T>(value: T) -> impl Filter<Extract = (T,), Error = Infallible> + Clone
where
    T: Send + Sync + Clone,
{
    warp::any().map(move || value.clone())
}
