#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ditto_url = "https://pokeapi.co/api/v2/pokemon/ditto";
    let ditto_text = reqwest::get(ditto_url).await?.text().await?;
    println!("Ditto get request:\n{}", ditto_text);
    Ok(())
}