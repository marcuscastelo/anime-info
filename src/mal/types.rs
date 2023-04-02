use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MALAnime {
    pub mal_id: Option<i32>,
    pub url: Option<String>,
    pub images: Images,
    pub trailer: Trailer,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: Option<String>,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Vec<Option<String>>,
    #[serde(rename = "type")]
    pub anime_type: Option<String>,
    pub source: Option<String>,
    pub episodes: Option<i32>,
    pub status: Option<String>,
    pub airing: bool,
    pub aired: Aired,
    pub duration: Option<String>,
    pub rating: Option<String>,
    pub score: Option<f32>,
    pub scored_by: Option<i32>,
    pub rank: Option<i32>,
    pub popularity: Option<i32>,
    pub members: Option<i32>,
    pub favorites: Option<i32>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub season: Option<String>,
    pub year: Option<i32>,
    pub broadcast: Broadcast,
    pub producers: Vec<MALEntry>,
    pub licensors: Vec<MALEntry>,
    pub studios: Vec<MALEntry>,
    pub genres: Vec<MALEntry>,
    pub explicit_genres: Vec<MALEntry>,
    pub themes: Vec<MALEntry>,
    pub demographics: Vec<MALEntry>,
    pub relations: Option<Vec<Relations>>,
    pub theme: Option<Theme>,
    pub external: Option<Vec<ExternalEntry>>,
    pub streaming: Option<Vec<ExternalEntry>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    pub jpg: Image,
    pub webp: Image,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>,
    pub large_image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trailer {
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub embed_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
    #[serde(rename = "type")]
    pub title_type: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Aired {
    pub from: Option<String>,
    pub to: Option<String>,
    pub prop: AiredProp,
    pub string: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiredProp {
    pub from: DayMonthYear,
    pub to: DayMonthYear,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayMonthYear {
    pub day: Option<i32>,
    pub month: Option<i32>,
    pub year: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Broadcast {
    pub day: Option<String>,
    pub time: Option<String>,
    pub timezone: Option<String>,
    pub string: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MALEntry {
    pub mal_id: Option<i32>,
    #[serde(rename = "type")]
    pub mal_type: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relations {
    pub mal_id: Option<i32>,
    pub entry: Vec<MALEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Theme {
    pub openings: Vec<Option<String>>,
    pub endings: Vec<Option<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalEntry {
    pub name: Option<String>,
    pub url: Option<String>,
}