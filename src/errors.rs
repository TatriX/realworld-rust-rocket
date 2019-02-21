use rocket::http::Status;
use rocket::request::Request;
use rocket::response::status;
use rocket::response::{self, Responder};
use rocket_contrib::json::Json;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::ops::{Deref, DerefMut};
use validator::ValidationErrors;

#[derive(Debug, Serialize)]
pub struct Errors {
    pub errors: ValidationErrors,
}

impl<'r> Responder<'r> for Errors {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        // TODO: get rid of allocations
        let mut errors = HashMap::new();
        for (field, ers) in self.errors.field_errors() {
            errors.insert(
                field,
                ers.into_iter()
                    .map(|err| err.description().to_owned())
                    .collect::<Vec<_>>(),
            );
        }
        status::Custom(
            Status::UnprocessableEntity,
            Json(json!({ "errors": errors })),
        )
        .respond_to(req)
    }
}

impl Errors {
    #[allow(dead_code)]
    pub fn new() -> Errors {
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

impl DerefMut for Errors {
    fn deref_mut(&mut self) -> &mut ValidationErrors {
        &mut self.errors
    }
}
