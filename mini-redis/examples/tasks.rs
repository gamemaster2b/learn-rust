use std::rc::Rc;
use tokio::task::yield_now;

#[tokio::main]
async fn main() {
    let greeting = tokio::spawn(async {
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();
        format!("Hello, {name}!")
    });

    let greating = greeting.await.unwrap();

    println!("{}", greating);

    tokio::spawn(async move {
        println!("{}", greating);
    });

    tokio::spawn(async {
        let rc = Rc::new("hello");

        // `rc` is used after `.await`. It must be persisted to
        // the task's state.
        // yield_now().await;

        println!("{}", rc);
    });
}
