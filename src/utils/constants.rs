use std::env;

use dotenv::dotenv;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref DATABASE_URL: String = set_database();
}

fn set_database() -> String{
    dotenv().ok();
    env::var("DATABASE_URL").unwrap()
}

