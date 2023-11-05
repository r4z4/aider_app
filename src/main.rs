use actix_web::{web, App, HttpResponse, HttpServer};
use crate::config::authorize_user;

mod config;

struct AppData {
    // Define your struct fields here
}

async fn index() -> HttpResponse {
    // Implement your logic for the /index route here
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up a postgres database connection using the credentials of PG_USER = postgres and PG_PASS = postgres and PG_DATABASE = aider_app
    let pg_user = "postgres";
    let pg_pass = "postgres";
    let pg_database = "aider_app";

    // Create a web::Data instance for the database config
    let app_data = web::Data::new(AppData {
        // Initialize your struct fields here
    });

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/index", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
