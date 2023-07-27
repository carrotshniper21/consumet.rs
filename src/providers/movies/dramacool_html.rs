use visdom::Vis;

use crate::models::IMovieResult;

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

    let mut id_vec: Vec<String> = vec![];
    let id_selector = page_fragment.find("div.block div.tab-content ul.list-episode-item li a");
    id_selector.map(|i, element| id_vec.push(element.get_attribute("href").unwrap().to_string()));

    Ok((next_page, total_page, id_vec))
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
        page_fragment.find(r#"div.details > div.info > p:contains("Released")"#);
    let release_date = release_date_selector
        .text()
        .replace("Released:", "")
        .trim()
        .to_owned();

    Ok(IMovieResult {
        id: Some(id),
        cover: None,
        title: Some(title),
        url: Some(url),
        image: Some(image),
        release_date: Some(release_date),
        media_type: None,
    })
}
