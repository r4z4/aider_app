use actix_web::{web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;
use crate::config::authorize_user;
use crate::handlebars_helpers::to_title_case;

mod config;
mod handlebars_helpers;

struct AppData {
    // Define your struct fields here
}

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    // Implement your logic for the /index route here
    let body = hb.render("index", &()).unwrap();
    HttpResponse::Ok().body(body)
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

    // Initialize Handlebars and register the template directory
    let mut hb = Handlebars::new();
    hb.register_templates_directory(".hbs", "./templates")
        .unwrap();
    
    // Register the handlebars helper function
    hb.register_helper("to_title_case", Box::new(to_title_case));

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .app_data(web::Data::new(hb.clone()))
            .route("/index", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
