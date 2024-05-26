use visdom::{types::Elements, Vis};

use crate::providers::movies::dramacool::{
    DramaCool, DramaCoolEpisode, DramaCoolResult, DramaCoolServer, BASE_URL,
};

pub(crate) trait DramaCoolHTML {
    fn parse_search(&self, page_html: String) -> (Vec<Option<String>>, bool, usize);
    fn single_page(&self, media_html: String, id: &str, url: String) -> DramaCoolResult;
    fn info_episode(&self, episode_html: String) -> Vec<DramaCoolEpisode>;
    fn info_server(&self, server_html: String) -> Vec<DramaCoolServer>;
}

impl DramaCoolHTML for DramaCool {
    fn parse_search(&self, page_html: String) -> (Vec<Option<String>>, bool, usize) {
        let elements = create_html_fragment(&page_html);

        let page_parser = Page { elements };

        (
            page_parser.page_ids(),
            page_parser.has_next_page(),
            page_parser.total_pages(),
        )
    }

    fn single_page(&self, media_html: String, id: &str, url: String) -> DramaCoolResult {
        let elements = create_html_fragment(&media_html);

        let search_parser = Search {
            elements: &elements,
            id,
        };

        let info_parser = Info {
            elements: &elements,
        };

        DramaCoolResult {
            id: id.to_string(),
            title: search_parser.title(),
            url,
            image: search_parser.image(),
            status: info_parser.status(),
            release_date: search_parser.release_date(),
            other_names: search_parser.other_names(),
            description: info_parser.description(),
            genres: info_parser.label("Genre:"),
            country: info_parser.label("Country:"),
        }
    }

    fn info_episode(&self, episode_html: String) -> Vec<DramaCoolEpisode> {
        let elements = create_html_fragment(&episode_html);

        let episode_parser = Episodes { elements };

        episode_parser.episode_results()
    }

    fn info_server(&self, server_html: String) -> Vec<DramaCoolServer> {
        let elements = create_html_fragment(&server_html);

        let server_parser = Server { elements };

        server_parser.parse_server_html()
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
        self.elements.find("ul.pagination li").has_class("selected")
    }

    pub fn total_pages(&self) -> usize {
        let total_pages_href = self
            .elements
            .find("ul.pagination li.last:last-child a")
            .attr("href");

        if let Some(page_href) = total_pages_href {
            if let Some(pages) = page_href.to_string().rsplit('=').next() {
                return pages.parse::<usize>().unwrap_or(1);
            }
        }

        1
    }

    pub fn page_ids(&self) -> Vec<Option<String>> {
        self.elements
            .find("div.block div.tab-content ul.list-episode-item li a")
            .map(|_, element| {
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
    pub fn title(self) -> String {
        match self.id.split('/').last() {
            Some(title) => title.to_owned(),
            None => String::new(),
        }
    }

    pub fn image(self) -> String {
        let image_attr = self.elements.find("div.details div.img img").attr("src");

        if let Some(image) = image_attr {
            return image.to_string();
        };

        String::new()
    }

    pub fn release_date(self) -> String {
        let release_date = self
            .elements
            .find("div.details div.info p:contains('Released:')")
            .text()
            .replace("Released:", "")
            .trim()
            .to_owned();

        if release_date == "0" {
            return String::from("N/A");
        }

        release_date
    }

    pub fn other_names(&self) -> Vec<String> {
        self.elements
            .find(".other_name > a")
            .map(|_, element| element.text().trim().to_owned())
    }
}

#[derive(Clone, Copy)]
pub struct Info<'page, 'b> {
    pub elements: &'b Elements<'page>,
}

impl<'page, 'b> Info<'page, 'b> {
    pub fn status(&self) -> String {
        self.elements
            .find("div.details div.info p:contains('Status:')")
            .text()
            .replace("Status:", "")
            .trim()
            .to_owned()
    }

    pub fn description(&self) -> String {
        let description = self
            .elements
            .find("div.details div.info p:contains('Description')")
            .next("p")
            .text();

        if description.contains("N/A") {
            return String::from("");
        }

        description
    }

    pub fn label(&self, label: &str) -> Vec<String> {
        self.elements
            .find(&format!("div.details div.info p:contains('{}')", label))
            .text()
            .replace(label, "")
            .split(";")
            .map(|s| s.trim().to_owned())
            .filter(|x| !x.is_empty())
            .collect()
    }
}

pub struct Episodes<'a> {
    pub elements: Elements<'a>,
}

impl<'a> Episodes<'a> {
    pub fn episode_id(&self) -> Vec<String> {
        let episode_ids: Vec<_> = self
            .elements
            .find("div.content-left > div.block-tab > div > div > ul > li > a")
            .map(|_, element| {
                element
                    .get_attribute("href")
                    .map(|href| href.to_string())
                    .unwrap_or(String::new())
            })
            .into_iter()
            .filter_map(|href| href.split(".html").next().map(|s| s.to_owned()))
            .collect();

        episode_ids
    }

    pub fn episode_title(&self) -> Vec<String> {
        self.elements
            .find("div.content-left > div.block-tab > div > div > ul > li > a > h3")
            .map(|_, element| element.text())
    }

    pub fn episode_sub_type(&self) -> Vec<String> {
        self.elements
            .find("div.content-left > div.block-tab > div > div > ul > li > a > span.type")
            .map(|_, element| element.text())
    }

    pub fn episode_release_date(&self) -> Vec<String> {
        self.elements
            .find("div.content-left > div.block-tab > div > div > ul > li > a > span.time")
            .map(|_, element| element.text())
    }

    pub fn episode_url(&self) -> Vec<String> {
        self.elements
            .find("div.content-left > div.block-tab > div > div > ul > li > a")
            .map(|_, element| {
                element
                    .get_attribute("href")
                    .map(|href| {
                        let href_string = href.to_string();
                        format!("{}{}", BASE_URL, href_string)
                    })
                    .unwrap_or(String::new())
            })
    }

    pub fn episode_results(&self) -> Vec<DramaCoolEpisode> {
        let episode_ids = self.episode_id();
        let episode_titles = self.episode_title();
        let episode_sub_types = self.episode_sub_type();
        let episode_release_dates = self.episode_release_date();
        let episode_urls = self.episode_url();

        let mut episodes: Vec<DramaCoolEpisode> = vec![];

        for i in 0..episode_ids.len() {
            episodes.push(DramaCoolEpisode {
                id: episode_ids[i].clone(),
                title: episode_titles[i].clone(),
                sub_type: episode_sub_types[i].clone(),
                release_date: episode_release_dates[i].clone(),
                url: episode_urls[i].clone(),
            })
        }

        episodes
    }
}

pub struct Server<'a> {
    pub elements: Elements<'a>,
}

impl<'a> Server<'a> {
    pub fn parse_server_html(&self) -> Vec<DramaCoolServer> {
        self.elements
            .find("div.anime_muti_link > ul > li")
            .map(|_, element| {
                let mut url = element
                    .get_attribute("data-video")
                    .map(|value| value.to_string())
                    .unwrap_or(String::from(""));

                let mut name = element
                    .get_attribute("class")
                    .map(|value| value.to_string())
                    .unwrap_or(String::from(""));

                if name.contains("Standard") {
                    name = String::from("asianload")
                }

                if url.starts_with("//") {
                    url = url.replace("//", "https://");
                }

                DramaCoolServer { name, url }
            })
    }
}
