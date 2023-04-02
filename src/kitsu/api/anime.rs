use crate::kitsu::types::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use url::form_urlencoded;

// https://kitsu.io/api/edge/anime?filter[text]=Tensei%20shitara&fields[anime]=titles,canonicalTitle,abbreviatedTitles,subtype
const KITSU_API_URL: &str = "https://kitsu.io/api/edge";
const ENDPOINT_ANIME: &str = "anime";
const FILTER_TEXT_KEY: &str = "filter[text]";
const FIELDS_ANIME_KEY: &str = "fields[anime]";
const FIELDS_ANIME_VALUES: &str = "titles,canonicalTitle,abbreviatedTitles,subtype";

#[derive(Debug, Serialize, Deserialize)]
struct AnimeResponse {
    data: Vec<KitsuAnime>,
}

fn format_anime_url(name: &str) -> String {
    let params = form_urlencoded::Serializer::new(String::new())
        .append_pair(FILTER_TEXT_KEY, name)
        .append_pair(FIELDS_ANIME_KEY, FIELDS_ANIME_VALUES)
        .finish()
        .to_string();

    format!("{}/{}?{}", KITSU_API_URL, ENDPOINT_ANIME, params)
}

/// Search for an anime by name.
/// ```
/// use anime_info::kitsu::api::anime::anime;
///
/// let animes = anime("Tensei shitara").unwrap();
/// assert_eq!(animes.len(), 10);
/// assert_eq!(animes[0].attributes.canonical_title, Some("That Time I Got Reincarnated as a Slime - Veldora's Journal".to_string()));
/// ```
pub fn anime(name: &str) -> Result<Vec<KitsuAnime>, reqwest::Error> {
    let client = Client::new();

    let url = format_anime_url(name);
    client
        .get(url)
        .send()?
        .json::<AnimeResponse>()
        .map(|r| r.data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let animes = anime("Tensei shitara").unwrap();
        assert_eq!(animes.len(), 10);
        assert_eq!(
            animes[0].attributes.canonical_title,
            Some("That Time I Got Reincarnated as a Slime - Veldora's Journal".to_string())
        );
    }
}