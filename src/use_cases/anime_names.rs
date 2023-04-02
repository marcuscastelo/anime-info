use std::vec;

// use crate::data::kitsu::api::anime as KitsuAnimeApi;
use crate::data::kitsu::types::KitsuAnime;
use crate::data::mal::api::anime as MalAnimeApi;
use crate::utils::vec_utils::VecUtils;

pub enum Query<'a> {
    Name(&'a str),
    Id(i32),
}

pub fn search_anime_names(query: &Query) -> Vec<String> {
    let mut names = vec![];

    let mal_anime = match query {
        Query::Name(name) => MalAnimeApi::get_anime_search(name),
        Query::Id(id) => vec![MalAnimeApi::get_anime_by_id(*id)],
    };

    // let kitsu_anime = KitsuAnimeApi::anime(anime_name);
    let kitsu_anime: Vec<KitsuAnime> = vec![];

    for anime in mal_anime {
        if let Some(title) = anime.title {
            names.push(title);
        }
        if let Some(title_english) = anime.title_english {
            names.push(title_english);
        }
        if let Some(title_japanese) = anime.title_japanese {
            names.push(title_japanese);
        }
        if let Some(title_synonyms) = anime.title_synonyms {
            for title_synonym in title_synonyms {
                names.push(title_synonym);
            }
        }
        for title in anime.titles {
            names.push(title.title);
        }
    }

    for anime in kitsu_anime {
        if let Some(canonical_title) = anime.attributes.canonical_title {
            names.push(canonical_title);
        }
        if let Some(abbreviated_titles) = anime.attributes.abbreviated_titles {
            for abbreviated_title in abbreviated_titles {
                names.push(abbreviated_title);
            }
        }
        if let Some(ja_jp) = anime.attributes.titles.ja_jp {
            names.push(ja_jp);
        }
        if let Some(en_jp) = anime.attributes.titles.en_jp {
            names.push(en_jp);
        }
        if let Some(en) = anime.attributes.titles.en {
            names.push(en);
        }
    }

    names.into_unique()
}

#[cfg(test)]
mod tests {
    use crate::use_cases::anime_names::Query;

    #[test]
    fn search_anime_names() {
        let names = super::search_anime_names(&Query::Name(
            "Tensei shitara Slime Datta Ken Movie: Guren no Kizuna-hen",
        ));
        assert!(names.len() >= 3);
        assert_eq!(
            names[0],
            "Tensei shitara Slime Datta Ken Movie: Guren no Kizuna-hen"
        );
        assert_eq!(
            names[1],
            "That Time I Got Reincarnated as a Slime: The Movie - Scarlet Bond"
        );
        assert_eq!(names[2], "劇場版 転生したらスライムだった件 紅蓮の絆編");
    }
}
