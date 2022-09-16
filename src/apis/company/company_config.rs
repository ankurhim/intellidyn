use hyper::Method;
use crate::apis::company::company_routes::{ post, get };
use crate::errors::CustomError;
use lambda_http::{ Response, Body, Request };

pub async fn resolve_routes(method: Method, route: &str, event: Request) -> Result<Response<Body>, CustomError> {
    println!("Resolving Route: {:?}", route);

    match route {
        "/company" => {
            match method {
                Method::POST => post(event).await,
                Method::GET => get(event).await,
                _ => todo!(),
            }
        },
        _ => {
            todo!()
        }
    }
}