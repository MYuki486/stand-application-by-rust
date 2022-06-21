use actix_web::{web, get, Responder};
use crate::controllers::make_response;

const PREFIX_STAND_USER_USER_CONTROLLER: &'static str = "/stand_user/user";

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(PREFIX_STAND_USER_USER_CONTROLLER)
            .service(get_index)
            .service(get_list),
    );
}

#[get("/")]
async fn get_index() -> impl Responder {
    make_response("user index")
}

#[get("/list")]
async fn get_list() -> impl Responder {
    make_response("user list")
}