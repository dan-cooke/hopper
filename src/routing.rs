use hyper::{header, Body, Response, StatusCode};

pub fn redirect_to(response: &mut Response<Body>, url: &str) {
    // Create a 301 redirect with Location header
    response.headers_mut().insert(
        "Location",
        header::HeaderValue::from_str(url).expect("Issue converting header"),
    );

    *response.status_mut() = StatusCode::MOVED_PERMANENTLY;
}
