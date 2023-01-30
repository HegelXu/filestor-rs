use deadpool_lapin::{Manager, Pool, PoolError};
use futures::{join, StreamExt};
use lapin::{options::*, types::FieldTable, BasicProperties, ConnectionProperties};
use std::convert::Infallible;
use std::result::Result as StdResult;
use std::time::Duration;
use thiserror::Error as ThisError;
use tokio_amqp::*;
use warp::{Filter, Rejection, Reply};

use crate::mq::producer::*;

pub type WebResult<T> = StdResult<T, Rejection>;
pub type RMQResult<T> = StdResult<T, PoolError>;
pub type Result<T> = StdResult<T, Error>;
pub type Connection = deadpool::managed::Object<deadpool_lapin::Manager>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("rmq error: {0}")]
    RMQError(#[from] lapin::Error),
    #[error("rmq pool error: {0}")]
    RMQPoolError(#[from] PoolError),
}

impl warp::reject::Reject for Error {}


//start_consume
pub async fn start_consume(pool:Pool) ->Result<()> {



    Ok(())
}

async fn rmq_listen(pool: Pool) -> Result<()> {
    let mut retry_interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        retry_interval.tick().await;
        println!("connecting rmq consumer...");
        match init_rmq_listen(pool.clone()).await {
            Ok(_) => println!("rmq listen returned"),
            Err(e) => eprintln!("rmq listen had an error: {}", e),
        };
    }
}

//stop_consume


#[warn(deprecated)]
async fn t_main() -> Result<()> {
    let addr =
        std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://rmq:rmq@127.0.0.1:5672/%2f".into());
    let manager = Manager::new(addr, ConnectionProperties::default());//.with_tokio());
    let pool: Pool = deadpool::managed::Pool::builder(manager)
        .max_size(10)
        .build()
        .expect("can create pool");

    let health_route = warp::path!("health").and_then(health_handler);
    let add_msg_route = warp::path!("msg")
        .and(warp::post())
        .and(with_rmq(pool.clone()))
        .and_then(add_msg_handler);
    let routes = health_route.or(add_msg_route);

    println!("Started server at localhost:8000");
    let _ = join!(
        warp::serve(routes).run(([0, 0, 0, 0], 8000)),
        rmq_listen(pool.clone())
    );
    Ok(())
}



async fn health_handler() -> WebResult<impl Reply> {
    Ok("OK")
}
