use hyper::Method;
use crate::apis::company::company_routes::{ post, get };

pub fn resolve_routes(method: Method, route: &str, event: Request) -> Result<Response<Body>, Error> {
    println!("Resolving Route: {:?}", route);

    match route {
        "/company" => {
            match method {
                Method::POST => post(event),
                Method::GET => apis::company::company_routes::get(event),
                _ => todo!(),
            }
        },
        _ => {
            todo!()
        }
    }
}