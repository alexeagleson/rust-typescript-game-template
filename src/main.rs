mod api;
mod database;
mod game;
mod websocket;

use api::MapDimensions;
use database::{Database, DatabaseLock};
use game::{
    map::{DEFAULT_MAP_HEIGHT, DEFAULT_MAP_WIDTH},
    world::WorldLock,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;
use websocket::{connections::ConnectionsLock, new_connection::handle_new_connection};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Database setup
    // Initiate a connection to the database file, creating the file if required.
    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");

    // Run migrations, which updates the database's schema to the latest version.
    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run database migrations");

    let db: DatabaseLock = Arc::new(RwLock::new(Database(database)));

    let db = warp::any().map(move || db.clone());

    // Websocket setup
    let connections = ConnectionsLock::default();
    let connections = warp::any().map(move || connections.clone());

    let world = WorldLock::default();
    let world = warp::any().map(move || world.clone());

    // GET /game -> websocket upgrade
    let game = warp::path!("api" / "game")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .and(connections)
        .and(world)
        .and(db)
        .map(
            |ws: warp::ws::Ws, connections: ConnectionsLock, world: WorldLock, db: DatabaseLock| {
                // This will call our function if the handshake succeeds.
                ws.on_upgrade(move |socket| handle_new_connection(socket, connections, db, world))
            },
        );

    let any_origin_get = warp::cors().allow_any_origin().allow_method("GET");

    // GET /game-config returns a `200 OK` with a JSON array of ids:
    let game_config = warp::path!("api" / "game-config")
        .map(|| {
            warp::reply::json(&MapDimensions {
                width: DEFAULT_MAP_WIDTH,
                height: DEFAULT_MAP_HEIGHT,
            })
        })
        .with(any_origin_get);

    // // GET / -> index html
    // let index = warp::path::end()
    //     .map(|| warp::reply::html(r#"<html>There is nothing to see here.</html>"#));

    // Serve static directory -- not currently used
    let index = warp::fs::dir("client/dist");

    let routes = index.or(game_config).or(game);

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
