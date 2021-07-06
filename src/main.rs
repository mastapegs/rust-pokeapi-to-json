use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    r#move: Details,
}

#[derive(Serialize, Deserialize, Debug)]
struct VersionDetails {
    rarity: u32,
    version: Details,
}

#[derive(Serialize, Deserialize, Debug)]
struct HeldItem {
    item: Details,
    version_details: Vec<VersionDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GameIndex {
    game_index: u32,
    version: Details,
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Ability {
    ability: Details,
    is_hidden: bool,
    slot: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct Pokemon {
    abilities: Vec<Ability>,
    base_experience: u32,
    forms: Vec<Details>,
    game_indices: Vec<GameIndex>,
    height: u32,
    held_items: Vec<HeldItem>,
    id: u32,
    is_default: bool,
    location_area_encounters: String,
    moves: Vec<Move>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ditto_url = "https://pokeapi.co/api/v2/pokemon/ditto";
    let ditto_text = reqwest::get(ditto_url).await?.text().await?;
    let ditto_json: Pokemon = serde_json::from_str(&ditto_text)?;
    println!("Ditto get request:\n{:#?}", ditto_json);
    Ok(())
}
