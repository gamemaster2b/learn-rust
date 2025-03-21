use mini_redis::{Result, client};

async fn say() {
    println!("World!");
}

#[tokio::main]
async fn main() -> Result<()> {
    let name = say();

    print!("Hello, ");

    name.await;

    // Open a connection to mini-redis
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world!".into()).await?;

    //get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    println!("\nHello, {:#?}", result.unwrap());

    Ok(())
}
