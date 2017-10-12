use std::env;
use persistent::Read;
use iron::typemap::Key;
use diesel::pg::PgConnection;
use r2d2::{Config, Pool};
use r2d2_diesel::ConnectionManager;

#[derive(Clone, Copy)]
pub struct ConnectionPool;
impl Key for ConnectionPool { type Value = Pool<ConnectionManager<PgConnection>>; }

pub fn connection_pool_middleware() -> (Read<ConnectionPool>, Read<ConnectionPool>) {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let config = Config::default();
    let manager =ConnectionManager::new(database_url);
    let pool: Pool<ConnectionManager<PgConnection>> = Pool::new(config, manager)
        .expect("failed to create database pool");

    Read::<ConnectionPool>::both(pool)
}
