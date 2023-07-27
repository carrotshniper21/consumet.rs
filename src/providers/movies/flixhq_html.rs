use super::FlixHQInfo;
use crate::models::{IEpisodeServer, IMovieEpisode, IMovieInfo, IMovieResult, TvType};

use scraper::{Html, Selector};
use visdom::Vis;

pub fn parse_page_html(page_html: String) -> anyhow::Result<(bool, usize, Vec<String>)> {
    let page_fragment = Vis::load(&page_html).unwrap();

    let next_page_selector =
        page_fragment.find("div.pre-pagination:nth-child(3) > nav:nth-child(1) > ul:nth-child(1)");

    let next_page = next_page_selector.has_class("active");

    let total_page_selector = page_fragment
        .find("div.pre-pagination:nth-child(3) > nav:nth-child(1) > ul:nth-child(1) > li.page-item:last-child a");

    let total_page = total_page_selector
        .attr("href")
        .expect("Can't get total pages")
        .to_string()
        .rsplit('=')
        .next()
        .and_then(|total_page| total_page.parse().ok())
        .unwrap_or(1);

    let mut id_vec: Vec<String> = vec![];
    let id_selector = page_fragment.find("div.film-poster > a");
    id_selector.map(|_, element| {
        id_vec.push(
            element
                .get_attribute("href")
                .unwrap()
                .to_string()
                .split_off(1),
        )
    });

    Ok((next_page, total_page, id_vec))
}

