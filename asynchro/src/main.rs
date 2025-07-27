#![deny(clippy::all)]


//use futures::executor::block_on;
use tokio::time::{sleep, Duration};
use futures::Future;

// async fn get_name() -> String {
//     "John".to_string()
// }

fn get_async_name() -> impl Future<Output = String> {
    let name = "John".to_string();
    async move {
        format!("Hello {name} Doe")
    }
}

async fn call_api_one() -> String {
    sleep(Duration::from_secs(1)).await;
    "One".to_string()
}

async fn call_api_two() -> String {
    sleep(Duration::from_secs(2)).await;
    "Two".to_string()
}

// fn call_api_one() -> impl Future<Output = String> {
//     async {
//         sleep(Duration::from_secs(1)).await;
//         "One".to_string()
//     }

// }

// fn call_api_two() -> impl Future<Output = String> {
//     async {
//         sleep(Duration::from_secs(2)).await;
//         "Two".to_string()
//     }
// }

#[tokio::main]
async fn main() {
    // let name = block_on(get_name());
    // println!("{name}");

    let one = call_api_one().await;
    println!("{one}");
    let two = call_api_two().await;
    println!("{two}");
}
