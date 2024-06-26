use crate::{
    models::types::TvType,
    providers::movies::flixhq::{FlixHQ, FlixHQEpisode, FlixHQResult, FlixHQServer, BASE_URL},
};

use visdom::{types::Elements, Vis};

pub(crate) trait FlixHQHTML {
    fn parse_recent_shows(&self, recent_html: String) -> Vec<Option<String>>;
    fn parse_recent_movies(&self, recent_html: String) -> Vec<Option<String>>;
    fn parse_trending_movies(&self, trending_html: String) -> Vec<Option<String>>;
    fn parse_trending_shows(&self, trending_html: String) -> Vec<Option<String>>;
    fn parse_search(&self, page_html: String) -> (Vec<Option<String>>, bool, usize);
    fn single_page(&self, media_html: String, id: &str, url: String) -> FlixHQResult;
    fn info_season(&self, season_html: String) -> Vec<String>;
    fn info_episode(&self, episode_html: String) -> Vec<FlixHQEpisode>;
    fn info_server(&self, server_html: String, media_id: &str) -> Vec<FlixHQServer>;
}

impl FlixHQHTML for FlixHQ {
    fn parse_recent_shows(&self, recent_html: String) -> Vec<Option<String>> {
        let elements = create_html_fragment(&recent_html);

        let trending_parser = Recent { elements };

        trending_parser.recent_shows()
    }

    fn parse_recent_movies(&self, recent_html: String) -> Vec<Option<String>> {
        let elements = create_html_fragment(&recent_html);

        let trending_parser = Recent { elements };

        trending_parser.recent_movies()
    }

    fn parse_trending_movies(&self, trending_html: String) -> Vec<Option<String>> {
        let elements = create_html_fragment(&trending_html);

        let trending_parser = Trending { elements };

        trending_parser.trending_movies()
    }

    fn parse_trending_shows(&self, trending_html: String) -> Vec<Option<String>> {
        let elements = create_html_fragment(&trending_html);

        let trending_parser = Trending { elements };

        trending_parser.trending_shows()
    }

    fn parse_search(&self, page_html: String) -> (Vec<Option<String>>, bool, usize) {
        let elements = create_html_fragment(&page_html);

        let page_parser = Page { elements };

        (
            page_parser.page_ids(),
            page_parser.has_next_page(),
            page_parser.total_pages(),
        )
    }

    fn single_page(&self, media_html: String, id: &str, url: String) -> FlixHQResult {
        let elements = create_html_fragment(&media_html);

        let search_parser = Search {
            elements: &elements,
            id,
        };

        let info_parser = Info {
            elements: &elements,
        };

        FlixHQResult {
            cover: search_parser.cover(),
            title: search_parser.title(),
            url,
            image: search_parser.image(),
            country: info_parser.label(1, "Country:"),
            genres: info_parser.label(2, "Genre:"),
            release_date: info_parser.label(3, "Released:").join(""),
            media_type: search_parser.media_type(),
            id: id.to_string(),
            description: info_parser.description(),
            quality: info_parser.quality(),
            rating: info_parser.rating(),
            duration: info_parser.duration(),
            production: info_parser.label(4, "Production:"),
            casts: info_parser.label(5, "Casts:"),
            tags: info_parser.label(6, "Tags:"),
        }
    }

    fn info_season(&self, season_html: String) -> Vec<String> {
        let elements = create_html_fragment(&season_html);

        let season_parser = Seasons { elements };

        season_parser
            .season_results()
            .into_iter()
            .flatten()
            .collect()
    }

    fn info_episode(&self, episode_html: String) -> Vec<FlixHQEpisode> {
        let elements = create_html_fragment(&episode_html);

        let episode_parser = Episodes { elements };

        episode_parser.episode_results()
    }

    fn info_server(&self, server_html: String, media_id: &str) -> Vec<FlixHQServer> {
        let elements = create_html_fragment(&server_html);

        let server_parser = Server { elements };

        server_parser.parse_server_html(media_id)
    }
}

pub fn create_html_fragment(page_html: &str) -> Elements<'_> {
    Vis::load(page_html).unwrap()
}

pub struct Page<'a> {
    pub elements: Elements<'a>,
}

impl<'a> Page<'a> {
    pub fn has_next_page(&self) -> bool {
        self.elements
        .find("div.pre-pagination:nth-child(3) > nav:nth-child(1) > ul:nth-child(1) > li:nth-child(1)")
        .has_class("active")
    }

    pub fn total_pages(&self) -> usize {
        let total_pages_attr = self.elements.find("div.pre-pagination:nth-child(3) > nav:nth-child(1) > ul:nth-child(1) > li.page-item:last-child a").attr("href");

        if let Some(total_pages) = total_pages_attr {
            if let Some(pages) = total_pages.to_string().rsplit('=').next() {
                return pages.parse::<usize>().unwrap_or(1);
            }
        }

        1
    }

    pub fn page_ids(&self) -> Vec<Option<String>> {
        self.elements.find("div.film-poster > a").map(|_, element| {
            element
                .get_attribute("href")?
                .to_string()
                .strip_prefix('/')
                .map(String::from)
        })
    }
}

#[derive(Clone, Copy)]
pub struct Search<'page, 'b> {
    pub elements: &'b Elements<'page>,
    pub id: &'b str,
}

impl<'page, 'b> Search<'page, 'b> {
    pub fn image(&self) -> String {
        let image_attr = self
            .elements
            .find("div.m_i-d-poster > div > img")
            .attr("src");

        if let Some(image) = image_attr {
            return image.to_string();
        };

        String::new()
    }

    pub fn title(&self) -> String {
        self.elements
        .find(
            "#main-wrapper > div.movie_information > div > div.m_i-detail > div.m_i-d-content > h2",
        )
        .text()
        .trim()
        .to_owned()
    }

