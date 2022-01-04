use bolt_client::*;
use bolt_proto::{message::*, version::*, Value};
use deadpool_bolt::{Manager, Pool};

async fn create_pool() -> Pool {
    let manager = Manager::new(
        "localhost:7687",
        None,
        [V4_3, V4_2, 0, 0],
        Metadata::from_iter(vec![
            ("user_agent", "example-client/X.Y.Z"),
            ("scheme", "basic"),
            // Replace these with credentials for your DB
            ("principal", "neo4j"),
            ("credentials", "neo4j"),
        ]),
    )
    .await
    .unwrap();

    Pool::builder(manager).max_size(15).build().unwrap()
}

#[tokio::main]
async fn main() {
    // Share this pool across your route handlers
    let pool: Pool = create_pool().await;

    // Do this in your route handler
    let mut client = pool.get().await.unwrap();

    // Use the connection however you wish
    client.run("RETURN 42 AS num;", None, None).await.unwrap();
    let (records, summary) = client
        .pull(Some(Metadata::from_iter(vec![("n", 1)])))
        .await
        .unwrap();
    println!("{:?}", summary);
    println!("{:?}", records);

    assert!(Success::try_from(summary).is_ok());
    assert_eq!(records[0].fields(), &[Value::from(42)]);

    // When the connection is dropped, it will be returned to the pool and reset
}
