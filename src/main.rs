use std::env;
use std::net::SocketAddr;

use std::convert::Infallible;
use hyper::{Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::From;

use hopper::request::handle_request;

#[tokio::main]
async fn main(){
     // We'll bind to 127.0.0.1:3000 for now
     let port = env::var("PORT").expect("NO PORT set").parse().unwrap();
     let addr = SocketAddr::from(([127, 0, 0, 1], port));
  
     let make_svc = make_service_fn(|_conn| async {
         // service_fn converts our function into a `Service`
         Ok::<_, Infallible>(service_fn(handle_request))
     });
 
     let server = Server::bind(&addr).serve(make_svc);
 
     // Run this server for... forever!
     if let Err(e) = server.await {
         eprintln!("server error: {}", e);
     }
}
