use serde::{Deserialize, Serialize};
/// ```
/// use anime_info::kitsu::KitsuType;
/// assert_eq!(KitsuType::Anime.as_str(), "anime");
/// assert_eq!(KitsuType::Manga.as_str(), "manga");
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnimeType {
    Anime,
    Manga,
}

impl AnimeType {
    pub fn as_str(&self) -> &str {
        match self {
            AnimeType::Anime => "anime",
            AnimeType::Manga => "manga",
        }
    }
}

/// ```
/// use anime_info::kitsu::KitsuSubtype;
/// assert_eq!(Subtype::TV.as_str(), "TV");
/// assert_eq!(Subtype::OVA.as_str(), "OVA");
/// assert_eq!(Subtype::Movie.as_str(), "movie");
/// assert_eq!(Subtype::Special.as_str(), "special");
/// assert_eq!(Subtype::ONA.as_str(), "ONA");
/// assert_eq!(Subtype::Music.as_str(), "music");
/// assert_eq!(Subtype::Manga.as_str(), "manga");
/// assert_eq!(Subtype::Novel.as_str(), "novel");
/// assert_eq!(Subtype::OneShot.as_str(), "one shot");
/// assert_eq!(Subtype::Doujin.as_str(), "doujin");
/// assert_eq!(Subtype::Manhwa.as_str(), "manhwa");
/// assert_eq!(Subtype::Manhua.as_str(), "manhua");
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KitsuSubtype {
    #[serde(rename = "TV")]
    TV,
    #[serde(rename = "OVA")]
    OVA,
    Movie,
    Special,
    ONA,
    Music,
    Manga,
    Novel,
    OneShot,
    Doujin,
    Manhwa,
    Manhua,
}

impl KitsuSubtype {
    pub fn as_str(&self) -> &str {
        match self {
            KitsuSubtype::TV => "TV",
            KitsuSubtype::OVA => "OVA",
            KitsuSubtype::Movie => "movie",
            KitsuSubtype::Special => "special",
            KitsuSubtype::ONA => "ONA",
            KitsuSubtype::Music => "music",
            KitsuSubtype::Manga => "manga",
            KitsuSubtype::Novel => "novel",
            KitsuSubtype::OneShot => "one shot",
            KitsuSubtype::Doujin => "doujin",
            KitsuSubtype::Manhwa => "manhwa",
            KitsuSubtype::Manhua => "manhua",
        }
    }
}

/// Titles for a Kitsu anime or manga.
#[derive(Debug, Serialize, Deserialize)]
pub struct KitsuTitles {
    pub en: Option<String>, // Example: "That Time I Got Reincarnated as a Slime - Veldora's Journal"
    pub en_jp: Option<String>, // Example: "Tensei shitara Slime Datta Ken - Kanwa: Verudora Nikki"
    pub ja_jp: Option<String>, // Example: "転生したらスライムだった件 閑話: ヴェルドラ日記"
}

/// Attributes for a Kitsu anime or manga.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KitsuAnimeAttributes {
    #[serde(rename = "subtype")]
    pub kitsu_subtype: KitsuSubtype,
    pub titles: KitsuTitles,
    pub canonical_title: Option<String>,
    pub abbreviated_titles: Option<Vec<String>>,
}

/// A Kitsu anime or manga.
#[derive(Debug, Serialize, Deserialize)]
pub struct KitsuAnime {
    pub id: String,
    #[serde(rename = "type")]
    pub anime_type: AnimeType,
    pub attributes: KitsuAnimeAttributes,
}