    pub fn cover(&self) -> String {
        let cover_attr = self.elements.find("div.w_b-cover").attr("style");
        if let Some(cover) = cover_attr {
            return cover
                .to_string()
                .replace("background-image: url(", "")
                .replace(')', "");
        };

        String::new()
    }

    pub fn media_type(&self) -> TvType {
        match self.id.split('/').next() {
            Some("tv") => TvType::TvSeries,
            Some("movie") => TvType::Movie,
            _ => todo!(),
        }
    }
}

/// Remy clarke was here & some red guy
#[derive(Clone, Copy)]
pub struct Info<'page, 'b> {
    pub elements: &'b Elements<'page>,
}

impl<'page, 'b> Info<'page, 'b> {
    pub fn label(&self, index: usize, label: &str) -> Vec<String> {
        self.elements
            .find(&format!(
                "div.m_i-d-content > div.elements > div:nth-child({})",
                index
            ))
            .text()
            .replace(label, "")
            .split(',')
            .map(|s| s.trim().to_owned())
            .filter(|x| !x.is_empty())
            .collect()
    }

    pub fn description(&self) -> String {
        self.elements.find("#main-wrapper > div.movie_information > div > div.m_i-detail > div.m_i-d-content > div.description").text().trim().to_owned()
    }

    pub fn quality(&self) -> String {
        self.elements
            .find("span.item:nth-child(1)")
            .text()
            .trim()
            .to_owned()
    }

    pub fn rating(&self) -> String {
        self.elements
            .find("span.item:nth-child(2)")
            .text()
            .trim()
            .to_owned()
    }

    pub fn duration(&self) -> String {
        self.elements
            .find("span.item:nth-child(3)")
            .text()
            .trim()
            .to_owned()
    }
}

pub struct Episodes<'a> {
    pub elements: Elements<'a>,
}

impl<'a> Episodes<'a> {
    pub fn episode_title(&self) -> Vec<Option<String>> {
        self.elements.find("ul > li > a").map(|_, element| {
            element
                .get_attribute("title")
                .map(|value| value.to_string())
        })
    }

    pub fn episode_id(&self) -> Vec<Option<String>> {
        self.elements.find("ul > li > a").map(|_, element| {
            element
                .get_attribute("data-id")
                .map(|value| value.to_string())
        })
    }

    pub fn episode_results(&self) -> Vec<FlixHQEpisode> {
        let episode_titles = self.episode_title();
        let episode_ids = self.episode_id();

        let mut episodes: Vec<FlixHQEpisode> = vec![];

        for (id, title) in episode_ids.iter().zip(episode_titles.iter()) {
            if let Some(id) = id {
                let url = format!("{}/ajax/v2/episode/servers/{}", BASE_URL, id);
                episodes.push(FlixHQEpisode {
                    id: id.clone(),
                    title: title.clone().unwrap_or(String::new()),
                    url,
                });
            }
        }

        episodes
    }
}

pub struct Seasons<'a> {
    pub elements: Elements<'a>,
}

impl<'a> Seasons<'a> {
    pub fn season_results(&self) -> Vec<Option<String>> {
        self.elements.find(".dropdown-menu > a").map(|_, element| {
            element
                .get_attribute("data-id")
                .map(|value| value.to_string())
        })
    }
}

pub struct Server<'a> {
    pub elements: Elements<'a>,
}

impl<'a> Server<'a> {
    pub fn parse_server_html(&self, media_id: &str) -> Vec<FlixHQServer> {
        self.elements.find("ul > li > a").map(|_, element| {
            let id = element
                .get_attribute("id")
                .map(|value| value.to_string().replace("watch-", ""))
                .unwrap_or(String::from(""));

            let name = element
                .get_attribute("title")
                .map(|value| value.to_string().trim_start_matches("Server ").to_owned());

            let url = format!("{}/watch-{}.{}", BASE_URL, media_id, id);
            let name = name.unwrap_or(String::from(""));

            FlixHQServer { name, url }
        })
    }
}

pub struct Recent<'a> {
    pub elements: Elements<'a>,
}

impl<'a> Recent<'a> {
    pub fn recent_movies(&self) -> Vec<Option<String>> {
        self.elements.find("#main-wrapper > div > section:nth-child(6) > div.block_area-content.block_area-list.film_list.film_list-grid > div > div.flw-item > div.film-poster > a").map(|_, element| {
            element
                .get_attribute("href")?
                .to_string()
                .strip_prefix('/')
                .map(String::from)
        })
    }

    pub fn recent_shows(&self) -> Vec<Option<String>> {
        self.elements.find("#main-wrapper > div > section:nth-child(7) > div.block_area-content.block_area-list.film_list.film_list-grid > div > div.flw-item > div.film-poster > a").map(|_, element| {
            element
                 .get_attribute("href")?
                .to_string()
                .strip_prefix('/')
                .map(String::from)

        })
    }
}

pub struct Trending<'a> {
    pub elements: Elements<'a>,
}

impl<'a> Trending<'a> {
    pub fn trending_movies(&self) -> Vec<Option<String>> {
        self.elements
            .find("div#trending-movies div.film_list-wrap div.flw-item div.film-poster a")
            .map(|_, element| {
                element
                    .get_attribute("href")?
                    .to_string()
                    .strip_prefix('/')
                    .map(String::from)
            })
    }

    pub fn trending_shows(&self) -> Vec<Option<String>> {
        self.elements
            .find("div#trending-tv div.film_list-wrap div.flw-item div.film-poster a")
            .map(|_, element| {
                element
                    .get_attribute("href")?
                    .to_string()
                    .strip_prefix('/')
                    .map(String::from)
            })
    }
}
