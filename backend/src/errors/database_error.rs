use {
    actix_web::{error::BlockingError, HttpResponse, ResponseError},
    derive_more::{Display, From},
    diesel::result::Error as DieselError,
};

#[derive(Display, From, Debug)]
pub struct DatabaseError(pub BlockingError<DieselError>);

impl std::error::Error for DatabaseError {}

impl ResponseError for DatabaseError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            DatabaseError(BlockingError::Error(DieselError::NotFound)) => {
                HttpResponse::NotFound().finish()
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
