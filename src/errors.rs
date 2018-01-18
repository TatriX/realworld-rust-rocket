use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket_contrib::Json;
use validator::ValidationErrors;
use std::ops::Deref;

#[derive(Serialize)]
pub struct Errors {
    pub errors: ValidationErrors,
}

impl<'r> Responder<'r> for Errors {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Json(self).respond_to(req)
    }
}

impl Errors {
    fn new() -> Errors {
        Errors {
            errors: ValidationErrors::new(),
        }
    }
}

impl Deref for Errors {
    type Target = ValidationErrors;

    fn deref(&self) -> &ValidationErrors {
        &self.errors
    }
}
