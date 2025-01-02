mod database;
mod settings;
mod user;

use crate::database::Database;
use crate::settings::Settings;
use crate::user::{controller, UserRepository, UserService};
use actix_web::{middleware, web, App, HttpServer};
use tracing::Level;
use tracing_subscriber::EnvFilter;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .init();

    let settings = match Settings::new() {
        Ok(settings) => settings,
        Err(e) => {
            println!("{}", e.to_string());
            std::process::exit(1);
        }
    };

    let pool = match Database::new(settings.db_addr()).await {
        Ok(db) => db.get_pool(),
        Err(e) => {
            println!("{}", e.to_string());
            std::process::exit(1);
        }
    };

    let repository = UserRepository::new(pool.clone());
    let service = UserService::new(repository, settings.clerk_config());

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(service.clone()))
            .service(
                web::scope("/api")
                    .route("/user", web::get().to(controller::list_users))
                    .route("/user", web::post().to(controller::login_or_register)),
            )
    })
    .bind(settings.server_addr())?
    .run()
    .await
}
