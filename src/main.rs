fn main() {
    // Create a very basic HTTP web server using actix_web that will connect to a postgres database instance
    
    // 1. Set up a postgres database connection using the credentials of PG_USER = postgres and PG_PASS = postgres and PG_DATABASE = aider_app
    // 2. In the main function, be sure to set up a AppData struct for our database config. It will be of type web::Data::new() and then it will pass whatever our struct name is
    // 3. Create one route at /index and for now just have it be a todo() that I can fill in later.

    println!("Hello, world!");
}
