use crate::klite::*;

mod klite;

// use futures::future::try_join_all;

#[tokio::main]
async fn main() {
    use tracing::info;
    tracing_subscriber::fmt::init();

    let c = reqwest::Client::new();
    let users = gen_users(500_0000);
    info!("last user is {:?}", users.last().unwrap());
    let resp = add_users_index(&c, users).await.unwrap();
    info!("resp: {}", resp);
    let data = commit_index(&c, "users").await.unwrap();
    info!("{}", data);
}
