#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

use std::convert::Infallible;
use std::collections::HashMap;
use std::net::IpAddr;

use rocket::http::HeaderMap;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Value;

struct RequestHeaders<'a> {
    headers: HeaderMap<'a>,
    ip: IpAddr,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestHeaders<'r> {
    type Error = Infallible;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let ip = req.client_ip().unwrap();
        Outcome::Success(RequestHeaders {
            headers: req.headers().clone(),
            ip: ip,
        })
    }
}

#[get("/")]
fn index(token: RequestHeaders) -> Value {
    let mut headers_map = HashMap::new();

	let headers = token.headers.clone();
    let _ = headers
        .into_iter()
        .map(|header| {
            let header_str = header.name.as_str();
            match header_str {
                "user-agent" => {
                    headers_map.insert("software", token.headers.get_one(header_str).unwrap());
                }
                "accept-language" => {
                    headers_map.insert("language", token.headers.get_one(header_str).unwrap());
                }
                "x-forwarded-for" | "x-real-ip" => {
                    headers_map.insert("ipaddress", token.headers.get_one(header_str).unwrap());
                }
                _ => (),
            }
        })
        .collect::<Vec<_>>();

	let ip_string = token.ip.to_string();
    headers_map.insert("ipaddress", &ip_string);

    json!(headers_map)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/whoami", routes![index])
}