pub fn parse_search_html(
    media_html: String,
    id: String,
    url: String,
) -> anyhow::Result<IMovieResult> {
    let page_fragment = Vis::load(&media_html).unwrap();

    let image_selector = page_fragment.find("div.m_i-d-poster > div > img");

    let image = image_selector
        .attr("src")
        .expect("Can't get image src")
        .to_string();

    let title_selector = page_fragment.find(
        "#main-wrapper > div.movie_information > div > div.m_i-detail > div.m_i-d-content > h2",
    );

    let title = title_selector.text().trim().to_owned();

    let release_date_selector =
        page_fragment.find("div.m_i-d-content > div.elements > div:nth-child(3)");

    let release_date = release_date_selector
        .last()
        .text()
        .replace("Released:", "")
        .trim()
        .to_owned();

    let cover_selector = page_fragment.find("div.w_b-cover");

    let cover = cover_selector
        .attr("style")
        .expect("Can't get cover style")
        .to_string()
        .replace("background-image: url(", "")
        .replace(')', "'");

    let media_type = match id.split('/').next() {
        Some("tv") => TvType::TvSeries,
        Some("movie") => TvType::Movie,
        _ => panic!("Err: Type {} not supported!", id),
    };

    Ok(IMovieResult {
        id: Some(id),
        cover: Some(cover),
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
    let media_fragment = Vis::load(&info_html).unwrap();

    let description_selector = media_fragment.find("#main-wrapper > div.movie_information > div > div.m_i-detail > div.m_i-d-content > div.description");

    let description = description_selector.text().trim().to_owned();

    let country_selector =
        media_fragment.find("div.m_i-d-content > div.elements > div:nth-child(1)");

    // let country: Vec<String> = media_fragment
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

    // let genre: Vec<String> = media_fragment
    //     .select(&genre_selector)
    //     .next()
    //     .map(|el| el.text())
    //     .ok_or(anyhow::anyhow!("Err: Can't get genres"))?
    //     .map(|genre| {
    //         let trimmed_genre = genre.trim().replace(',', "").replace("Genre:", "");
    //         trimmed_genre
    //     })
    //     .filter(|trimmed_genre| !trimmed_genre.is_empty())
    //     .collect();

    // let production_selector =
    //     Selector::parse("div.m_i-d-content > div.elements > div:nth-child(4)").unwrap();

    // let production: Vec<String> = media_fragment
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

    // let casts: Vec<String> = media_fragment
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

    // let tag_selector =
    //     Selector::parse("div.m_i-d-content > div.elements > div:nth-child(6)").unwrap();

    // let tags: Vec<String> = media_fragment
    //     .select(&tag_selector)
    //     .next()
    //     .map(|el| el.text())
    //     .ok_or(anyhow::anyhow!("Err: Can't get tags"))?
    //     .map(|tag| {
    //         let trimmed_tag = tag.trim().replace(',', "").replace("Tags:", "");
    //         trimmed_tag
    //     })
    //     .filter(|trimmed_tag| !trimmed_tag.is_empty())
    //     .collect();

    // let rating_selector = Selector::parse("span.item:nth-child(2)").unwrap();

    // let rating = media_fragment
    //     .select(&rating_selector)
    //     .next()
    //     .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_owned())
    //     .ok_or(anyhow::anyhow!("Err: Can't get rating"))?;

    // let duration_selector = Selector::parse("span.item:nth-child(3)").unwrap();

    // let duration = media_fragment
    //     .select(&duration_selector)
    //     .next()
    //     .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_owned())
    //     .ok_or(anyhow::anyhow!("Err: Can't get duration"))?;

    Ok(FlixHQInfo {
        base: search_results,
        info: IMovieInfo {
            genres: None,
            description: Some(description),
            rating: None,
            status: None,
            duration: None,
            country: None,
            production: None,
            casts: None,
            tags: None,
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
        let url = format!("{}/ajax/v2/episode/servers/{}", base_url, id);

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

pub fn parse_server_html(
    server_html: String,
    base_url: &str,
    is_movie: bool,
    media_id: String,
) -> anyhow::Result<Vec<IEpisodeServer>> {
    let server_fragment = Html::parse_fragment(&server_html);
    let server_item_selector = Selector::parse("ul > li > a").unwrap();

    let (server_ids, server_names) = if is_movie {
        let server_ids: Vec<String> = server_fragment
            .select(&server_item_selector)
            .filter_map(|element| element.value().attr("data-linkid"))
            .map(|data| data[..].to_owned())
            .collect();

        let server_names: Vec<String> = server_fragment
            .select(&server_item_selector)
            .filter_map(|element| element.value().attr("title"))
            .map(|data| data[..].to_owned())
            .collect();

        (server_ids, server_names)
    } else {
        let server_ids: Vec<String> = server_fragment
            .select(&server_item_selector)
            .filter_map(|element| element.value().attr("data-id"))
            .map(|data| data[..].to_owned())
            .collect();

        let server_names: Vec<String> = server_fragment
            .select(&server_item_selector)
            .filter_map(|element| element.value().attr("title"))
            .map(|data| data[..].to_owned().replace("Server ", ""))
            .collect();

        (server_ids, server_names)
    };

    let mut servers: Vec<IEpisodeServer> = Vec::new();

    for (id, name) in server_ids.iter().zip(server_names.iter()) {
        let url = format!("{}/watch-{}.{}", base_url, media_id, id);

        let server = IEpisodeServer {
            name: name.to_owned(),
            url,
        };

        servers.push(server);
    }

    Ok(servers)
}

pub fn parse_recent_movie_html(page_html: String) -> anyhow::Result<Vec<String>> {
    let page_fragment = Html::parse_fragment(&page_html);

    // NOTE: don't use `?` for `Result<Selector, SelectorErrorKind>`
    // `SelectorErrorKind` can not be shared between threads safely

    let item_selector = Selector::parse("#main-wrapper > div > section:nth-child(6) > div.block_area-content.block_area-list.film_list.film_list-grid > div > div.flw-item").unwrap();

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

    Ok(id)
}

pub fn parse_recent_shows_html(page_html: String) -> anyhow::Result<Vec<String>> {
    let page_fragment = Html::parse_fragment(&page_html);

    // NOTE: don't use `?` for `Result<Selector, SelectorErrorKind>`
    // `SelectorErrorKind` can not be shared between threads safely

    let item_selector = Selector::parse("#main-wrapper > div > section:nth-child(7) > div.block_area-content.block_area-list.film_list.film_list-grid > div > div.flw-item").unwrap();

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

    Ok(id)
}

pub fn parse_trending_movie_html(page_html: String) -> anyhow::Result<Vec<String>> {
    let page_fragment = Html::parse_fragment(&page_html);

    // NOTE: don't use `?` for `Result<Selector, SelectorErrorKind>`
    // `SelectorErrorKind` can not be shared between threads safely

    let item_selector =
        Selector::parse("div#trending-movies div.film_list-wrap div.flw-item").unwrap();

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

    Ok(id)
}

pub fn parse_trending_shows_html(page_html: String) -> anyhow::Result<Vec<String>> {
    let page_fragment = Html::parse_fragment(&page_html);

    // NOTE: don't use `?` for `Result<Selector, SelectorErrorKind>`
    // `SelectorErrorKind` can not be shared between threads safely

    let item_selector = Selector::parse("div#trending-tv div.film_list-wrap div.flw-item").unwrap();

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

    Ok(id)
}
