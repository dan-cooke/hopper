use hyper::{StatusCode};

pub struct HopperError {
    pub status_code: StatusCode
}

impl std::convert::From<mongodb::error::Error> for HopperError {
    fn from(_mongo_error: mongodb::error::Error) -> Self {
        HopperError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}