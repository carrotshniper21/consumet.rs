<h1>FlixHQ</h1>

```rs
use consumet_api_rs::models::{MovieParser, BaseParser};
use consumet_api_rs::providers::movies;

let flixhq = movies::FlixHQ;
```

<h2>Methods</h2>

- [search](#search)
- [fetch\_media\_info](#fetch_media_info)
- [fetch\_episode\_sources](#fetch_episode_sources)
- [fetch\_episode\_servers](#fetch_episode_servers)
- [fetch\_recent\_movies](#fetch_recent_movies)
- [fetch\_recent\_shows](#fetch_recent_shows)
- [fetch\_trending\_movies](#fetch_trending_movies)
- [fetch\_trending\_shows](#fetch_trending_shows)

### search
> Note: This method is a subclass of the [`BaseParser`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/base_parser.rs) trait. meaning it is available across most categories.


<h4>Parameters</h4>

| Parameter       | Type     | Description                                                                                                                                |
| --------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| query           | `string` | query to search for. (*In this case, We're searching for `Vincenzo`*) P.S: `vincenzo` is a really good korean drama i highly recommend it. |
| page (optional) | `number` | page number (default: 1)                                                                                                                   |

```rust
let data = flixhq.search("Vincenzo".to_owned(), None).await?;
println!("{:#?}", data);
```

returns a future which resolves into an vector of movies/tv series. (*[`impl Future<Output = Result<ISearch<Vec<IMovieResult>>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L452-L462)*)\
            output:
```
ISearch {
    current_page: Some(
        1,
    ),
    has_next_page: Some(
        false,
    ),
    total_pages: 1,
    total_results: 1,
    results: [
        IMovieResult {
            id: Some(
                "tv/watch-vincenzo-67955",
            ),
            cover: Some(
                "https://img.flixhq.to/xxrz/1200x600/379/54/ed/54ed3e2164e4efa4c9ccc248e03f0032/54ed3e2164e4efa4c9ccc248e03f0032.jpg",
            ),
            title: Some(
                "Vincenzo",
            ),
            url: Some(
                "https://flixhq.to/tv/watch-vincenzo-67955",
            ),
            image: Some(
                "https://img.flixhq.to/xxrz/250x400/379/79/6b/796b32989cf1308b9e0619524af5b022/796b32989cf1308b9e0619524af5b022.jpg",
            ),
            release_date: Some(
                "2021-02-20",
            ),
            media_type: Some(
                TvSeries,
            ),
        },
        {...}
        ...
    ],
}
```

### fetch_media_info

<h4>Parameters</h4>

| Parameter | Type     | Description                                                                                                                     |
| --------- | -------- | ------------------------------------------------------------------------------------------------------------------------------- |
| mediaId   | `string` | takes media id or url as a parameter. (*media id or url can be found in the media search results as shown on the above method*) |

```rust
let data = flixhq.fetch_media_info("tv/watch-vincenzo-67955".to_owned()).await?;
println!("{:#?}", data);
```

returns a future which resolves into an movie info object (including the episodes). (*[`impl Future<Output = Result<FlixHQInfo>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/providers/movies/flixhq.rs#L22-L26)*)\
output:
```
FlixHQInfo {
    base: IMovieResult {
        id: Some(
            "tv/watch-vincenzo-67955",
        ),
        cover: Some(
            "https://img.flixhq.to/xxrz/1200x600/379/54/ed/54ed3e2164e4efa4c9ccc248e03f0032/54ed3e2164e4efa4c9ccc248e03f0032.jpg",
        ),
        title: Some(
            "Vincenzo",
        ),
        url: Some(
            "https://flixhq.to/tv/watch-vincenzo-67955",
        ),
        image: Some(
            "https://img.flixhq.to/xxrz/250x400/379/79/6b/796b32989cf1308b9e0619524af5b022/796b32989cf1308b9e0619524af5b022.jpg",
        ),
        release_date: Some(
            "2021-02-20",
        ),
        media_type: Some(
            TvSeries,
        ),
    },
    info: IMovieInfo {
        genres: Some(
            [
                "Action & Adventure",
                "Crime",
            ],
        ),
        description: Some(
            "At age of 8, Park Joo-Hyung went to Italy after he was adopted. He is now an adult and has the name of Vincenzo Cassano. He is lawyer, who works for the Mafia as a consigliere. Because of a war between mafia groups, he flies to South Korea. In Korea, he gets involved with lawyer Hong Cha-Young. She is the type of attorney who will do anything to win a case. Vincenzo Cassano falls in love with her. He also achieves social justice by his own way.",
        ),
        rating: Some(
            "8.4",
        ),
        status: None,
        duration: Some(
            "60 min",
        ),
        country: Some(
            [
                "South Korea",
            ],
        ),
        production: Some(
            [
                "Studio Dragon",
                "Logos Film",
            ],
        ),
        casts: Some(
            [
                "Kwak Dong-yeon",
                "Kim Yeo-jin",
                "Ok Taec-yeon",
                "Jeon Yeo-been",
                "Song Joong-ki",
            ],
        ),
        tags: Some(
            [
                "Watch Vincenzo Online Free",
                "Vincenzo Online Free",
                "Where to watch Vincenzo",
                "Vincenzo movie free online",
                "Vincenzo free online",
            ],
        ),
        total_episodes: Some(
            20,
        ),
        seasons: Some(
            IMovieSeason {
                season: 1,
                image: None,
                episodes: Some(
                    [
                        [
                            IMovieEpisode {
                                id: "1167571",
                                title: "Eps 1: Episode #1.1",
                                url: Some(
                                    "https://flixhq.to/ajax/v2/episode/servers/1167571",
                                ),
                                number: None,
                                season: Some(
                                1,
                            ),
                            description: None,
                            image: None,
                            release_date: None,
                            },
                            {...}
                        ],
                    ],
                ),
            },
        ),
        episodes: Some(
            [
                [
                    IMovieEpisode {
                        id: "1167571",
                        title: "Eps 1: Episode #1.1",
                        url: Some(
                            "https://flixhq.to/ajax/v2/episode/servers/1167571",
                        ),
                        number: None,
                        season: Some(
                            1,
                        ),
                        description: None,
                        image: None,
                        release_date: None,
                    },
                    {...}
                ],
            ],
        ),
    },
}
```

### fetch_episode_sources

<h4>Parameters</h4>

| Parameter         | Type                                                                                                   | Description                                                                                                                                             |
| ----------------- | ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| episodeId         | `string`                                                                                               | takes episode id as a parameter. (*episode id can be found in the media info object*)                                                                   |
| mediaId           | `string`                                                                                               | takes media id as a parameter. (*media id can be found in the media info object*)                                                                       |
| server (optional) | [`StreamingServers`](https://github.com/carrotshniper21/consumet-api-rs/blob/master/src/models/types.ts#L166-L183) | takes server enum as a parameter. *default: [`StreamingServers::VidCloud`](https://github.com/carrotshniper21/consumet-api-rs/blob/master/src/models/types.rs#L177)* |


```rust
let data = flixhq.fetch_episode_sources("1167571".to_owned(), "tv/watch-vincenzo-67955".to_owned(), None).await?;
println!("{:#?}", data);
```
returns a future which resolves into an vector of episode sources and subtitles. (*[`impl Future<Output = Result<ISource>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L374-L380)*)\
output:


```
ISource {
    headers: Some(
        "https://dokicloud.one/embed-4/UCOzqDHOU3UO?z=",
    ),
    intro: None,
    subtitles: Some(
        [
            ISubtitle {
                id: None,
                url: Some(
                    "https://cc.2cdns.com/26/7f/267fbca84e18437aa7c7df80179b0751/ara-3.vtt",
                ),
                lang: Some(
                    "Arabic - Arabic",
                ),
            },
            ISubtitle {
                id: None,
                url: Some(
                    "https://cc.2cdns.com/26/7f/267fbca84e18437aa7c7df80179b0751/chi-4.vtt",
                ),
                lang: Some(
                    "Chinese - Chinese Simplified",
                ),
            },
            {...} 
            ...
        ],
    ),
    sources: Some(
        [
            IVideo {
                url: Some(
                    "https://owt.webarchivecdn.com/_v10/01b3e0bf48e643923f849702a32bd97a5c4360797759b0838c8f34597271ed8bf541e616b85a255a1320417863fe1980c9c6d12d471fb6d7961711321a2d9cb1be23897428798cbcc3b97d9d706357ecb6da5d1fb3c16fd51a4a691c0f014cc2148227666bb1235192ae7bb1a52b6db8cb2a29c52f47094bce0efb39fcc9eb6e16950cd25ec3872f80cf24cc2632ef1c/playlist.m3u8",
                ),
                quality: Some(
                    "auto",
                ),
                is_m3u8: Some(
                    true,
                ),
                is_dash: None,
                size: None,
                other: None,
            },
            {...} 
            ...
        ],
    ),
}
```

### fetch_episode_servers

<h4>Parameters</h4>

| Parameter | Type     | Description                                                                                                   |
| --------- | -------- | ------------------------------------------------------------------------------------------------------------- |
| episodeId | `string` | take an episode id or url as a parameter. (*episode id or episode url can be found in the media info object*) |
| mediaId   | `string` | takes media id as a parameter. (*media id can be found in the media info object*)                             |

```rust
let data = flixhq.fetch_episode_servers("1167571".to_owned(), "tv/watch-vincenzo-67955".to_owned()).await?;
println!("{:#?}", data);
```
returns a future which resolves into an vector of episode servers. (*[`impl Future<Output = Result<Vec<IEpisodeServer>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L148-L153)*)\
output:
```
[
    IEpisodeServer {
        name: "UpCloud",
        url: "https://flixhq.to/watch-tv/watch-vincenzo-67955.4829542",
    },
    IEpisodeServer {
        name: "Vidcloud",
        url: "https://flixhq.to/watch-tv/watch-vincenzo-67955.4087001",
    },
    IEpisodeServer {
        name: "Voe",
        url: "https://flixhq.to/watch-tv/watch-vincenzo-67955.7823107",
    },
    {...},
    ...
]
```

<p align="end">(<a href="https://github.com/carrotshniper21/consumet-api-rs/blob/main/docs/guides/movies.md#">back to movie providers list</a>)</p>

### fetch_recent_movies

```rust
let data = flixhq.fetch_recent_movies().await?;
println!("{:#?}", data)
```

returns a future which resolves into an vector of movies. (*[`impl Future<Output = Result<Vec<IMovieResult>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L452-L462)*)\
output:
```
[
    IMovieResult {
        id: Some(
            "movie/watch-the-little-mermaid-88243",
        ),
        cover: Some(
            "https://img.flixhq.to/xxrz/1200x600/379/17/c1/17c19945d0546aa21c3ce84b4eef22f4/17c19945d0546aa21c3ce84b4eef22f4.jpg",
        ),
        title: Some(
            "The Little Mermaid",
        ),
        url: Some(
            "https://flixhq.to/movie/watch-the-little-mermaid-88243",
        ),
        image: Some(
            "https://img.flixhq.to/xxrz/250x400/379/fc/7d/fc7de6b957d0dfb5f3b719c812093766/fc7de6b957d0dfb5f3b719c812093766.jpg",
        ),
        release_date: Some(
            "2023-05-24",
        ),
        media_type: Some(
            Movie,
        ),
    },
    IMovieResult {
        id: Some(
            "movie/watch-justice-league-warworld-98344",
        ),
        cover: Some(
            "https://img.flixhq.to/xxrz/1200x600/379/20/19/20197bf01bb04ccfec89b8c42261ed2e/20197bf01bb04ccfec89b8c42261ed2e.jpg",
        ),
        title: Some(
            "Justice League: Warworld",
        ),
        url: Some(
            "https://flixhq.to/movie/watch-justice-league-warworld-98344",
        ),
        image: Some(
            "https://img.flixhq.to/xxrz/250x400/379/d4/6e/d46e0478e7349f8b2fecc134d8940286/d46e0478e7349f8b2fecc134d8940286.jpg",
        ),
        release_date: Some(
            "2023-07-25",
        ),
        media_type: Some(
            Movie,
        ),
    },
    IMovieResult {
        id: Some(
            "movie/watch-the-venture-bros-radiant-is-the-blood-of-the-baboon-heart-98488",
        ),
        cover: Some(
            "https://img.flixhq.to/xxrz/1200x600/379/28/f9/28f9654cdc5640a23ad5a897f2d58d3d/28f9654cdc5640a23ad5a897f2d58d3d.jpg",
        ),
        title: Some(
            "The Venture Bros.: Radiant is the Blood of the Baboon Heart",
        ),
        url: Some(
            "https://flixhq.to/movie/watch-the-venture-bros-radiant-is-the-blood-of-the-baboon-heart-98488",
        ),
        image: Some(
            "https://img.flixhq.to/xxrz/250x400/379/a9/82/a982fdb426f0216725255b2e7a1a8b21/a982fdb426f0216725255b2e7a1a8b21.jpg",
        ),
        release_date: Some(
            "2023-07-21",
        ),
        media_type: Some(
            Movie,
        ),
    },
    {...},
    ...
]
```


### fetch_recent_shows

```rust
let data = flixhq.fetch_recent_shows().await?;
println!("{:#?}", data);
```

returns a future which resolves into an vector of tv shows. (*[`impl Future<Output = Result<Vec<IMovieResult>>>`](https://github.com//consumet-api-rs/blob/master/src/models/types.rs#L452-L462)*)\
output:
```
[
    IMovieResult {
        id: Some(
            "tv/watch-null-98587",
        ),
        cover: Some(
            "https://img.flixhq.to/xxrz/1200x600/379/df/78/df789708304f2e051d8292b8405f000f/df789708304f2e051d8292b8405f000f.jpg",
        ),
        title: Some(
            "The Everlasting Love",
        ),
        url: Some(
            "https://flixhq.to/tv/watch-null-98587",
        ),
        image: Some(
            "https://img.flixhq.to/xxrz/250x400/379/a9/fa/a9faae56016d18d0202d1c29f46e1151/a9faae56016d18d0202d1c29f46e1151.jpg",
        ),
        release_date: Some(
            "2023-06-08",
        ),
        media_type: Some(
            TvSeries,
        ),
    },
    IMovieResult {
        id: Some(
            "tv/watch-stay-with-me-98584",
        ),
        cover: Some(
            "https://img.flixhq.to/xxrz/1200x600/379/14/06/140639f49f4da20d968e1d9e682f5210/140639f49f4da20d968e1d9e682f5210.jpg",
        ),
        title: Some(
            "Stay With Me",
        ),
        url: Some(
            "https://flixhq.to/tv/watch-stay-with-me-98584",
        ),
        image: Some(
            "https://img.flixhq.to/xxrz/250x400/379/85/08/8508103d3eb891537a2b30a92636ef35/8508103d3eb891537a2b30a92636ef35.jpg",
        ),
        release_date: Some(
            "2023-07-07",
        ),
        media_type: Some(
            TvSeries,
        ),
    },
    IMovieResult {
        id: Some(
            "tv/watch-hijack-97780",
        ),
        cover: Some(
            "https://img.flixhq.to/xxrz/1200x600/379/35/bd/35bd068e760804a39144280edcca3aeb/35bd068e760804a39144280edcca3aeb.jpg",
        ),
        title: Some(
            "Hijack",
        ),
        url: Some(
            "https://flixhq.to/tv/watch-hijack-97780",
        ),
        image: Some(
            "https://img.flixhq.to/xxrz/250x400/379/bb/b3/bbb3557a25a80ce3023d454218495652/bbb3557a25a80ce3023d454218495652.jpg",
        ),
        release_date: Some(
            "2023-06-27",
        ),
        media_type: Some(
            TvSeries,
        ),
    },
    {...},
    ...
]
```


### fetch_trending_movies

```rust
let data = flixhq.fetch_trending_movies().await?;
println!("{:#?}", data);
```

returns a future which resolves into an vector of movies. (*[`impl Future<Output = Result<Vec<IMovieResult>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L452-L462)*)\
output:
```
[
    IMovieResult {
        id: Some(
            "movie/watch-the-little-mermaid-88243",
        ),
        cover: Some(
            "https://img.flixhq.to/xxrz/1200x600/379/17/c1/17c19945d0546aa21c3ce84b4eef22f4/17c19945d0546aa21c3ce84b4eef22f4.jpg",
        ),
        title: Some(
            "The Little Mermaid",
        ),
        url: Some(
            "https://flixhq.to/movie/watch-the-little-mermaid-88243",
        ),
        image: Some(
            "https://img.flixhq.to/xxrz/250x400/379/fc/7d/fc7de6b957d0dfb5f3b719c812093766/fc7de6b957d0dfb5f3b719c812093766.jpg",
        ),
        release_date: Some(
            "2023-05-24",
        ),
        media_type: Some(
            Movie,
        ),
    },
    IMovieResult {
        id: Some(
            "movie/watch-barbie-693",
        ),
        cover: Some(
            "https://img.flixhq.to/xxrz/1200x600/379/48/62/48627a6116659665a1c4dc208a80d218/48627a6116659665a1c4dc208a80d218.jpg",
        ),
        title: Some(
            "Barbie",
        ),
        url: Some(
            "https://flixhq.to/movie/watch-barbie-693",
        ),
        image: Some(
            "https://img.flixhq.to/xxrz/250x400/379/b2/a2/b2a2078a74e2e3b53693554ce3fbdd64/b2a2078a74e2e3b53693554ce3fbdd64.jpg",
        ),
        release_date: Some(
            "2023-07-19",
        ),
        media_type: Some(
            Movie,
        ),
    },
    IMovieResult {
        id: Some(
            "movie/watch-resident-evil-death-island-98491",
        ),
        cover: Some(
            "https://img.flixhq.to/xxrz/1200x600/379/6e/09/6e092181babf80c690b2f8acb47038e6/6e092181babf80c690b2f8acb47038e6.jpg",
        ),
        title: Some(
            "Resident Evil: Death Island",
        ),
        url: Some(
            "https://flixhq.to/movie/watch-resident-evil-death-island-98491",
        ),
        image: Some(
            "https://img.flixhq.to/xxrz/250x400/379/e2/02/e202f50399e893e25922555a4332d8ee/e202f50399e893e25922555a4332d8ee.jpg",
        ),
        release_date: Some(
            "2023-06-22",
        ),
        media_type: Some(
            Movie,
        ),
    },
    {...},
    ...
]
```


### fetch_trending_shows

```rust
let data = flixhq.fetch_trending_shows().await?;
println!("{:#?}", data);
```

returns a future which resolves into an vector of tv shows. (*[`impl Future<Output = Result<Vec<IMovieResult>>>`](https://github.com/carrotshniper21/consumet-api-rs/blob/main/src/models/types.rs#L452-L462)*)\
output:
```
[
    IMovieResult {
        id: Some(
            "tv/watch-null-98587",
        ),
        cover: Some(
            "https://img.flixhq.to/xxrz/1200x600/379/df/78/df789708304f2e051d8292b8405f000f/df789708304f2e051d8292b8405f000f.jpg",
        ),
        title: Some(
            "The Everlasting Love",
        ),
        url: Some(
            "https://flixhq.to/tv/watch-null-98587",
        ),
        image: Some(
            "https://img.flixhq.to/xxrz/250x400/379/a9/fa/a9faae56016d18d0202d1c29f46e1151/a9faae56016d18d0202d1c29f46e1151.jpg",
        ),
        release_date: Some(
            "2023-06-08",
        ),
        media_type: Some(
            TvSeries,
        ),
    },
    IMovieResult {
        id: Some(
            "tv/watch-stay-with-me-98584",
        ),
        cover: Some(
            "https://img.flixhq.to/xxrz/1200x600/379/14/06/140639f49f4da20d968e1d9e682f5210/140639f49f4da20d968e1d9e682f5210.jpg",
        ),
        title: Some(
            "Stay With Me",
        ),
        url: Some(
            "https://flixhq.to/tv/watch-stay-with-me-98584",
        ),
        image: Some(
            "https://img.flixhq.to/xxrz/250x400/379/85/08/8508103d3eb891537a2b30a92636ef35/8508103d3eb891537a2b30a92636ef35.jpg",
        ),
        release_date: Some(
            "2023-07-07",
        ),
        media_type: Some(
            TvSeries,
        ),
    },
    IMovieResult {
        id: Some(
            "tv/watch-hijack-97780",
        ),
        cover: Some(
            "https://img.flixhq.to/xxrz/1200x600/379/35/bd/35bd068e760804a39144280edcca3aeb/35bd068e760804a39144280edcca3aeb.jpg",
        ),
        title: Some(
            "Hijack",
        ),
        url: Some(
            "https://flixhq.to/tv/watch-hijack-97780",
        ),
        image: Some(
            "https://img.flixhq.to/xxrz/250x400/379/bb/b3/bbb3557a25a80ce3023d454218495652/bbb3557a25a80ce3023d454218495652.jpg",
        ),
        release_date: Some(
            "2023-06-27",
        ),
        media_type: Some(
            TvSeries,
        ),
    },
    {...},
    ...
]
```
