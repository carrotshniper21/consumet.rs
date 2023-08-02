use crate::models::types::{IEpisodeServer, IMovieEpisode, TvType};
use visdom::{types::Elements, Vis};

pub fn create_html_fragment(page_html: &str) -> dyn Send + Elements<'_> {
    Vis::load(page_html).unwrap()
}

pub struct Page<'a> {
    pub elements: Elements<'a>
}

impl<'a> Page<'a> {
    pub fn has_next_page(&self) -> bool {
        self.elements
        .find("div.pre-pagination:nth-child(3) > nav:nth-child(1) > ul:nth-child(1) > li:nth-child(1)")
        .has_class("active")
    }

    pub fn total_pages(&self) -> Option<usize> {
        self.elements
        .find("div.pre-pagination:nth-child(3) > nav:nth-child(1) > ul:nth-child(1) > li.page-item:last-child a").attr("href")?
        .to_string()
        .rsplit('=')
        .next()?
        .parse::<usize>().ok()
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
    pub id: &'b str
}

impl<'page, 'b> Search<'page, 'b> {
    pub fn search_image(self) -> Option<String> {
        Some(
            self.elements
                .find("div.m_i-d-poster > div > img")
                .attr("src")?
                .to_string(),
        )
    }

    pub fn search_title(self) -> String {
        self.elements
        .find(
            "#main-wrapper > div.movie_information > div > div.m_i-detail > div.m_i-d-content > h2",
        )
        .text()
        .trim()
        .to_owned()
    }

    pub fn search_cover(self) -> Option<String> {
        Some(
            self.elements
                .find("div.w_b-cover")
                .attr("style")?
                .to_string()
                .replace("background-image: url(", "")
                .replace(')', ""),
        )
    }

    pub fn search_media_type(self) -> Option<TvType> {
        match self.id.split('/').next() {
            Some("tv") => Some(TvType::TvSeries),
            Some("movie") => Some(TvType::Movie),
            _ => None,
        }
    }
}

/// Remy clarke was here & some red guy
#[derive(Clone, Copy)]
pub struct Info<'page, 'b> {
    pub elements: &'b Elements<'page>,
}

impl<'page, 'b> Info<'page, 'b> {
    pub fn info_label(&self, index: usize, label: &str) -> Vec<String> {
        self.elements
            .find(&format!(
                "div.m_i-d-content > div.elements > div:nth-child({})",
                index
            ))
            .text()
            .replace(label, "")
            .split(',')
            .map(|s| s.trim().to_owned())
            .collect()
    }

    pub fn info_description(&self) -> Option<String> {
        Some(self.elements.find("#main-wrapper > div.movie_information > div > div.m_i-detail > div.m_i-d-content > div.description").text().trim().to_owned())
    }

    pub fn info_rating(&self) -> Option<String> {
        Some(
            self.elements
                .find("span.item:nth-child(2)")
                .text()
                .trim()
                .to_owned(),
        )
    }

    pub fn info_duration(&self) -> Option<String> {
        Some(
            self.elements
                .find("span.item:nth-child(3)")
                .text()
                .trim()
                .to_owned(),
        )
    }
}

pub struct Episodes {
    pub episodes: Vec<IMovieEpisode>,
    pub index: usize,
}

impl Iterator for Episodes {
    type Item = IMovieEpisode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.episodes.len() {
            let episode = self.episodes[self.index].clone();
            self.index += 1;
            Some(episode)
        } else {
            None
        }
    }
}

impl Episodes {
    pub fn episode_title(fragment: &Elements<'_>) -> Vec<Option<String>> {
        fragment.find("ul > li > a").map(|_, element| {
            element
                .get_attribute("title")
                .map(|value| value.to_string())
        })
    }

    pub fn episode_id(fragment: &Elements<'_>) -> Vec<Option<String>> {
        fragment.find("ul > li > a").map(|_, element| {
            element
                .get_attribute("data-id")
                .map(|value| value.to_string())
        })
    }

    pub fn episode_results(fragment: Elements<'_>, base_url: &str, i: usize) -> Self {
        let episode_titles = Self::episode_title(&fragment);
        let episode_ids = Self::episode_id(&fragment);

        let episode: Vec<IMovieEpisode> = episode_ids
            .iter()
            .zip(episode_titles.iter())
            .flat_map(|(id, title)| id.as_ref().map(|id| (id, title)))
            .map(|(id, title)| {
                let url = format!("{}/ajax/v2/episode/servers/{}", base_url, id);
                IMovieEpisode {
                    id: id.clone(),
                    title: title.clone(),
                    season: Some(i + 1),
                    url,
                    number: None,
                    description: None,
                    image: None,
                    release_date: None,
                }
            })
            .collect();

        Self {
            episodes: episode,
            index: 0,
        }
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
    pub element: Elements<'a>,
}

impl<'a> Server<'a> {
    pub fn parse_server_html(
        &self,
        base_url: &str,
        media_id: &str,
    ) -> anyhow::Result<Vec<IEpisodeServer>> {
        let servers: Vec<IEpisodeServer> = self.element.find("ul > li > a").map(|_, element| {
            let id = element
                .get_attribute("id")
                .unwrap()
                .to_string()
                .replace("watch-", "");

            let name = element
                .get_attribute("title")
                .unwrap()
                .to_string()
                .trim_start_matches("Server ")
                .to_owned();

            let url = format!("{}/watch-{}.{}", base_url, media_id, id);

            IEpisodeServer { name, url }
        });

        Ok(servers)
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
