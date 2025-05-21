use mini_redis::client;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    let task_names: [&str; 3] = ["Alice", "Bella", "Crystall"];
    println!(
        "There are three tasks to run async writing and reading to the redis instance: {}, {} and {}\n",
        task_names[0], task_names[1], task_names[2]
    );

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for name in task_names {
        let handle: JoinHandle<()> = tokio::spawn(async move {
            task(name).await;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
}

async fn task(name: &str) {
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    for key in 1..=10 {
        if let Some(value) = client.get(&(key.to_string())[..]).await.unwrap() {
            println!("{} read: {:?}", name, value);
        } else {
            let value = format!("{} wrote {}", name, key);
            match client
                .set(&(key.to_string())[..], value.clone().into())
                .await
            {
                Ok(_) => {
                    println!("{value}");
                }
                Err(e) => {
                    eprintln!("{} failed to write {} because {:?}", name, key, e)
                }
            };
        }
    }
}
