#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

use std::convert::Infallible;

use std::collections::HashMap;

use rocket::http::HeaderMap;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Value;

struct RequestHeaders<'a>(HeaderMap<'a>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestHeaders<'r> {
    type Error = Infallible;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(RequestHeaders(req.headers().clone()))
    }
}

#[get("/")]
fn index(token: RequestHeaders) -> Value {
    let mut headers_map = HashMap::new();

    let _ = token
        .0.clone()
        .into_iter()
        .map(|header| {
            let header_str = header.name.as_str();
            headers_map.insert(header_str.to_string(), token.0.get_one(header_str).unwrap());
        })
        .collect::<Vec<_>>();

    json!({
        "message": "hello",
        "headers": headers_map})
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/", routes![index])
}
