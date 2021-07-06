use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct OtherSprites {
    dream_world: Option<String>,
    official_artwork: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sprites {
    back_female: String,
    back_shiny_female: String,
    back_default: String,
    front_female: String,
    front_shiny_female: String,
    back_shiny: String,
    front_default: String,
    front_shiny: String,
    other: OtherSprites,
}

#[derive(Serialize, Deserialize, Debug)]
struct VersionGroupDetails {
    level_learned_at: u32,
    move_learn_method: Details,
    version_group: Details,
}

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    r#move: Details,
    version_group_details: Vec<VersionGroupDetails>,
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
struct PokemonAbility {
    ability: Details,
    is_hidden: bool,
    slot: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct Pokemon {
    abilities: Vec<PokemonAbility>,
    base_experience: u32,
    forms: Vec<Details>,
    game_indices: Vec<GameIndex>,
    height: u32,
    held_items: Vec<HeldItem>,
    id: u32,
    is_default: bool,
    location_area_encounters: String,
    moves: Vec<Move>,
    name: String,
    order: u32,
    weight: u32,
    species: Details,
    // sprites: Sprites,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ditto_url = "https://pokeapi.co/api/v2/pokemon/ditto";
    let ditto_text = reqwest::get(ditto_url).await?.text().await?;
    let ditto_json: Pokemon = serde_json::from_str(&ditto_text)?;
    println!("Ditto get request:\n{:#?}", ditto_json);
    Ok(())
}
