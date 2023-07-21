use crate::models::{
    BaseParser, BaseProvider, IMovieInfo, IMovieResult, ISearch, MovieParser, TvType,
};

use async_trait::async_trait;
use scraper::{Html, Selector};

pub struct FlixHQ;

#[derive(Debug)]
pub struct FlixHQSearch {
    pub base: IMovieResult,
    pub seasons: Option<usize>,
}

impl BaseProvider for FlixHQ {
    #[inline]
    fn name(&self) -> &str {
        "FlixHQ"
    }

    #[inline]
    fn base_url(&self) -> &str {
        "https://flixhq.to"
    }

    #[inline]
    fn logo(&self) -> &str {
        "https://upload.wikimedia.org/wikipedia/commons/7/7a/MyAnimeList_Logo.png"
    }

    #[inline]
    fn class_path(&self) -> &str {
        "MOVIES.FlixHQ"
    }
}

impl BaseParser for FlixHQ {}

#[async_trait]
impl MovieParser for FlixHQ {
    type SearchResult = FlixHQSearch;

    #[inline]
    fn supported_types(&self) -> &[TvType] {
        &[TvType::Movie, TvType::TvSeries]
    }

    async fn search(
        &self,
        query: String,
        page: Option<usize>,
    ) -> anyhow::Result<ISearch<Self::SearchResult>> {
        let page = page.unwrap_or(1);

        let url = format!("{}/search/{}?page={}", self.base_url(), query, page);
        let data = reqwest::Client::new().get(url).send().await?.text().await?;

        let (next_page, total_page, id_season) = {
            let fragment = Html::parse_fragment(&data);

            // NOTE: don't use `?` for `Result<Selector, SelectorErrorKind>`
            // `SelectorErrorKind` can not be shared between threads safely

            let next_page_selector = Selector::parse(
                "div.pre-pagination:nth-child(3) > nav:nth-child(1) > ul:nth-child(1)",
            )
            .unwrap();

            let next_page = fragment
                .select(&next_page_selector)
                .last()
                .map(|element| !element.text().any(|text| text.trim() == "active"))
                .unwrap_or(false);

            let total_page_selector = Selector::parse(
                "div.pre-pagination:nth-child(3) > nav:nth-child(1) > ul:nth-child(1)",
            )
            .unwrap();

            let last_page_selector = Selector::parse("li.page-item:last-child a").unwrap();
            let item_selector = Selector::parse(".film_list-wrap > div.flw-item").unwrap();

            let total_page = fragment
                .select(&total_page_selector)
                .next()
                .and_then(|total_page_element| {
                    total_page_element.select(&last_page_selector).next()
                })
                .and_then(|last_page_element| last_page_element.value().attr("href"))
                .map(|last_page_href| {
                    last_page_href
                        .rsplit('=')
                        .next()
                        .and_then(|last_page_str| last_page_str.parse().ok())
                        .unwrap_or(1)
                })
                .unwrap_or(1);

            let id_selector = Selector::parse("div.film-poster > a").unwrap();
            let seasons_selector =
                Selector::parse("div.fd-infor > span.fdi-item:nth-child(1)").unwrap();

            let id_season = fragment
                .select(&item_selector)
                .map(|element| {
                    let id = element
                        .select(&id_selector)
                        .next()
                        .and_then(|el| el.value().attr("href"))
                        .map(|href| href[1..].to_owned())
                        .unwrap();

                    let seasons_text = element
                        .select(&seasons_selector)
                        .next()
                        .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_owned())
                        .unwrap();

                    let seasons = if seasons_text.contains("SS") {
                        match seasons_text.split("SS").nth(1) {
                            None => Some(1usize),
                            Some(t) => t.trim().parse::<usize>().ok(),
                        }
                    } else {
                        None
                    };

                    (id, seasons)
                })
                .collect::<Vec<_>>();

            (next_page, total_page, id_season)
        };

        // NOTE: `Vec<impl Future<Output = Result<FlixHQSearch>>`
        let tasks = id_season
            .into_iter()
            .map(|(id, seasons)| async move {
                self.fetch(&id, seasons)
                    .await
                    .map_err(|err| anyhow::anyhow!("Err: Can't fetch {}, {}", id, err))
            })
            .collect::<Vec<_>>();

        let results = futures::future::try_join_all(tasks).await?;

        Ok(ISearch {
            current_page: Some(page),
            has_next_page: Some(next_page),
            total_pages: total_page,
            total_results: results.len(),
            results,
        })
    }

    async fn fetch_media_info(&self, media_id: String) -> anyhow::Result<IMovieInfo> {
        if !media_id.starts_with(self.base_url()) {
            let _ = format!("{}/{}", self.base_url(), media_id);
        }

        Ok(IMovieInfo {
            cover: None,
            recommendations: None,
            genres: None,
            description: None,
            rating: None,
            status: None,
            duration: None,
            production: None,
            casts: None,
            tags: None,
            total_episodes: None,
            seasons: None,
            episodes: None,
        })
    }
}

