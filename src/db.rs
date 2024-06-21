use std::env;

use once_cell::sync::Lazy;
use redis::Client;

pub static mut REDIS: Lazy<Client> = Lazy::new(|| {
    Client::open(env::var("REDIS_URL").unwrap_or("redis://127.0.0.1/".to_string())).unwrap()
});
