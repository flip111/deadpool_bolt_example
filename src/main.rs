use bolt_client::*;
use bolt_proto::{message::*, value::*, version::*, Message, Value};
use deadpool_bolt::Manager;
use std::collections::HashMap;
use deadpool::managed::Manager as DeadPoolManager;
use tokio_util::compat::Compat;
use tokio::io::BufStream;
use futures::executor::block_on;

async fn makeClient() -> Client<Compat<BufStream<bolt_client::Stream>>> {
  let manager = deadpool_bolt::Manager::new("localhost:7687", None, [V4_3, V4_2, 0, 0], HashMap::new().into());
  let client: Client<Compat<BufStream<bolt_client::Stream>>> = deadpool_bolt::Manager::create(manager);
  return client;
}

fn main() {

  let client = block_on(makeClient);

  println!("Hello, world!");
}
