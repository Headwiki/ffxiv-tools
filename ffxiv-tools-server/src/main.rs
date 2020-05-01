use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://raw.githubusercontent.com/xivapi/ffxiv-datamining/master/csv/Item.csv")
        .await?
        .text()
        .await?;

    //println!("{:#?}", resp);
    let result = parse(resp).unwrap();
    println!("Id: {}, Name: {}, Image: {}, Untradeable: {}", &result[4][0], &result[4][10], &result[4][11], &result[4][22]);

    Ok(())
}

fn parse(text: String) -> Result<Vec<Vec<String>>, Error> {

    let mut result = Vec::new();

    for line in text.lines() {
        //let mut line = line;
        result.push(line.split(",").map(|s| s.to_string()).collect());
    }

    Ok(result)
}