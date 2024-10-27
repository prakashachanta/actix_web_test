use actix_web::web::{self};

use super::handlers;

pub fn config(config: &mut web::ServiceConfig){
    config
    .service(web::scope("/home")
        .service(handlers::home_handlers::test)
       .service(handlers::home_handlers::greet)
    )
    .service(web::scope("/user")
        .service(handlers::home_handlers::add_user)
        .service(handlers::home_handlers::get_users)
        .service(handlers::home_handlers::get_user)
        .service(handlers::home_handlers::delete_user));
        
}