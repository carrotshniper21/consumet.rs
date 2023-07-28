use super::FlixHQInfo;
use crate::models::{IEpisodeServer, IMovieEpisode, IMovieInfo, IMovieResult, TvType};
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

    let id_selector = page_fragment.find("div.film-poster > a");
    let ids: Vec<String> = id_selector.map(|_, element| {
        element
            .get_attribute("href")
            .unwrap()
            .to_string()
            .split_off(1)
    });

    Ok((next_page, total_page, ids))
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
        .replace(')', "");

    let media_type = match id.split('/').next() {
        Some("tv") => TvType::TvSeries,
        Some("movie") => TvType::Movie,
        _ => panic!("Err: Type {} not supported!", id),
    };

    Ok(IMovieResult {
        id: Some(id),
        cover: Some(cover),
        title: Some(title),
        other_names: None,
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
    let info_fragment = Vis::load(&info_html).unwrap();

    let description_selector = info_fragment.find("#main-wrapper > div.movie_information > div > div.m_i-detail > div.m_i-d-content > div.description");

    let description = description_selector.text().trim().to_owned();

    let country_selector =
        info_fragment.find("div.m_i-d-content > div.elements > div:nth-child(1)");

    let country: Vec<String> = country_selector
        .text()
        .replace("Country:", "")
        .split(',')
        .map(|s| s.trim().to_owned())
        .collect();

    let genre_selector = info_fragment.find("div.m_i-d-content > div.elements > div:nth-child(2)");

    let genre: Vec<String> = genre_selector
        .text()
        .replace("Genre:", "")
        .split(",")
        .map(|s| s.trim().to_owned())
        .collect();

    let production_selector =
        info_fragment.find("div.m_i-d-content > div.elements > div:nth-child(4)");

    let production: Vec<String> = production_selector
        .text()
        .replace("Production:", "")
        .split(",")
        .map(|s| s.trim().to_owned())
        .collect();

    let cast_selector = info_fragment.find("div.m_i-d-content > div.elements > div:nth-child(5)");

    let casts: Vec<String> = cast_selector
        .text()
        .replace("Casts:", "")
        .split(",")
        .map(|s| s.trim().to_owned())
        .collect();

    let tag_selector = info_fragment.find("div.m_i-d-content > div.elements > div:nth-child(6)");

    let tags: Vec<String> = tag_selector
        .text()
        .replace("Tags:", "")
        .split(",")
        .map(|s| s.trim().to_owned())
        .collect();

    let rating_selector = info_fragment.find("span.item:nth-child(2)");

    let rating = rating_selector.text().trim().to_owned();

    let duration_selector = info_fragment.find("span.item:nth-child(3)");

    let duration = duration_selector.text().trim().to_owned();

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
    let episode_fragment = Vis::load(&episode_html).unwrap();
    let episode_selector = episode_fragment.find("ul > li > a");

    let episode_ids: Vec<String> = episode_selector
        .map(|_, element| element.get_attribute("data-id").unwrap().to_string());

    let episode_titles: Vec<String> =
        episode_selector.map(|_, element| element.get_attribute("title").unwrap().to_string());

    let mut episodes: Vec<IMovieEpisode> = Vec::new();

    for (id, title) in episode_ids.iter().zip(episode_titles.iter()) {
        let url = format!("{}/ajax/v2/episode/servers/{}", base_url, id);

        let episode = IMovieEpisode {
            id: Some(id.clone()),
            title: Some(title.clone()),
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
    let season_fragment = Vis::load(&season_html).unwrap();

    let season_item_selector = season_fragment.find(".dropdown-menu > a");
    let season_ids: Vec<String> = season_item_selector
        .map(|_, element| element.get_attribute("data-id").unwrap().to_string());

    Ok(season_ids)
}

pub fn parse_server_html(
    server_html: String,
    base_url: &str,
    is_movie: bool,
    media_id: String,
) -> anyhow::Result<Vec<IEpisodeServer>> {
    let server_fragment = Vis::load(&server_html).unwrap();
    let server_item_selector = server_fragment.find("ul > li > a");

    let (server_ids, server_names) = if is_movie {
        let server_ids: Vec<String> = server_item_selector
            .map(|_, element| element.get_attribute("data-linkid").unwrap().to_string());

        let server_names: Vec<String> = server_item_selector
            .map(|_, element| element.get_attribute("title").unwrap().to_string());

        (server_ids, server_names)
    } else {
        let server_ids: Vec<String> = server_item_selector
            .map(|_, element| element.get_attribute("data-id").unwrap().to_string());

        let server_names: Vec<String> = server_item_selector.map(|_, element| {
            element
                .get_attribute("title")
                .unwrap()
                .to_string()
                .replace("Server ", "")
        });

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
    let page_fragment = Vis::load(&page_html).unwrap();

    let id_selector = page_fragment.find("#main-wrapper > div > section:nth-child(6) > div.block_area-content.block_area-list.film_list.film_list-grid > div > div.flw-item > div.film-poster > a");

    let ids: Vec<String> = id_selector.map(|_, element| {
        element
            .get_attribute("href")
            .unwrap()
            .to_string()
            .split_off(1)
    });

    Ok(ids)
}

pub fn parse_recent_shows_html(page_html: String) -> anyhow::Result<Vec<String>> {
    let page_fragment = Vis::load(&page_html).unwrap();

    let id_selector = page_fragment.find("#main-wrapper > div > section:nth-child(7) > div.block_area-content.block_area-list.film_list.film_list-grid > div > div.flw-item > div.film-poster > a");

    let ids: Vec<String> = id_selector.map(|_, element| {
        element
            .get_attribute("href")
            .unwrap()
            .to_string()
            .split_off(1)
    });

    Ok(ids)
}

pub fn parse_trending_movie_html(page_html: String) -> anyhow::Result<Vec<String>> {
    let page_fragment = Vis::load(&page_html).unwrap();

    let id_selector =
        page_fragment.find("div#trending-movies div.film_list-wrap div.flw-item div.film-poster a");

    let ids: Vec<String> = id_selector.map(|_, element| {
        element
            .get_attribute("href")
            .unwrap()
            .to_string()
            .split_off(1)
    });

    Ok(ids)
}

pub fn parse_trending_shows_html(page_html: String) -> anyhow::Result<Vec<String>> {
    let page_fragment = Vis::load(&page_html).unwrap();

    let id_selector =
        page_fragment.find("div#trending-tv div.film_list-wrap div.flw-item div.film-poster a");

    let ids: Vec<String> = id_selector.map(|_, element| {
        element
            .get_attribute("href")
            .unwrap()
            .to_string()
            .split_off(1)
    });

    Ok(ids)
}
