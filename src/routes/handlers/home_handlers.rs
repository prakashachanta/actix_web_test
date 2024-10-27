use actix_web::{delete, get, post, web, HttpResponse, Responder};
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

pub async fn get_user_by_id(db: &DatabaseConnection, id: i32)-> Result<Option<user::Model>, DbErr>{
    user::Entity::find_by_id(id).one(db).await
}

pub async fn delete(db: &DatabaseConnection, id: i32)->Result<user::Model, DbErr> {
    if let Ok(Some(user)) = user::Entity::find_by_id(id).one(db).await {
        user::Entity::delete_by_id(id).exec(db).await.unwrap();
        Ok(user)
    } else {
        Err(DbErr::RecordNotFound(format!("User with ID {} not found", id)))
    }
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
async fn add_user(req_body: web::Json<user::Model>,application_state: web::Data<AppState>,) -> HttpResponse {
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

#[get("/getUser/{id}")]
pub async fn get_user(application_state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let id = id.into_inner();
    match get_user_by_id(&application_state.db, id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user), // Return user as JSON
        Ok(None) => HttpResponse::NotFound().finish(), // Return 404 if user not found
        Err(err) => {
            eprintln!("Database error: {:?}", err); // Log the error
            let message = format!("User with Id {:?} not found", id);
                    HttpResponse::InternalServerError().body(message) 
        }
    }
}

#[delete("/deleteUser/{id}")]
pub async fn delete_user(application_state: web::Data<AppState>, id: web::Path<i32>)-> impl Responder{
    match delete(&application_state.db, id.into_inner()).await{
        Ok(user) => {
            HttpResponse::Ok().json(user) // Return the deleted user as JSON
        }
        Err(err) => {
            eprintln!("Error deleting user: {:?}", err); // Log the error
            match err {
                DbErr::RecordNotFound(_) => HttpResponse::NotFound().finish(), // 404 if user not found
                _ => HttpResponse::InternalServerError().finish(), // Other errors
            }
        }
    }
}

