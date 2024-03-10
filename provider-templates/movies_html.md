<h1>Movie Html Parsing Template</h1>

<h2>Template Methods</h2>

- [Imports](#imports)
- [Methods](#methods)
- [Structs](#structs)
  - [Page](#page)
    - [Implmentation](#implmentation)
  - [Search](#search)
    - [Implmentation](#implmentation-1)
  - [Info](#info)
  - [Episodes](#episodes)
    - [Implmentation](#implmentation-2)
  - [Seasons](#seasons)
    - [Implmentations](#implmentations)
  - [Servers](#servers)
    - [Implmentations](#implmentations-1)

# Imports
```rs
use crate::models::types::{IEpisodeServer, IMovieEpisode, TvType};
use visdom::{types::Elements, Vis};
```

# Methods
```rs
pub fn create_html_fragment(page_html: &str) -> Elements<'_> {
    Vis::load(page_html).unwrap()
}
```

# Structs
## Page
```rs
pub struct Page<'a> {
    pub elements: Elements<'a>,
}
```

### Implmentation
```rs
impl<'a> Page<'a> {
    pub fn has_next_page(&self) -> bool {
        todo!()
    }

    pub fn total_pages(&self) -> Option<usize> {
        todo!()
    }

    pub fn page_ids(&self) -> Vec<Option<String>> {
        todo!()
    }
}
```

## Search
```rs
#[derive(Clone, Copy)]
pub struct Search<'page, 'b> {
    pub elements: &'b Elements<'page>,
    pub id: &'b str,
}
```

### Implmentation
```rs
impl<'page, 'b> Search<'page, 'b> {
    pub fn search_image(self) -> Option<String> {
        todo!()
    }

    pub fn search_title(self) -> String {
        todo!()
    }

    pub fn search_cover(self) -> Option<String> {
        todo!()
    }

    pub fn search_media_type(self) -> Option<TvType> {
        todo!()
    }
}
```

## Info
```rs
#[derive(Clone, Copy)]
pub struct Info<'page, 'b> {
    pub elements: &'b Elements<'page>,
}
```

```rs
impl<'page, 'b> Info<'page, 'b> {
    pub fn info_label(&self, index: usize, label: &str) -> Vec<String> {
        todo!()
    }

    pub fn info_description(&self) -> Option<String> {
        todo!()
    }

    pub fn info_rating(&self) -> Option<String> {
        todo!()
    }

    pub fn info_duration(&self) -> Option<String> {
        todo!()
    }
}
```

## Episodes
```rs
pub struct Episodes {
    pub episodes: Vec<IMovieEpisode>,
    pub index: usize,
}
```

### Implmentation
```rs
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
        todo!()
    }

    pub fn episode_id(fragment: &Elements<'_>) -> Vec<Option<String>> {
        todo!()
    }

    pub fn episode_results(fragment: Elements<'_>, base_url: &str, i: usize) -> Self {
        todo!()
    }
}
```

## Seasons
```rs
pub struct Seasons<'a> {
    pub elements: Elements<'a>,
}
```

### Implmentations
```rs
impl<'a> Seasons<'a> {
    pub fn season_results(&self) -> Vec<Option<String>> {
        todo!()
    }
}
```

## Servers
```rs
pub struct Server<'a> {
    pub element: Elements<'a>,
}
```

### Implmentations
```rs
impl<'a> Server<'a> {
    pub fn parse_server_html(
        &self,
        base_url: &str,
        media_id: &str,
    ) -> anyhow::Result<Vec<IEpisodeServer>> {
        todo!()
    }
}
```
