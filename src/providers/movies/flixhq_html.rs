use super::FlixHQInfo;
use crate::models::{IMovieEpisode, IMovieInfo, IMovieResult, TvType};

use scraper::{Html, Selector};

pub fn parse_page_html(page_html: String) -> anyhow::Result<(bool, usize, Vec<String>)> {
    let page_fragment = Html::parse_fragment(&page_html);

    // NOTE: don't use `?` for `Result<Selector, SelectorErrorKind>`
    // `SelectorErrorKind` can not be shared between threads safely

    let next_page_selector =
        Selector::parse("div.pre-pagination:nth-child(3) > nav:nth-child(1) > ul:nth-child(1)")
            .unwrap();

    let next_page = page_fragment
        .select(&next_page_selector)
        .last()
        .map(|element| !element.text().any(|text| text.trim() == "active"))
        .unwrap_or(false);

    let total_page_selector =
        Selector::parse("div.pre-pagination:nth-child(3) > nav:nth-child(1) > ul:nth-child(1)")
            .unwrap();

    let last_page_selector = Selector::parse("li.page-item:last-child a").unwrap();
    let item_selector = Selector::parse(".film_list-wrap > div.flw-item").unwrap();

    let total_page = page_fragment
        .select(&total_page_selector)
        .next()
        .and_then(|total_page_element| total_page_element.select(&last_page_selector).next())
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

    let id = page_fragment
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

    Ok((next_page, total_page, id))
}

pub fn parse_search_html(
    media_html: String,
    id: String,
    url: String,
) -> anyhow::Result<IMovieResult> {
    let page_fragment = Html::parse_fragment(&media_html);

    // NOTE: don't use `?` for `Result<Selector, SelectorErrorKind>`
    // `SelectorErrorKind` can not be shared between threads safely

    let image_selector = Selector::parse("div.m_i-d-poster > div > img").unwrap();

    let image = page_fragment
        .select(&image_selector)
        .next()
        .and_then(|el| el.value().attr("src"))
        .map(|t| t.to_owned())
        .ok_or(anyhow::anyhow!("Err: Can't get image src"))?;

    let title_selector = Selector::parse(
        "#main-wrapper > div.movie_information > div > div.m_i-detail > div.m_i-d-content > h2",
    )
    .unwrap();

    let title = page_fragment
        .select(&title_selector)
        .next()
        .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_owned())
        .ok_or(anyhow::anyhow!("Err: Can't get title"))?;

    let release_date_selector =
        Selector::parse("div.m_i-d-content > div.elements > div:nth-child(3)").unwrap();

    let release_date = page_fragment
        .select(&release_date_selector)
        .next()
        .and_then(|el| el.last_child())
        .and_then(|child| child.value().as_text())
        .map(|text| text.trim().to_owned())
        .ok_or(anyhow::anyhow!("Err: Can't get release date"))?;

    let cover_selector = Selector::parse("div.w_b-cover").unwrap();

    let cover = page_fragment
        .select(&cover_selector)
        .next()
        .and_then(|cover_element| cover_element.value().attr("style"))
        .map(|text| text.replace("background-image: url(", "").replace(')', ""))
        .ok_or(anyhow::anyhow!("Err: Can't get cover"))?;

    let media_type = match id.split('/').next() {
        Some("tv") => TvType::TvSeries,
        Some("movie") => TvType::Movie,
        _ => panic!("Err: Type {} not supported!", id),
    };

    Ok(IMovieResult {
        id: Some(id.to_owned()),
        cover: Some(cover.to_owned()),
        title: Some(title),
        url: Some(url),
        image: Some(image),
        release_date: Some(release_date),
        media_type: Some(media_type),
    })
}

