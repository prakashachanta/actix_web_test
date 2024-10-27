use std::env;
use lazy_static::lazy_static;

lazy_static!(
    pub static ref PORT: u16 = set_port();
    pub static ref ADDRESS: String= set_address();
    pub static ref DATABASE_URL: String = set_database_url();
);

fn set_port()-> u16{
    dotenv::dotenv().ok();
    env::var("PORT").unwrap().parse().unwrap()
}

fn set_address()-> String{
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap()
}

fn set_database_url()-> String{
    dotenv::dotenv().ok();
    env::var("DATABASE_URL").unwrap()
}