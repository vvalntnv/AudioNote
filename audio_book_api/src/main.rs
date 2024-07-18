use actix_web::{self, web, App, HttpResponse, HttpServer};

async fn healthpoint() -> HttpResponse {
    HttpResponse::Ok().body("I am ALIVE!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(healthpoint))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await 
}
