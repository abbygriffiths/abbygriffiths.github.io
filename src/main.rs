#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

use std::collections::HashMap;
use std::net::IpAddr;

use rocket::http::{HeaderMap, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Value;

struct APIRequest<'a> {
    headers: HeaderMap<'a>,
    ip: IpAddr,
}

#[derive(Debug)]
enum RequestError {
    IpNotFound,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for APIRequest<'r> {
    type Error = RequestError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(client_ip) = req.client_ip() {
            println!("Client IP: {}", client_ip);
            Outcome::Success(APIRequest {
                headers: req.headers().clone(),
                ip: client_ip,
            })
        } else {
            Outcome::Failure((Status::Unauthorized, Self::Error::IpNotFound))
        }
    }
}

#[get("/")]
fn index(request: APIRequest) -> Value {
    let mut headers_map: HashMap<String, String> = request
        .headers
        .clone()
        .into_iter()
        .map(|header| {
            (
                header.name.as_str().to_string(),
                request
                    .headers
                    .get_one(header.name.as_str())
                    .unwrap()
                    .to_string(),
            )
        })
        .collect();

    headers_map.insert("ipaddress".to_string(), request.ip.to_string());

    json!(headers_map)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/whoami", routes![index])
}
