use crate::mal::types::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use url::form_urlencoded;

const MAL_API_URL: &str = "https://api.jikan.moe/v4";
const ENDPOINT_ANIME: &str = "anime";

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeResponse {
    data: Vec<MALAnime>,
}

enum AnimeQuery<'a> {
    FullById { id: i32 },
    ById { id: i32 },
    // TODO: Add more queries
    Search { q: &'a str },
}

fn format_anime_url(query: &AnimeQuery) -> String {
    let mut params = form_urlencoded::Serializer::new(String::new());

    let params = match query {
        AnimeQuery::Search { q } => params.append_pair("q", q),
        _ => {
            todo!()
        }
    };

    let params = params.finish().to_string();

    format!("{}/{}?{}", MAL_API_URL, ENDPOINT_ANIME, params)
}

pub fn get_anime_full_by_id() {
    todo!()
}

pub fn get_anime_by_id() {
    todo!()
}

pub fn get_anime_characters() {
    todo!()
}

pub fn get_anime_staff() {
    todo!()
}

pub fn get_anime_episodes() {
    todo!()
}

pub fn get_anime_episode_by_id() {
    todo!()
}

pub fn get_anime_news() {
    todo!()
}

pub fn get_anime_forum() {
    todo!()
}

pub fn get_anime_videos() {
    todo!()
}

pub fn get_anime_videos_episodes() {
    todo!()
}

pub fn get_anime_pictures() {
    todo!()
}

pub fn get_anime_statistics() {
    todo!()
}

pub fn get_anime_more_info() {
    todo!()
}

pub fn get_anime_recommendations() {
    todo!()
}

pub fn get_anime_user_updates() {
    todo!()
}

pub fn get_anime_reviews() {
    todo!()
}

pub fn get_anime_relations() {
    todo!()
}

pub fn get_anime_themes() {
    todo!()
}

pub fn get_anime_external() {
    todo!()
}

pub fn get_anime_streaming() {
    todo!()
}

pub fn get_anime_search(search: &str) -> AnimeResponse {
    let client = Client::new();

    let query = AnimeQuery::Search {
        q: search,
    };

    let url = format_anime_url(&query);
    client
        .get(url)
        .send()
        .unwrap()
        .json::<AnimeResponse>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_anime_url_test() {
        let query = AnimeQuery::Search {
            q: "Tensei shitara",
        };

        let url = format_anime_url(&query);
        assert_eq!(
            url,
            "https://api.jikan.moe/v4/anime?q=Tensei+shitara"
        );
    }

    #[test]
    fn get_anime_search_test() {
        let a = get_anime_search("Tensei shitara Slime Datta Ken Movie: Guren no Kizuna-hen");
        println!("Success");
    }
}