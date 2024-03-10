use visdom::{types::Elements, Vis};

use crate::providers::movies::dramacool::{DramaCool, DramaCoolResult};

pub trait DramaCoolHTML {
    fn parse_search(&self, page_html: String) -> (Vec<Option<String>>, bool, usize);
    fn single_page(&self, media_html: String, id: &str, url: String) -> DramaCoolResult;
}

impl DramaCoolHTML for DramaCool {
    fn parse_search(&self, page_html: String) -> (Vec<Option<String>>, bool, usize) {
        let fragment = create_html_fragment(&page_html);

        let page_parser = Page { elements: fragment };

        (
            page_parser.page_ids(),
            page_parser.has_next_page(),
            page_parser.total_pages(),
        )
    }

    fn single_page(&self, media_html: String, id: &str, url: String) -> DramaCoolResult {
        let fragment = create_html_fragment(&media_html);

        let search_parser = Search {
            elements: &fragment,
            id,
        };

        DramaCoolResult {
            title: search_parser.title(),
            other_names: search_parser.other_names(),
            url,
            image: search_parser.image(),
            release_date: search_parser.release_date(),
            id: id.to_string(),
        }
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
        let image_attr = self
            .elements
            .find("div.details div.img img")
            .attr("src");

        if let Some(image) = image_attr {
            return image.to_string();
        };

        String::new()
    }

    pub fn release_date(self) -> String {
        self.elements
            .find(r#"div.details div.info p:contains("Released:")"#)
            .text()
            .replace("Released:", "")
            .trim()
            .to_owned()
    }

    pub fn other_names(&self) -> Vec<String> {
        self.elements
            .find(".other_name > a")
            .map(|_, element| element.text().trim().to_owned())
    }
}
