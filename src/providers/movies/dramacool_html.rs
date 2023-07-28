use super::DramaCoolInfo;
use crate::models::{IMovieEpisode, IMovieInfo, IMovieResult, MediaStatus, IMovieSeason};

use visdom::Vis;

pub fn parse_page_html(page_html: String) -> anyhow::Result<(bool, usize, Vec<String>)> {
    let page_fragment = Vis::load(&page_html).unwrap();

    let next_page_selector = page_fragment.find("ul.pagination li");
    let next_page = next_page_selector.has_class("selected");

    let total_page_selector = page_fragment.find("ul.pagination li.last:last-child a");
    let total_page = total_page_selector
        .attr("href")
        .unwrap()
        .to_string()
        .rsplit('=')
        .next()
        .and_then(|total_page| total_page.parse().ok())
        .unwrap_or(1);

    let id_selector = page_fragment.find("div.block div.tab-content ul.list-episode-item li a");
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

    let title = id.split('/').last().unwrap().to_owned();

    let image_selector = page_fragment.find("div.details div.img img");
    let image = image_selector.attr("src").unwrap().to_string();

    let release_date_selector =
        page_fragment.find(r#"div.details div.info p:contains("Released:")"#);
    let release_date = release_date_selector
        .text()
        .replace("Released:", "")
        .trim()
        .to_owned();

    let other_name_selector = page_fragment.find(".other_name > a");
    let other_names: Vec<String> =
        other_name_selector.map(|_, element| element.text().trim().to_owned());

    Ok(IMovieResult {
        id: Some(id),
        cover: None,
        title: Some(title),
        other_names: Some(other_names),
        url: Some(url),
        image: Some(image),
        release_date: Some(release_date),
        media_type: None,
    })
}

pub fn parse_info_html(
    info: String,
    search_results: IMovieResult,
) -> anyhow::Result<DramaCoolInfo> {
    let info_fragment = Vis::load(&info).unwrap();

    let decription_selector = info_fragment.find("div.details div.info p:nth-child(4)");

    let description = decription_selector.text().trim().to_owned();

    let status_selector = info_fragment.find(r#"div.details div.info p:contains("Status:")"#);
    let status = status_selector.text().replace("Status:", "");

    let media_status = match status.trim() {
        "OnGoing" => MediaStatus::OnGoing,
        "Completed" => MediaStatus::Completed,
        "Hiatus" => MediaStatus::Hiatus,
        "Cancelled" => MediaStatus::Cancelled,
        "NotYetAired" => MediaStatus::NotYetAired,
        "Unknown" => MediaStatus::Unknown,
        _ => panic!("Status {} not found!", status),
    };

    let episode_selector =
        info_fragment.find("div.content-left > div.block-tab > div > div > ul > li'");
    let mut episodes: Vec<Vec<IMovieEpisode>> = vec![];

    for episode in episode_selector {
        let episode = IMovieEpisode {
            id: None,
            title: None,
            season: None,
            url: None,
            number: None,
            description: None,
            image: None,
            release_date: None,
        };

        episodes.push(vec![episode]);
    }

    Ok(DramaCoolInfo {
        base: search_results,
        info: IMovieInfo {
            genres: None,
            description: Some(description),
            rating: None,
            status: Some(media_status),
            duration: None,
            country: None,
            production: None,
            casts: None,
            tags: None,
            total_episodes: Some(episodes.len()),
            seasons: Some(IMovieSeason {
                season: None,
                image: None,
                episodes: Some(episodes.clone()) 
            }),
            episodes: Some(episodes),
        },
    })
}
