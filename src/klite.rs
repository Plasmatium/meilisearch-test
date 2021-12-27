use fake::{Dummy, Fake, Faker, faker::name::en::Name, locales::EN};
use mongodb::bson::oid::ObjectId;
use reqwest::Client;
use serde::Serialize;

///// USER //////
#[derive(Debug, Serialize)]
pub struct User {
    id: String,
    name: String,
}

pub fn gen_users(count: usize) -> Vec<User> {
    let mut users = Vec::with_capacity(count);
    let faker = Name();
    for _ in 0..count {
        let id = ObjectId::new().to_hex();
        let name = faker.fake();
        let user = User{id, name};
        users.push(user);
    }
    users
}

pub async fn add_users_index(c: &Client, users: Vec<User>) -> anyhow::Result<String> {
    let url = "http://localhost:8000/indexes/users/documents";
    let body = serde_json::to_string(&users)?;
    let resp_data = c.post(url).body(body).send().await?.text().await?;
    Ok(resp_data)
}

pub async fn commit_index(c: &Client, index: &str) -> anyhow::Result<String> {
    let url = format!("http://localhost:8000/indexes/{}/commit", index);
    let resp_data = c.post(url).send().await?.text().await?;
    Ok(resp_data)
}

////// DOC ///////

pub async fn run() -> anyhow::Result<()> {
    todo!()
}

pub fn pick_shared(users: Vec<User>) -> Vec<String> {
    todo!()
}

pub mod searcher {
    include!(concat!(env!("OUT_DIR"), "/searcher.rs"));
}

impl searcher::KeyFields {
    fn new_with(users: Vec<String>) -> Self {
        let id = ObjectId::new().to_hex();
        let author = ObjectId::new().to_hex();
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::klite::add_users_index;

    use super::{commit_index, gen_users, searcher::KeyFields};
    use fake::{faker::name::en::Name, Fake};
    use rand::distributions::Uniform;
    use tracing::info;

    #[tokio::test]
    async fn test_gen_user() {
        tracing_subscriber::fmt::init();

        let c = reqwest::Client::new();
        let users = gen_users(10);
        info!("last user is {:?}", users.last().unwrap());
        let resp = add_users_index(&c, users).await.unwrap();
        info!("resp: {}", resp);
        let data = commit_index(&c, "users").await.unwrap();
        info!("{}", data);
    }

    #[tokio::test]
    async fn test_commit() {
        tracing_subscriber::fmt::init();

        let c = reqwest::Client::new();
        let data = commit_index(&c, "users").await.unwrap();
        info!("{}", data);
    }

    #[test]
    fn test_names() {
        tracing_subscriber::fmt::init();
        let faker = Name();
        let names = (1..10).map(|_| -> String {
            let name: String = faker.fake();
            let name = format!("name:\\\"{}\\\"", name);
            name
        }).collect::<Vec<String>>();
        let names = names.join(" ");
        info!("names is {}", names);
    }

    #[test]
    fn test_rand() {
        use rand::prelude::*;
        tracing_subscriber::fmt::init();
        let range = Uniform::from(0..20);
        let values: Vec<u64> = rand::thread_rng().sample_iter(&range).take(100).collect();
        info!("{:?}", values);
    }
}