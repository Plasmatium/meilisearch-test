use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use futures::future::try_join_all;
use tokio::task;


// use futures::future::try_join_all;


#[tokio::main]
async fn main() {
    let lines = read_lines("/home/jonny/playground/search-engine/wiki-articles.json").unwrap();
    let c = reqwest::Client::new();
    let mut payload = Vec::with_capacity(10010);
    let mut payload_size = 0;
    let mut joins = vec![];
    for (idx, line) in lines.enumerate() {
        let mut l = line.unwrap();
        let mut obj: serde_json::Value = serde_json::from_str(&l).unwrap();
        if let serde_json::Value::Object(ref mut o) = obj {
            o.insert("id".into(), serde_json::Value::Number(idx.into()));
        }
        l = serde_json::to_string(&obj).unwrap();
        payload_size += l.len() + 1; // +1 for ','
        payload.push(l);
        
        // stop at 99MB
        if payload_size > 1024 * 1024 * 950 {
            payload[0] = "[".to_string() + &payload[0];
            let last_idx = payload.len() - 1;
            payload[last_idx] = payload[last_idx].clone() + "]";
            let p = payload.join(",");

            println!("go! p.len is {}MB", p.len() / 1024 / 1024);

            let c = c.clone();
            let j = task::spawn(async move {
                let resp = c.put("http://localhost:7700/indexes/wiki/documents")
                    .header("content-type", "application/json")
                    .body(p)
                    .send().await.unwrap();
                println!("dealed: {}, status code: {}", idx, resp.status());
            });
            joins.push(j);

            payload.clear();
            payload_size = 0;
        }
    }

    payload[0] = "[".to_string() + &payload[0];
    let last_idx = payload.len() - 1;
    payload[last_idx] = payload[last_idx].clone() + "]";
    let p = payload.join(",");

    println!("go! p.len is {}MB", p.len() / 1024 / 1024);

    let c = c.clone();
    let j = task::spawn(async move {
        let resp = c.put("http://localhost:7700/indexes/wiki/documents")
            .header("content-type", "application/json")
            .body(p)
            .send().await.unwrap();
        println!("rest dealed: {}, status code: {}", payload.len(), resp.status());
    });
    joins.push(j);
    try_join_all(joins).await.unwrap();
    // std::thread::sleep(Duration::from_secs(600));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}