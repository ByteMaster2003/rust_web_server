use actix_web::{post, get, web, HttpResponse, Responder};

#[get("/login")]
pub async fn login() -> impl Responder {
  HttpResponse::Ok().json("Login endpoint")
}

#[post("/register")]
pub async fn register() -> impl Responder {
  HttpResponse::Ok().json("Register endpoint")
}

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(login).service(register);
}
