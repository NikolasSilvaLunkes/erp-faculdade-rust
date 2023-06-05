//! Database-related functions
use crate::config::{Config, CONFIG};
use actix_web::web;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PoolError},
    Connection,
};

#[serde(untagged)]
#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(field_identifier, rename_all = "lowercase")]
pub enum DatabaseConnection {
    Postgres,
}

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;

pub type PostgresPool = Pool<PgConnection>;

#[cfg(feature = "postgres")]
pub type PoolType = PostgresPool;

#[derive(Clone)]
pub enum InferPool {
    Postgres(PostgresPool),
}

impl InferPool {
    pub fn init_pool(config: Config) -> Result<Self, r2d2::Error> {
        match config.database {
            DatabaseConnection::Postgres => {
                init_pool::<PgConnection>(config).map(InferPool::Postgres)
            }
        }
        .map_err(Into::into)
    }
}

pub fn init_pool<T>(config: Config) -> Result<Pool<T>, PoolError>
where
    T: Connection + 'static,
{
    let manager = ConnectionManager::<T>::new(config.database_url);
    Pool::builder().build(manager)
}

pub fn add_pool(cfg: &mut web::ServiceConfig) {
    let pool = InferPool::init_pool(CONFIG.clone()).expect("Failed to create connection pool");
    match pool {
        InferPool::Postgres(postgres_pool) => cfg.data(postgres_pool),
    };
}
