<h1>Movie Template</h1>

<h2>Template Methods</h2>

- [Imports](#imports)
- [Structs](#structs)
- [Main Methods](#main-methods)
  - [Search](#search)
  - [Single Search](#single-search)
  - [Info](#info)
  - [Servers](#servers)
  - [Sources](#sources)

# Imports
```rs
use super::<provider_name_html>::{ /* Html Parsing Methods */ };

use crate::models::{BaseProvider};

use crate::extractors::{ /* Server Extractors */ };

use serde::Deserialize;
```

# Structs
```rs
pub struct <provider_name>;

#[derive(Debug, Deserialize)]
pub struct <provider_name>Info {
    link: String,
}

#[derive(Debug)]
pub struct <provider_name>Info {
    pub base: IMovieResult,
    pub info: IMovieInfo,
}
```

# Main Methods
## Search
| Parameter       | Type    | Description              |
| --------------- | ------- | ------------------------ |
| query           | `&str`  | query to search for.     |
| page (optional) | `usize` | page number (default: 1) |


```rs
    pub async fn search(
        &self,
        query: &str,
        page: Option<usize>,
    ) -> anyhow::Result<ISearch<IMovieResult>> {
        todo!()
    }
```

## Single Search
| Parameter | Type   | Description                                                                                                                     |
| --------- | ------ | ------------------------------------------------------------------------------------------------------------------------------- |
| id        | `&str` | takes media id or url as a parameter. (*media id or url can be found in the media search results as shown on the above method*) |

```rs
    pub async fn fetch_search_result(&self, id: &str) -> anyhow::Result<IMovieResult> {
        todo!()
    }
```

## Info
| Parameter | Type   | Description                                                                                                                     |
| --------- | ------ | ------------------------------------------------------------------------------------------------------------------------------- |
| media_id  | `&str` | takes media id or url as a parameter. (*media id or url can be found in the media search results as shown on the above method*) |

```rs
    pub async fn info(&self, media_id: &str) -> anyhow::Result<<provider_name>Info> {
        todo!()
    }
```

## Servers
| Parameter  | Type   | Description                                                                                                   |
| ---------- | ------ | ------------------------------------------------------------------------------------------------------------- |
| episode_id | `&str` | take an episode id or url as a parameter. (*episode id or episode url can be found in the media info object*) |
| mediaId    | `&str` | takes media id as a parameter. (*media id can be found in the media info object*)                             |

```rs
    pub async fn servers(
        &self,
        episode_id: &str,
        media_id: &str,
    ) -> anyhow::Result<Vec<IEpisodeServer>> {
        todo!()
    }
```
## Sources

| Parameter         | Type                                                                                                             | Description                                                                                                                                                        |
| ----------------- | ---------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| episode_id        | `&str`                                                                                                           | takes episode id as a parameter. (*episode id can be found in the media info object*)                                                                              |
| media_id          | `&str`                                                                                                           | takes media id as a parameter. (*media id can be found in the media info object*)                                                                                  |
| server (optional) | [`StreamingServers`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L170-L183) | takes server enum as a parameter. *default: [`StreamingServers::VidCloud`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L177)* |

```rs
    pub async fn sources(
        &self,
        episode_id: &str,
        media_id: &str,
        server: Option<StreamingServers>,
    ) -> anyhow::Result<ISource> {
        todo!()
    }
```
