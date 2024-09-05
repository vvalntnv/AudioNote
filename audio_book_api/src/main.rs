mod database;
mod books;
mod generics;
mod stream;
mod audio;
mod auth;  

use std::env;

use actix_web::{self, web, App, HttpResponse, HttpServer};
use books::services::books_scope;
use stream::services::stream_scope;
use database::connector::DatabaseConnection;
use dotenv;

struct AppState {
    db: DatabaseConnection,
    secret: String
}

async fn healthpoint() -> HttpResponse {
    HttpResponse::Ok().body("I am ALIVE!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").ok();

    let app_state = AppState {
        db: DatabaseConnection::from_env().await,
        secret: env::var("SECRET").unwrap()
    };
    let app_state = web::Data::new(app_state);

    println!("Are bate bachka");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(healthpoint))
            .service(
                web::scope("/books").configure(books_scope)
            )
            .service(
                web::scope("/stream").configure(stream_scope)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await 
}
