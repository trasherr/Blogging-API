use std::env;

use dotenv::dotenv;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref DATABASE_URL: String = set_database();
    pub static ref TOKEN: String = set_token();
}

fn set_database() -> String{
    dotenv().ok();
    env::var("DATABASE_URL").unwrap()
}


fn set_token() -> String{
    dotenv().ok();
    env::var("TOKEN").unwrap()
}
