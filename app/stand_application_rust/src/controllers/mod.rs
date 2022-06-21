use actix_web::{HttpResponse, Responder};
use serde::{Serialize};

pub mod stand_club;
pub mod stand_easy_resume;
pub mod stand_user;

pub fn make_response<T> (result: T) -> impl Responder
    where T: Serialize {
    let j = serde_json::to_string(&result).unwrap();
    HttpResponse::Ok().json(j)
}