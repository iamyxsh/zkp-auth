mod adapters;
mod models;
mod ports;

use std::sync::{Arc, Mutex};

use actix_cors::Cors;
use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use adapters::{http::ApiAdapter, mongo::MongoAdapter, redis::RedisAdapter};

use chaum_pedersen_zkp::models::ZKP;
use ports::api_port::APIPort;
use serde_json::json;

use crate::adapters::http::{ChallengeRequest, RegisterRequest, VerifyRequest};

#[get("/")]
async fn ping() -> impl Responder {
    "pong"
}

#[post("/register")]
async fn register(
    api: web::Data<Arc<Mutex<ApiAdapter>>>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    let mut guard = api.lock().unwrap();
    match guard.register(req.0).await {
        Ok(_) => HttpResponse::Ok().json(json!({"msg": "User successfully registered"})),
        Err(err) => HttpResponse::InternalServerError().json(json!({"msg": err})),
    }
}

#[post("/challenge")]
async fn challenge(
    api: web::Data<Arc<Mutex<ApiAdapter>>>,
    req: web::Json<ChallengeRequest>,
) -> impl Responder {
    let mut guard = api.lock().unwrap();
    match guard.create_challenge(req.0).await {
        Ok(res) => HttpResponse::Ok().json(json!({"auth_id": res.auth_id, "c": res.c })),
        Err(err) => HttpResponse::InternalServerError().json(json!({"msg": err})),
    }
}

#[post("/verify")]
async fn verify(
    api: web::Data<Arc<Mutex<ApiAdapter>>>,
    req: web::Json<VerifyRequest>,
) -> impl Responder {
    let mut guard = api.lock().unwrap();
    match guard.verify_challenge(req.0).await {
        Ok(user_info) => HttpResponse::Ok().json(json!({"session_id": user_info.session_id})),
        Err(err) => HttpResponse::InternalServerError().json(json!({"msg": err})),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let core = ZKP::default();
    let mongo = MongoAdapter::connect(
        "mongodb://localhost:27017".to_string(),
        "zkp".to_string(),
        "user_info".to_string(),
    )
    .await;

    let redis = RedisAdapter::connect("redis://localhost:6379".to_string()).await;
    let api = ApiAdapter { core, mongo, redis };
    let data = Arc::new(Mutex::new(api.clone()));
    HttpServer::new(move || {
        let cors = Cors::permissive();
        let api_scope = actix_web::web::scope("/api")
            .service(register)
            .service(challenge)
            .service(verify);
        App::new()
            .wrap(cors)
            .app_data(Data::new(data.clone()))
            .service(ping)
            .service(api_scope)
    })
    .bind(("localhost", 5000))?
    .run()
    .await
}
