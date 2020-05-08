use std::env;
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to db
    let (client, connection) =
        tokio_postgres::connect(
            &format!("host={} dbname={} user={} password={}", 
                env::var("POSTGRES_HOST").unwrap(),
                env::var("POSTGRES_DB").unwrap(),
                env::var("POSTGRES_USER").unwrap(),
                env::var("POSTGRES_PASSWORD").unwrap()), 
            NoTls
            ).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    init(&client).await?;

    Ok(())
}

async fn init(client: &tokio_postgres::Client) -> Result<(), Error> {
    // Check if item table is populated
    let rows = client
        .query("SELECT * FROM item LIMIT 1", &[])
        .await?;

    println!("{:?}", rows.len());

/*         let resp = reqwest::get("https://raw.githubusercontent.com/xivapi/ffxiv-datamining/master/csv/Item.csv")
        .await?
        .text()
        .await?;

    let result = parse(resp).unwrap();
    println!("Id: {}, Name: {}, Image: {}, Untradeable: {}", &result[4][0], &result[4][10], &result[4][11], &result[4][22]); */

    Ok(())
}

fn parse(text: String) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();

    for line in text.lines() {
        result.push(line.split(",").map(|s| s.to_string()).collect());
    }

    Ok(result)
}