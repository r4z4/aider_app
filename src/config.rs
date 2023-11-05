// config.rs

use actix_web::HttpRequest;
use regex::Regex;

pub struct AuthorizedUser {
    // Define your struct fields here
}

lazy_static! {
    pub static ref RE_USERNAME: Regex = Regex::new(r"^[a-zA-Z0-9]{4,}$").unwrap();
    pub static ref RE_SPECIAL_CHAR: Regex = Regex::new("^.*?[@$!%*?&].*$").unwrap();
    pub static ref RE_EMAIL: Regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"
    )
    .unwrap();
    pub static ref ACCEPTED_SECONDARIES: Vec<String> = vec![
        "Apt".to_owned(),
        "Apt.".to_owned(),
        "Ste".to_owned(),
        "Ste.".to_owned(),
        "Suite".to_owned(),
        "Apartment".to_owned(),
        "#".to_owned(),
        "No.".to_owned(),
        "No".to_owned()
    ];
    pub static ref ACCEPTED_PRIMARIES: Vec<&'static str> = vec![
        "St.", "St", "Street", "Ave.", "Av.", "Ave", "Avenue", "Parkway", "Pkwy", "Pkwy.", "Dr.",
        "Dr", "Drive", "Ln", "Lane", "Ln."
    ];
}

#[derive(Serialize, Debug)]
pub struct ValidationErrorMap {
    pub key: String
    pub errs: Vec<ValidationError>,
}

#[derive(Serialize)]
pub struct FormErrorResponse {
    pub errors: Option<Vec<ValidationErrorMap>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Post {
    pub slug: String,
    pub title: String
    pub author: String,
    pub date: String,
    pub body: String
}

pub fn authorize_user(req: &HttpRequest) -> Option<AuthorizedUser> {
    // Implement your logic to validate the user cookie and make a call to the postgres database instance
    // Return an instance of AuthorizedUser if the user is authorized, otherwise return None
    unimplemented!()
}
