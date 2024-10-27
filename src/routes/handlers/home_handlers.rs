use actix_web::{get, post, web, HttpResponse, Responder};
use entity::user;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};

use crate::utils::app_state::AppState;

pub async fn create_user(db: &DatabaseConnection, data: user::Model) -> Result<user::Model, DbErr> {
    let active_user = user::ActiveModel {
        email: Set(data.email.to_owned()),
        name: Set(data.name.to_owned()),
        ..Default::default() 
    };
    active_user.insert(db).await
}

pub async fn get_all_user(db: &DatabaseConnection)-> Result<Vec<user::Model>, DbErr>{
    user::Entity::find().all(db).await
}

#[get("/test")]
pub async fn test()-> impl Responder{
    HttpResponse::Ok().body("Test!!!")
}

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello, {}", name))  // Returns plain text with 200 OK
}

#[post("/createUser")]
async fn add_user(
    req_body: web::Json<user::Model>,
    application_state: web::Data<AppState>,
) -> HttpResponse {
    let _ =
        create_user(&application_state.db, req_body.into_inner())
            .await;
    HttpResponse::Ok().body("success")
}

#[get("/getUsers")]
pub async fn get_users(application_state: web::Data<AppState>) -> impl Responder {
    match get_all_user(&application_state.db).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            println!("Error fetching users: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch users")
        }
    }
}