use crate::data::mal::types::*;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use url::form_urlencoded;

const MAL_API_URL: &str = "https://api.jikan.moe/v4";
const ENDPOINT_ANIME: &str = "anime";

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    data: T,
}

enum AnimeQuery<'a> {
    // FullById { id: i32 },
    ById { id: i32 },
    // TODO: Add more queries
    Search { q: &'a str },
}

fn format_anime_url(query: &AnimeQuery) -> String {
    let mut params = form_urlencoded::Serializer::new(String::new());

    match query {
        AnimeQuery::Search { q } => { params.append_pair("q", q); }
        AnimeQuery::ById { id } => {}
    };

    let params = params.finish().to_string();

    match query {
        AnimeQuery::ById { id } => format!("{}/{}/{}", MAL_API_URL, ENDPOINT_ANIME, id),
        _ => format!("{}/{}?{}", MAL_API_URL, ENDPOINT_ANIME, params)
    }
}

pub fn get_anime_full_by_id() {
    todo!()
}

pub fn get_anime_by_id(id: i32) -> MALAnime {
    let client = Client::new();

    let query = AnimeQuery::ById { id };
    let url = format_anime_url(&query);

    client
        .get(url)
        .send()
        .unwrap()
        .json::<ApiResponse<MALAnime>>()
        .map(|r| r.data)
        .unwrap()
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

pub fn get_anime_search(search: &str) -> Vec<MALAnime> {
    let client = Client::new();

    let query = AnimeQuery::Search {
        q: search,
    };

    let url = format_anime_url(&query);
    client
        .get(url)
        .send()
        .unwrap()
        .json::<ApiResponse<Vec<MALAnime>>>()
        .map(|r| r.data)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_anime_url_test_search() {
        let query = AnimeQuery::Search {
            q: "Tensei shitara Slime Datta Ken Movie: Guren no Kizuna-hen",
        };

        let url = format_anime_url(&query);
        assert_eq!(
            url,
            "https://api.jikan.moe/v4/anime?q=Tensei+shitara+Slime+Datta+Ken+Movie%3A+Guren+no+Kizuna-hen"
        );
    }

    #[test]
    fn format_anime_url_test_by_id() {
        let query = AnimeQuery::ById { id: 49877 };

        let url = format_anime_url(&query);
        assert_eq!(url, "https://api.jikan.moe/v4/anime/49877");
    }

    #[test]
    fn get_anime_by_id_test() {
        let a = get_anime_by_id(49877);
        println!("Success");
    }

    #[test]
    fn get_anime_search_test() {
        let a = get_anime_search("Tensei shitara Slime Datta Ken Movie: Guren no Kizuna-hen");
        println!("Success");
    }
}