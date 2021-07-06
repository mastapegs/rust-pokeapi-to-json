struct GameIndex {
    game_index: u32,
    version: Details,
}

struct Details {
    name: String,
    url: String,
}

struct Ability {
    ability: Details,
    is_hidden: bool,
    slot: u8,
}

struct Pokemon {
    abilities: Vec<Ability>,
    base_experience: u32,
    forms: Vec<Details>,
    game_indices: Vec<GameIndex>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ditto_url = "https://pokeapi.co/api/v2/pokemon/ditto";
    let ditto_text = reqwest::get(ditto_url).await?.text().await?;
    let ditto_json: serde_json::Value = serde_json::from_str(&ditto_text)?;
    println!("Ditto get request:\n{:#?}", ditto_json);
    Ok(())
}