mod database;
mod books;
mod generics;

use actix_web::{self, web, App, HttpResponse, HttpServer};
use books::services::users_scope;
use database::connector::DatabaseConnection;
use dotenv;

struct AppState {
    db: DatabaseConnection
}

async fn healthpoint() -> HttpResponse {
    HttpResponse::Ok().body("I am ALIVE!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").ok();

    let app_state = AppState {
        db: DatabaseConnection::from_env().await
    };
    let app_state = web::Data::new(app_state);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(healthpoint))
            .service(
                web::scope("/books").configure(users_scope)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await 
}
