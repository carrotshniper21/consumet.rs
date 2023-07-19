use crate::models::base_parser::BaseParser;
use crate::models::base_provider::BaseProvider;
use crate::models::movie_parser::MovieParser;
use crate::models::types::{IMovieResult, ISearch, TvType};
use async_trait::async_trait;
use scraper::{Html, Selector};

pub struct FlixHQ;

#[derive(Debug)]
pub struct FlixHQSearchResult {
    pub base: IMovieResult,
    pub seasons: Option<usize>,
    pub description: Option<String>,
    pub country: Option<Vec<String>>,
    pub genre: Option<Vec<String>>,
    pub production: Option<Vec<String>>,
    pub casts: Option<Vec<String>>,
}

impl BaseParser for FlixHQ {}

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

#[async_trait]
impl MovieParser for FlixHQ {
    type SearchResult = FlixHQSearchResult;

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

        let (next_page, total_page, ids) = {
            let fragment = Html::parse_fragment(&data);

            // NOTE: don't use `?` for `Result<Selector, SelectorErrorKind>`
            // `SelectorErrorKind` can not be shared between threads safely
            let next_page_selector = Selector::parse(
                "div.pre-pagination:nth-child(3) > nav:nth-child(1) > ul:nth-child(1)",
            )
            .unwrap();
            let total_page_selector = Selector::parse(
                "div.pre-pagination:nth-child(3) > nav:nth-child(1) > ul:nth-child(1)",
            )
            .unwrap();
            let last_page_selector = Selector::parse("li.page-item:last-child a").unwrap();
            let item_selector = Selector::parse(".film_list-wrap > div.flw-item").unwrap();
            let id_selector = Selector::parse("div.film-poster > a").unwrap();

            let next_page = fragment
                .select(&next_page_selector)
                .last()
                .map(|element| !element.text().any(|text| text.trim() == "active"))
                .unwrap_or(false);

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

            let ids = fragment
                .select(&item_selector)
                .map(|element| {
                    element
                        .select(&id_selector)
                        .next()
                        .and_then(|el| el.value().attr("href"))
                        .map(|href| href[1..].to_owned())
                        .unwrap()
                })
                .collect::<Vec<_>>();

            (next_page, total_page, ids)
        };

        // `Vec<impl Future<Output = Result<FlixHQSearchResult>>>`
        let tasks = ids
            .iter()
            .map(|id| async move {
                self.fetch(id)
                    .await
                    .map_err(|err| anyhow::anyhow!("can't fetch {}, err: {}", id, err))
            })
            .collect::<Vec<_>>();

        // `Vec<FlixHQSearchResult>`
        let results = futures::future::try_join_all(tasks).await?;

        Ok(ISearch {
            current_page: Some(page),
            has_next_page: Some(next_page),
            total_pages: total_page,
            total_results: results.len(),
            results,
        })
    }
}

impl FlixHQ {
    async fn fetch(&self, id: &str) -> anyhow::Result<FlixHQSearchResult> {
        println!("Fetching {}", id);

        let url = format!("{}/{}", self.base_url(), id);
        let data = reqwest::Client::new()
            .get(&url)
            .send()
            .await?
            .text()
            .await?;

        let movie_type = match id.split("/").next() {
            Some("tv") => TvType::TvSeries,
            Some("movie") => TvType::Movie,
            _ => panic!("id {} not supported!", id),
        };

        let html = Html::parse_fragment(&data);

        // NOTE: don't use `?` for `Result<Selector, SelectorErrorKind>`
        // `SelectorErrorKind` can not be shared between threads safely
        let title_selector = Selector::parse("div.m_i-d-content > h2 > a").unwrap();
        let image_selector = Selector::parse("div.m_i-d-poster > div > img").unwrap();
        let release_date_selector =
            Selector::parse("div.m_i-d-content > div.elements > div:nth-child(3)").unwrap();
        let seasons_selector = Selector::parse(
            "#content-episodes > div > div > div.slc-eps > div.sl-title > div > div",
        )
        .unwrap();
        let description_selector = Selector::parse("#main-wrapper > div.movie_information > div > div.m_i-detail > div.m_i-d-content > div.description").unwrap();
        let country_selector = Selector::parse("#main-wrapper > div.movie_information > div > div.m_i-detail > div.m_i-d-content > div.elements > div:nth-child(1)").unwrap();
        // let genre_selector = Selector::parse("").unwrap();
        // let production_selector = Selector::parse("").unwrap();
        // let casts_selector = Selector::parse("").unwrap();

        let title = html
            .select(&title_selector)
            .next()
            .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_owned())
            .ok_or(anyhow::anyhow!("Can't get title"))?;

        let image = html
            .select(&image_selector)
            .next()
            .and_then(|el| el.value().attr("src"))
            .map(|t| t.to_owned())
            .ok_or(anyhow::anyhow!("Can't get image src"))?;

        let release_date = html
            .select(&release_date_selector)
            .next()
            .and_then(|el| el.last_child())
            .and_then(|child| child.value().as_text())
            .map(|text| text.trim().to_owned())
            .ok_or(anyhow::anyhow!("Can't get release date"))?;

        let seasons = html
            .select(&seasons_selector)
            .next()
            .map(|el| el.children().count());

        let description = html
            .select(&description_selector)
            .next()
            .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_owned())
            .ok_or(anyhow::anyhow!("Can't get description"))?;

        let country = html
            .select(&country_selector)
            .next()
            .and_then(|el| el.last_child())
            .and_then(|child| child.value().as_text())
            .ok_or(anyhow::anyhow!("Can't get country"))?
            .split(",")
            .map(|s| s.trim().to_owned())
            .collect::<Vec<_>>();

        Ok(FlixHQSearchResult {
            base: IMovieResult {
                id: id.to_owned(),
                title,
                url,
                image: Some(image),
                release_date: Some(release_date),
                movie_type: Some(movie_type),
            },
            seasons,
            description: Some(description),
            country: Some(country),
            genre: None,
            production: None,
            casts: None,
        })
    }
}
