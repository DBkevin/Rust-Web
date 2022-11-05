use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub enum MyError {
    ActixError(String),
    NotFoud(String),
    TeraError(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl std::error::Error for MyError {}

impl MyError {
    fn errro_response(&self) -> String {
        match self {
            MyError::ActixError(msg) => {
                println!("Server error occurred:{:?}", msg);
                "Internal server eror".into()
            }
            MyError::TeraError(msg) => {
                println!("Error in rendering the Template{:?}", msg);
                msg.into()
            }
            MyError::NotFoud(msg) => {
                println!("Not Fond error occurred:{:?}", msg);
                msg.into()
            }
        }
    }
}
impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::ActixError(_msg) | MyError::TeraError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            MyError::NotFoud(_msg) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response()
        })
    }


}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}
impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}
