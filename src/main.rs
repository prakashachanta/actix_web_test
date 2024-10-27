use actix_web::{middleware::Logger, web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use utils::app_state::AppState;
mod utils;
mod routes;



#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    let port: u16 = (*utils::constants::PORT).clone();
    let address: String= (*utils::constants::ADDRESS).clone();
    let db_url: String = (*utils::constants::DATABASE_URL).clone();

    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    let db:DatabaseConnection = sea_orm::Database::connect(db_url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    dotenv::dotenv().ok();
    env_logger::init();

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(AppState{db: db.clone()})).wrap(Logger::default()).configure(routes::home_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}