use std::vec;

use crate::data::mal::api::anime as MalAnimeApi;
use crate::data::kitsu::api::anime as KitsuAnimeApi;
use crate::utils::vec_utils::VecUtils;

pub fn search_anime_names(anime_name: &str) -> Vec<String> {
    let mut names = vec![];
    
    let mal_anime = MalAnimeApi::get_anime_search(anime_name);
    let kitsu_anime = KitsuAnimeApi::anime(anime_name);

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
    #[test]
    fn search_anime_names() {
        let names = super::search_anime_names("Tensei shitara Slime Datta Ken Movie: Guren no Kizuna-hen");
        assert_eq!(names.len(), 20);
        assert_eq!(names[0], "That Time I Got Reincarnated as a Slime - Veldora's Journal");
        assert_eq!(names[1], "Tensei Shitara Slime Datta Ken");
        assert_eq!(names[2], "転生したらスライムだった件");
        assert_eq!(names[3], "転生したらスライムだった件");
        assert_eq!(names[4], "転生したらスライムだった件");
        assert_eq!(names[5], "転生したらスライムだった件");
        assert_eq!(names[6], "転生したらスライムだった件");
        assert_eq!(names[7], "転生したらスライムだった件");
        assert_eq!(names[8], "転生したらスライムだった件");
        assert_eq!(names[9], "転生したらスライムだった件");
        assert_eq!(names[10], "転生したらスライムだった件");
        assert_eq!(names[11], "転生したらスライムだった件");
        assert_eq!(names[12], "転生したらスライムだった件");
        assert_eq!(names[13], "転生したらスライムだった件");
        assert_eq!(names[14], "転生したらスライムだった件");
        assert_eq!(names[15], "転生したらスライムだった件");
        assert_eq!(names[16], "転生したらスライムだった件");
        assert_eq!(names[17], "転生したらスライムだった件");
        assert_eq!(names[18], "転生したらスライムだった件");
    }
}