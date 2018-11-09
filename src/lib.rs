extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate actix_web;
extern crate dotenv;
extern crate serde_derive;

use actix_web::{http::Method, App, actix::{Actor, SyncContext, Addr, SyncArbiter}};
use diesel::{prelude::*, mysql::MysqlConnection, r2d2::{ConnectionManager, Pool}};
use dotenv::dotenv;
use std::env;

pub mod answers;
pub mod index;
pub mod models;
pub mod schema;

pub struct DbExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct AppState {
        db: Addr<DbExecutor>,
}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!(format!("Error connecting to {}", database_url)))
}

pub fn create_app() -> App<AppState> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

    App::with_state(AppState {db: addr.clone()})
        .resource("/", |r| r.method(Method::GET).f(index::get))
        .resource("/answers", |r| r.method(Method::POST).with(answers::post))
}
