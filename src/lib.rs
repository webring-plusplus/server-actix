use std::borrow::Cow;

use actix_web::{dev::HttpServiceFactory, web, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct WebringPlusplus {
    version: u32,
    links: Vec<Cow<'static, str>>,
}

pub async fn webring_plusplus(links: Vec<Cow<'static, str>>) -> impl Responder {
    web::Json(WebringPlusplus { version: 1, links })
}

pub fn webring_plusplus_service(links: Vec<Cow<'static, str>>) -> impl HttpServiceFactory {
    web::resource("/webring++.json").route(web::get().to(move || webring_plusplus(links.clone())))
}
