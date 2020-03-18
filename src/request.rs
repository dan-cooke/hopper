use crate::{db, error};
use hyper::{Body, Method, Request, Response, StatusCode};
use std::convert::Infallible;

fn handle_get(path: &str) -> Result<String, error::HopperError> {
    let client = db::connect_to_db()?;
    let alias = &path[1..];
    db::find_original_url(client, alias)
}

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match req.method() {
        &Method::GET => {
            match handle_get(req.uri().path()) {
                Ok(original_url) => {
                    crate::routing::redirect_to(&mut response, original_url.as_str());
                }
                Err(db_error) => *response.status_mut() = db_error.status_code,
            }
        }
        &Method::POST => {
            *response.body_mut() = Body::from("THANK U DFOR POSTING ME ");
        }
        _ => {
            *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
        }
    }

    Ok(response)
}