pub fn parse_info_html(
    info_html: String,
    search_results: IMovieResult,
) -> anyhow::Result<FlixHQInfo> {
    let media_fragment = Html::parse_fragment(&info_html);

    // NOTE: For media_url html fragment
    let description_selector = Selector::parse("#main-wrapper > div.movie_information > div > div.m_i-detail > div.m_i-d-content > div.description").unwrap();

    let description = media_fragment
        .select(&description_selector)
        .next()
        .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_owned())
        .ok_or(anyhow::anyhow!("Err: Can't get description"))?;

    let country_selector =
        Selector::parse("div.m_i-d-content > div.elements > div:nth-child(1)").unwrap();

    let country: Vec<String> = media_fragment
        .select(&country_selector)
        .next()
        .map(|el| el.text())
        .ok_or(anyhow::anyhow!("Err: Can't get country"))?
        .map(|country| {
            let trimmed_country = country.trim().replace(',', "").replace("Country:", "");
            trimmed_country
        })
        .filter(|trimmed_country| !trimmed_country.is_empty())
        .collect();

    let genre_selector =
        Selector::parse("div.m_i-d-content > div.elements > div:nth-child(2)").unwrap();

    let genre: Vec<String> = media_fragment
        .select(&genre_selector)
        .next()
        .map(|el| el.text())
        .ok_or(anyhow::anyhow!("Err: Can't get genres"))?
        .map(|genre| {
            let trimmed_genre = genre.trim().replace(',', "").replace("Genre:", "");
            trimmed_genre
        })
        .filter(|trimmed_genre| !trimmed_genre.is_empty())
        .collect();

    let production_selector =
        Selector::parse("div.m_i-d-content > div.elements > div:nth-child(4)").unwrap();

    let production: Vec<String> = media_fragment
        .select(&production_selector)
        .next()
        .map(|el| el.text())
        .ok_or(anyhow::anyhow!("Err: Can't get production"))?
        .map(|production| {
            let trimmed_production = production
                .trim()
                .replace(',', "")
                .replace("Production:", "");
            trimmed_production
        })
        .filter(|trimmed_production| !trimmed_production.is_empty())
        .collect();

    let cast_selector =
        Selector::parse("div.m_i-d-content > div.elements > div:nth-child(5)").unwrap();

    let casts: Vec<String> = media_fragment
        .select(&cast_selector)
        .next()
        .map(|el| el.text())
        .ok_or(anyhow::anyhow!("Err: Can't get casts"))?
        .map(|cast| {
            let trimmed_cast = cast.trim().replace(',', "").replace("Casts:", "");
            trimmed_cast
        })
        .filter(|trimmed_cast| !trimmed_cast.is_empty())
        .collect();

    let tag_selector =
        Selector::parse("div.m_i-d-content > div.elements > div:nth-child(6)").unwrap();

    let tags: Vec<String> = media_fragment
        .select(&tag_selector)
        .next()
        .map(|el| el.text())
        .ok_or(anyhow::anyhow!("Err: Can't get tags"))?
        .map(|tag| {
            let trimmed_tag = tag.trim().replace(',', "").replace("Tags:", "");
            trimmed_tag
        })
        .filter(|trimmed_tag| !trimmed_tag.is_empty())
        .collect();

    let rating_selector = Selector::parse("span.item:nth-child(2)").unwrap();

    let rating = media_fragment
        .select(&rating_selector)
        .next()
        .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_owned())
        .ok_or(anyhow::anyhow!("Err: Can't get rating"))?;

    let duration_selector = Selector::parse("span.item:nth-child(3)").unwrap();

    let duration = media_fragment
        .select(&duration_selector)
        .next()
        .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_owned())
        .ok_or(anyhow::anyhow!("Err: Can't get duration"))?;

    Ok(FlixHQInfo {
        base: search_results,
        info: IMovieInfo {
            genres: Some(genre),
            description: Some(description),
            rating: Some(rating),
            status: None,
            duration: Some(duration),
            country: Some(country),
            production: Some(production),
            casts: Some(casts),
            tags: Some(tags),
            total_episodes: None,
            seasons: None,
            episodes: None,
        },
    })
}

pub fn parse_episode_html(
    base_url: &str,
    episode_html: String,
    i: usize,
) -> anyhow::Result<Vec<IMovieEpisode>> {
    let episode_fragment = Html::parse_fragment(&episode_html);
    let episode_item_selector = Selector::parse("ul > li > a").unwrap();

    let episode_ids: Vec<String> = episode_fragment
        .select(&episode_item_selector)
        .filter_map(|element| element.value().attr("data-id"))
        .map(|data| data[..].to_owned())
        .collect();

    let episode_titles: Vec<String> = episode_fragment
        .select(&episode_item_selector)
        .filter_map(|element| element.value().attr("title"))
        .map(|data| data[..].to_owned())
        .collect();

    let mut episodes: Vec<IMovieEpisode> = Vec::new();

    for (id, title) in episode_ids.iter().zip(episode_titles.iter()) {
        let url = format!("{}/ajax/episode/servers/{}", base_url, id);

        let episode = IMovieEpisode {
            id: id.clone(),
            title: title.clone(),
            season: Some(i + 1),
            url: Some(url),
            number: None,
            description: None,
            image: None,
            release_date: None,
        };

        episodes.push(episode);
    }

    Ok(episodes)
}

pub fn parse_season_html(season_html: String) -> anyhow::Result<Vec<String>> {
    let season_fragment = Html::parse_fragment(&season_html);

    let season_item_selector = Selector::parse(".dropdown-menu > a").unwrap();
    let season_ids: Vec<String> = season_fragment
        .select(&season_item_selector)
        .filter_map(|element| element.value().attr("data-id"))
        .map(|data| data[..].to_owned())
        .collect();

    Ok(season_ids)
}
