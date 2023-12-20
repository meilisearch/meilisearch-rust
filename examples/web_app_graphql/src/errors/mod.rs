use diesel::ConnectionError;
use diesel_async::pooled_connection::deadpool::{BuildError, PoolError};
use diesel_migrations::MigrationError;
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Missing environment variable")]
    Envy(#[from] envy::Error),
    #[error("Input/Output error")]
    Io(#[from] std::io::Error),
    #[error("Database error")]
    Diesel(#[from] diesel::result::Error),
    #[error("Deadpool build error")]
    DeadpoolBuild(#[from] BuildError),
    #[error("Migration error")]
    Migration(#[from] MigrationError),
    #[error("Connection error")]
    DieselConnection(#[from] ConnectionError),
    #[error("Pool Error")]
    Pool(#[from] PoolError),
    #[error("Serde json error")]
    SerDe(#[from] SerdeError),
}
