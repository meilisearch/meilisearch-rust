use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{guard, App, HttpServer};
use diesel::migration::MigrationSource;
use diesel::{Connection, PgConnection};
use diesel_migrations::FileBasedMigrations;
use meilisearch_ex::{build_schema, index, index_graphiql, app_env_vars::AppEnvVars, errors::ApplicationError};


#[actix_web::main]
async fn main() -> Result<(), ApplicationError> {
    let _ = dotenvy::dotenv();

    let app_env_vars = envy::from_env::<AppEnvVars>()?;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mut db_connection = PgConnection::establish(&app_env_vars.database_url)?;
    let mut migrations = FileBasedMigrations::from_path(&app_env_vars.migrations_dir_path)?
        .migrations()
        .unwrap();

    migrations.sort_by_key(|m| m.name().to_string());

    for migration in migrations {
        migration.run(&mut db_connection).unwrap();
    }

    let schema = build_schema(&app_env_vars)?;

    println!("GraphiQL IDE: http://localhost:8081");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
                    .supports_credentials(),
            )
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await?;

    Ok(())
}
