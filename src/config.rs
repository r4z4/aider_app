// config.rs

use actix_web::HttpRequest;

pub struct AuthorizedUser {
    // Define your struct fields here
}

pub fn authorize_user(req: &HttpRequest) -> Option<AuthorizedUser> {
    // Implement your logic to validate the user cookie and make a call to the postgres database instance
    // Return an instance of AuthorizedUser if the user is authorized, otherwise return None
    unimplemented!()
}