impl FlixHQ {
    async fn fetch(&self, id: &str, seasons: Option<usize>) -> anyhow::Result<FlixHQSearch> {
        let url = format!("{}/{}", self.base_url(), id);

        let data = reqwest::Client::new()
            .get(&url)
            .send()
            .await?
            .text()
            .await?;

        let movie_type = match id.split('/').next() {
            Some("tv") => TvType::TvSeries,
            Some("movie") => TvType::Movie,
            _ => panic!("Err: Type {} not supported!", id),
        };

        let html = Html::parse_fragment(&data);

        // NOTE: don't use `?` for `Result<Selector, SelectorErrorKind>`
        // `SelectorErrorKind` can not be shared between threads safely

        let image_selector = Selector::parse("div.m_i-d-poster > div > img").unwrap();

        let image = html
            .select(&image_selector)
            .next()
            .and_then(|el| el.value().attr("src"))
            .map(|t| t.to_owned())
            .ok_or(anyhow::anyhow!("Err: Can't get image src"))?;

        let title_selector = Selector::parse(
            "#main-wrapper > div.movie_information > div > div.m_i-detail > div.m_i-d-content > h2",
        )
        .unwrap();

        let title = html
            .select(&title_selector)
            .next()
            .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_owned())
            .ok_or(anyhow::anyhow!("Err: Can't get title"))?;

        let release_date_selector =
            Selector::parse("div.m_i-d-content > div.elements > div:nth-child(3)").unwrap();

        let release_date = html
            .select(&release_date_selector)
            .next()
            .and_then(|el| el.last_child())
            .and_then(|child| child.value().as_text())
            .map(|text| text.trim().to_owned())
            .ok_or(anyhow::anyhow!("Err: Can't get release date"))?;

        Ok(FlixHQSearch {
            base: IMovieResult {
                id: id.to_owned(),
                title,
                url,
                image,
                release_date,
                movie_type,
            },
            seasons,
        })
    }
}

// let description_selector = Selector::parse("#main-wrapper > div.movie_information > div > div.m_i-detail > div.m_i-d-content > div.description").unwrap();

// let description = html
//     .select(&description_selector)
//     .next()
//     .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_owned())
//     .ok_or(anyhow::anyhow!("Err: Can't get description"))?;

// let country_selector =
//     Selector::parse("div.m_i-d-content > div.elements > div:nth-child(1)").unwrap();

// let country: Vec<String> = html
//     .select(&country_selector)
//     .next()
//     .map(|el| el.text())
//     .ok_or(anyhow::anyhow!("Err: Can't get country"))?
//     .map(|country| {
//         let trimmed_country = country.trim().replace(',', "").replace("Country:", "");
//         trimmed_country
//     })
//     .filter(|trimmed_country| !trimmed_country.is_empty())
//     .collect();

// let genre_selector =
//     Selector::parse("div.m_i-d-content > div.elements > div:nth-child(2)").unwrap();

// let genre: Vec<String> = html
//     .select(&genre_selector)
//     .next()
//     .map(|el| el.text())
//     .ok_or(anyhow::anyhow!("Err: Can't get genres"))?
//     .map(|genre| {
//         let trimmed_genre = genre.trim().replace(',', "").replace("Casts:", "");
//         trimmed_genre
//     })
//     .filter(|trimmed_genre| !trimmed_genre.is_empty())
//     .collect();

// let production_selector =
//     Selector::parse("div.m_i-d-content > div.elements > div:nth-child(4)").unwrap();

// let production: Vec<String> = html
//     .select(&production_selector)
//     .next()
//     .map(|el| el.text())
//     .ok_or(anyhow::anyhow!("Err: Can't get production"))?
//     .map(|production| {
//         let trimmed_production = production
//             .trim()
//             .replace(',', "")
//             .replace("Production:", "");
//         trimmed_production
//     })
//     .filter(|trimmed_production| !trimmed_production.is_empty())
//     .collect();

// let cast_selector =
//     Selector::parse("div.m_i-d-content > div.elements > div:nth-child(5)").unwrap();

// let casts: Vec<String> = html
//     .select(&cast_selector)
//     .next()
//     .map(|el| el.text())
//     .ok_or(anyhow::anyhow!("Err: Can't get casts"))?
//     .map(|cast| {
//         let trimmed_cast = cast.trim().replace(',', "").replace("Casts:", "");
//         trimmed_cast
//     })
//     .filter(|trimmed_cast| !trimmed_cast.is_empty())
//     .collect();
