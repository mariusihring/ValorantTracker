use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Rank {
    key: String,
    player_name: String,
    player_tag: String,
}
impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.player_name, self.player_tag)
    }
}
#[derive(Deserialize, Debug)]
pub struct Player {
    status: i32,
    pub data: Data,
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}#{} \n region: {} \n level: {}",
            self.data.name.clone().unwrap(),
            self.data.tag.clone().unwrap(),
            self.data.region.clone().unwrap(),
            self.data.account_level.clone().unwrap()
        )
    }
}

#[derive(Debug)]
pub struct CouldNotFindPlayer {
    name: String,
}

impl Display for CouldNotFindPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not find player '{}'", self.name)
    }
}

impl std::error::Error for CouldNotFindPlayer {}

pub async fn get_rank(
    place: &str,
    api_key: &str,
    player_name: &str,
    player_tag: &str,
    client: &Client,
) -> Result<Player, Box<dyn std::error::Error>> {
    const PLAYER_REQUEST: &str = "https://api.henrikdev.xyz/valorant/v1/account/";

    let url = format!("{}{}/{}", PLAYER_REQUEST, player_name, player_tag);

    let request = client.get(url).build().unwrap();

    let resp = client.execute(request).await?.json::<Player>().await?;

    Ok(resp)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    #[serde(rename = "small")]
    pub small: Option<String>,

    #[serde(rename = "large")]
    pub large: Option<String>,

    #[serde(rename = "wide")]
    pub wide: Option<String>,

    #[serde(rename = "id")]
    pub id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "puuid")]
    pub puuid: Option<String>,

    #[serde(rename = "region")]
    pub region: Option<String>,

    #[serde(rename = "account_level")]
    pub account_level: Option<i32>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "tag")]
    pub tag: Option<String>,

    #[serde(rename = "card")]
    pub card: Option<Card>,

    #[serde(rename = "last_update")]
    pub last_update: Option<String>,

    #[serde(rename = "last_update_raw")]
    pub last_update_raw: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    #[serde(rename = "status")]
    pub status: Option<i32>,

    #[serde(rename = "data")]
    pub data: Option<Data>,
}
