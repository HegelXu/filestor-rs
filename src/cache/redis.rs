extern crate redis;
use lazy_static::lazy_static;
use std::sync::Mutex;



const REDIS_HOST: &str = "127.0.0.1:6379/";
const REDIS_PASS: &str = "testupload";

lazy_static! {
    #[derive(Clone)]
    pub static ref POOL: Mutex<redis::Connection> = {
        let redis_conn_url = format!("redis://:{}@{}",  REDIS_PASS, REDIS_HOST);
        let mut con = redis::Client::open(redis_conn_url)
            .expect("Invalid connection URL")
            .get_connection()
            .expect("failed to connect to Redis");
       
        Mutex::new(con)
    };
}

pub fn redis_pool() -> POOL {
    return POOL.clone();
}

