use actix_web::web::{self};

use super::handlers;

pub fn config(config: &mut web::ServiceConfig){
    config
    .service(web::scope("/home")
        .service(handlers::home_handlers::test)
       .service(handlers::home_handlers::greet)
        // .service(handlers::home_handlers::find_all_users)
    )
    .service(web::scope("/user")
        .service(handlers::home_handlers::add_user)
        .service(handlers::home_handlers::get_users));
        
}