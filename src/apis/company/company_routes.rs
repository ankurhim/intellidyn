use lambda_http::{ Request, Response, Body, Error };
use crate::apis::company::company_model::{ define_company, list_companies };

pub async fn post(event: Request) -> Result<Response<Body>, Error> {
    let result = define_company(event).await?;

    let body = Body::from(result);
    let response = Response::builder()
    .status(200)
    .header("content-type", "application/json")
    .body(body)
    .map_err(Box::new).unwrap();

    Ok(response)
}

pub async fn get(event: Request) -> Result<Response<Body>, Error> {
    let data = list_companies(event).await?;

    let s = serde_json::to_string_pretty(&data).unwrap();
    let body = Body::from(s);
    let response = Response::builder()
    .status(200)
    .header("content-type", "application/json")
    .body(body)
    .map_err(Box::new).unwrap();

    Ok(response)
}