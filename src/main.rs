#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ditto_url = "https://pokeapi.co/api/v2/pokemon/ditto";
    let ditto_text = reqwest::get(ditto_url).await?.text().await?;
    let ditto_json: serde_json::Value = serde_json::from_str(&ditto_text)?;
    println!("Ditto get request:\n{:#?}", ditto_json);
    Ok(())
}