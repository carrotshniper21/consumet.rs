use visdom::{types::Elements, Vis};

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

    pub fn total_pages(&self) -> Option<usize> {
        self.elements
            .find("ul.pagination li.last:last-child a")
            .attr("href")?
            .to_string()
            .rsplit('=')
            .next()?
            .parse::<usize>()
            .ok()
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
    pub fn search_title(self) -> String {
        match self.id.split('/').last() {
            Some(title) => title.to_owned(),
            None => String::new(),
        }
    }

    pub fn search_image(self) -> Option<String> {
        Some(
            self.elements
                .find("div.details div.img img")
                .attr("src")?
                .to_string(),
        )
    }

    pub fn search_release_date(self) -> String {
        self.elements
            .find(r#"div.details div.info p:contains("Released:")"#)
            .text()
            .replace("Released:", "")
            .trim()
            .to_owned()
    }

    pub fn search_other_names(&self) -> Option<Vec<String>> {
        Some(
            self.elements
                .find(".other_name > a")
                .map(|_, element| element.text().trim().to_owned()),
        )
    }
}

pub struct Info<'a> {
    pub elements: Elements<'a>,
}

impl<'a> Info<'a> {}
