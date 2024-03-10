<h1>FlixHQ</h1>

```rs
use consumet::providers::movies;

let flixhq = movies::FlixHQ;
```

<h2>Methods</h2>

- [search](#search)
- [info](#info)
- [sources](#sources)
- [servers](#servers)
- [recent\_movies](#recent_movies)
- [recent\_shows](#recent_shows)
- [trending\_movies](#trending_movies)
- [trending\_shows](#trending_shows)

### search
<h4>Parameters</h4>

| Parameter       | Type     | Description                                                                                                                                |
| --------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| query           | `string` | query to search for. (*In this case, We're searching for `Vincenzo`*) P.S: `vincenzo` is a really good korean drama i highly recommend it. |
| page (optional) | `number` | page number (default: 1)                                                                                                                   |

```rust
let data = flixhq.search("Vincenzo", None).await?;
println!("{:#?}", data);
```

returns a future which resolves into FlixHQSearchResults. (*[`impl Future<Output = Result<FlixHQSearchResults>>`](https://github.com/eatmynerds/consumet.rs/blob/master/src/providers/movies/flixhq.rs#L60-L68)*)\
            output:
```rust
FlixHQSearchResults {
    current_page: 1,
    has_next_page: false,
    total_pages: 1,
    total_results: 1,
    results: [
        FlixHQResult {
            id: "tv/watch-vincenzo-67955",
            cover: "https://img.flixhq.to/xxrz/1200x600/379/54/ed/54ed3e2164e4efa4c9ccc248e03f0032/54ed3e2164e4efa4c9ccc248e03f0032.jpg",
            title: "Vincenzo",
            url: "https://flixhq.to/tv/watch-vincenzo-67955",
            image: "https://img.flixhq.to/xxrz/250x400/379/79/6b/796b32989cf1308b9e0619524af5b022/796b32989cf1308b9e0619524af5b022.jpg",
            release_date: "2021-02-20",
            media_type: TvSeries,
            genres: [
                "Action & Adventure",
                "Crime",
            ],
            decription: "At age of 8, Park Joo-Hyung went to Italy after he was adopted. He is now an adult and has the name of Vincenzo Cassano. He is lawyer, who works for the Mafia as a consigliere. Because of a war between mafia groups, he flies to South Korea. In Korea, he gets involved with lawyer Hong Cha-Young. She is the type of attorney who will do anything to win a case. Vincenzo Cassano falls in love with her. He also achieves social justice by his own way.",
            rating: "8.4",
            quality: "HD",
            duration: "60 min",
            country: [
                "South Korea",
            ],
            production: [
                "Studio Dragon",
                "Logos Film",
            ],
            casts: [
                "Kwak Dong-yeon",
                "Kim yeo-jin",
                "Ok Taec-yeon",
                "Jeon Yeo-been",
                "Song Joong-ki",
            ],
            tags: [
                "Watch Vincenzo Online Free",
                "Vincenzo Online Free",
                "Where to watch Vincenzo",
                "Vincenzo movie free online",
                "Vincenzo free online",
            ],
        },
        {...}
        ...
    ],
}
```

### info

<h4>Parameters</h4>

| Parameter | Type     | Description                                                                                                                     |
| --------- | -------- | ------------------------------------------------------------------------------------------------------------------------------- |
| mediaId   | `string` | takes media id or url as a parameter. (*media id or url can be found in the media search results as shown on the above method*) |

```rust
let data = flixhq.info("tv/watch-vincenzo-67955").await?;
println!("{:#?}", data);
```

returns a future which resolves into an enum containing extra media info (including the episodes). (*[`impl Future<Output = Result<FlixHQInfo>>`](https://github.com/eatmynerds/consumet.rs/blob/master/src/providers/movies/flixhq.rs#L90-L94)*)\
output:
```rust
Tv(
    FlixHQShow {
        id: "tv/watch-vincenzo-67955",
        cover: "https://img.flixhq.to/xxrz/1200x600/379/54/ed/54ed3e2164e4efa4c9ccc248e03f0032/54ed3e2164e4efa4c9ccc248e03f0032.jpg",
        title: "Vincenzo",
        url: "https://flixhq.to/tv/watch-vincenzo-67955",
        image: "https://img.flixhq.to/xxrz/250x400/379/79/6b/796b32989cf1308b9e0619524af5b022/796b32989cf1308b9e0619524af5b022.jpg",
        release_date: "2021-02-20",
        media_type: TvSeries,
        genres: [
            "Action & Adventure",
            "Crime",
        ],
        description: "At age of 8, Park Joo-Hyung went to Italy after he was adopted. He is now an adult and has the name of Vincenzo Cassano. He is lawyer, who works for the Mafia as a consigliere. Because of a war between mafia groups, he flies to South Korea. In Korea, he gets involved with lawyer Hong Cha-Young. She is the type of attorney who will do anything to win a case. Vincenzo Cassano falls in love with her. He also achieves social justice by his own way.",
        rating: "8.4",
        quality: "HD",
        duration: "60 min",
        country: [
            "South Korea",
        ],
        production: [
            "Studio Dragon",
            "Logos Film",
        ],
        casts: [
            "Kwak Dong-yeon",
            "Kim Yeo-jin",
            "Ok Taec-yeon",
            "Jeon Yeo-been",
            "Song Joong-ki",
        ],
        tags: [
            "Watch Vincenzo Online Free",
            "Vincenzo Online Free",
            "Where to watch Vincenzo",
            "Vincenzo movie free online",
            "Vincenzo free online",
        ],
        total_episodes: 20,
        seasons: FlixHQSeason {
            total_seasons: 1,
            episodes: [
                [
                    FlixHQEpisode {
                        id: "1167571",
                        title: "Eps 1: Episode 1",
                        url: "https://flixhq.to/ajax/v2/episode/servers/1167571",
                    },
                    {...}
                ],
            ],
        },
    },
)
```

### servers

<h4>Parameters</h4>

| Parameter | Type     | Description                                                                                                   |
| --------- | -------- | ------------------------------------------------------------------------------------------------------------- |
| episodeId | `string` | take an episode id or url as a parameter. (*episode id or episode url can be found in the media info object*) |
| mediaId   | `string` | takes media id as a parameter. (*media id can be found in the media info object*)                             |

```rust
let data = flixhq.servers("1167571", "tv/watch-vincenzo-67955").await?;
println!("{:#?}", data);
```
returns a future which resolves into FlixHQServers (*[`impl Future<Output = Result<FlixHQServers>>`](https://github.com/eatmynerds/consumet.rs/blob/master/src/providers/movies/flixhq.rs#L36-L39)*)\
output:
```rust
FlixHQServers {
    servers: [
        FlixHQServer {
            name: "UpCloud",
            url: "https://flixhq.to/watch-tv/watch-vincenzo-67955.4829542",
        },
        FlixHQServer {
            name: "Vidcloud",
            url: "https://flixhq.to/watch-tv/watch-vincenzo-67955.4087001",
        },
        FlixHQServer {
            name: "Voe",
            url: "https://flixhq.to/watch-tv/watch-vincenzo-67955.7823107",
        },
        FlixHQServer {
            name: "DoodStream",
            url: "https://flixhq.to/watch-tv/watch-vincenzo-67955.4087002",
        },
        FlixHQServer {
            name: "MixDrop",
            url: "https://flixhq.to/watch-tv/watch-vincenzo-67955.6488473",
        },
    ],
}
```

### sources

<h4>Parameters</h4>

| Parameter         | Type                                                                                                               | Description                                                                                                                                                          |
| ----------------- | ------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| episodeId         | `string`                                                                                                           | takes episode id as a parameter. (*episode id can be found in the media info object*)                                                                                |
| mediaId           | `string`                                                                                                           | takes media id as a parameter. (*media id can be found in the media info object*)                                                                                    |
| server (optional) | [`StreamingServers`](https://github.com/eatmynerds/consumet.rs/blob/master/src/models/types.rs#L185-L198) | takes server enum as a parameter. *default: [`StreamingServers::VidCloud`](https://github.com/consumet-rs/consumet.rs/blob/master/src/models/types.rs#L177)* |


```rust
let data = flixhq.sources("1167571", "tv/watch-vincenzo-67955", None).await?;
println!("{:#?}", data);
```
returns a future which resolves into FlixHQSource. (*[`impl Future<Output = Result<FlixHQSource>>`](https://github.com/eatmynerds/consumet.rs/blob/master/src/providers/movies/flixhq.rs#L29-L34)*)\
output:


```rust
FlixHQSource {
    headers: "https://rabbitstream.net/embed-4/61fuoYgDxBPQ?z=",
    subtitles: VidCloud(
        [
            VidCloudSubtitle {
                url: "https://cc.2cdns.com/26/7f/267fbca84e18437aa7c7df80179b0751/ara-3.vtt",
                lang: "Arabic - Arabic",
            },
            {...}
        ],
    ),
    sources: VidCloud(
        [
            VidCloudSource {
                url: "https://o.pollllop.com/_v11/01b3e0bf48e643923f849702a32bd97a5c4360797759b0838c8f34597271ed8bf541e616b85a255a1320417863fe1980c9c6d12d471fb6d7961711321a2d9cb1be23897428798cbcc3b97d9d706357ecb6da5d1fb3c16fd51a4a691c0f014cc2148227666bb1235192ae7bb1a52b6db8fd6cd2f2300471e000680b1d06acd6b3b96b32a7519e7daae0044abdf5e0f3d4/playlist.m3u8",
                quality: "auto",
                is_m3u8: true,
            },
            {...}
        ],
    ),
}
```



<p align="end">(<a href="https://github.com/eatmynerds/consumet.rs/blob/master/docs/guides/movies.md#">back to movie providers list</a>)</p>

### recent_movies

```rust
let data = flixhq.recent_movies().await?;
println!("{:#?}", data)
```

returns a future which resolves into an vector of movies. (*[`impl Future<Output = Result<Vec<FlixHQResult>>>`](https://github.com/eatmynerds/consumet.rs/blob/master/src/providers/movies/flixhq.rs#L70-L88)*)\
output:
```rust
[
    FlixHQResult {
        id: "movie/watch-one-life-106294",
        cover: "https://img.flixhq.to/xxrz/1200x600/379/ab/ef/abef2a8a5c2bdf33b12bb0c4dc17408a/abef2a8a5c2bdf33b12bb0c4dc17408a.jpg",
        title: "One Life",
        url: "https://flixhq.to/movie/watch-one-life-106294",
        image: "https://img.flixhq.to/xxrz/250x400/379/60/fc/60fc7e3848aa007433e8ad399273f438/60fc7e3848aa007433e8ad399273f438.jpg",
        release_date: "2023-09-09",
        media_type: Movie,
        genres: [
            "Drama",
            "History",
            "War",
        ],
        description: "British stockbroker Nicholas Winton visits Czechoslovakia in the 1930s and forms plans to assist in the rescue of Jewi
sh children before the onset of World War II, in an operation that came to be known as the Kindertransport.",
        rating: "7.6",
        quality: "HD",
        duration: "109 min",
        country: [
            "United Kingdom",
            "United States of America",
        ],
        production: [
            "See-Saw Films",
            "MBK Productions",
            "BBC Film",
            "FilmNation Entertainment",
            "Cross City Films",
            "Lipsync Productions",
            "BBC Films",
        ],
        casts: [
            "Matilda Thorpe",
            "Alex Sharp",
            "Ziggy Heath",
            "Samantha Spiro",
            "Samuel Finzi",
        ],
        tags: [
            "Watch One Life Online Free",
            "One Life Online Free",
            "Where to watch One Life",
            "One Life movie free online",
            "One Life free online",
        ],
    },
    {...}
]
```


### recent_shows

```rust
let data = flixhq.recent_shows().await?;
println!("{:#?}", data);
```

returns a future which resolves into an vector of tv shows. (*[`impl Future<Output = Result<Vec<FlixHQResult>>>`](https://github.com/eatmynerds/consumet.rs/blob/master/src/providers/movies/flixhq.rs#L70-L88)*)\
output:
```rust
[
    FlixHQResult {
        id: "tv/watch-summer-house-34273",
        cover: "https://img.flixhq.to/xxrz/1200x600/379/3e/3b/3e3b9887b6ccc204fce5ac525461d674/3e3b9887b6ccc204fce5ac525461d674.jpg",
        title: "Summer House",
        url: "https://flixhq.to/tv/watch-summer-house-34273",
        image: "https://img.flixhq.to/xxrz/250x400/379/1b/49/1b49f07d6b32574cc101e4f841f409f5/1b49f07d6b32574cc101e4f841f409f5.jpg",
        release_date: "2017-01-13",
        media_type: TvSeries,
        genres: [
            "Reality",
        ],
        description: "Take the beach town of Montauk, New York by storm while following a group of nine friends who make the exclusive enclave their go-to party spot between Memorial Day and Labor Day — that is, when they're not hustling at their day jobs. You could say they work hard and party way harder.",
        rating: "6",
        quality: "HD",
        duration: "45 min",
        country: [
            "N/A",
        ],
        production: [
            "N/A",
        ],
        casts: [
            "Amanda Batula",
            "Danielle Olivera",
            "Kyle Cooke",
            "Lindsay Hubbard",
            "Carl Radke",
        ],
        tags: [
            "Watch Summer House Online Free",
            "Summer House Online Free",
            "Where to watch Summer House",
            "Summer House movie free online",
            "Summer House free online",
        ],
    },
    {...}
]
```


### trending_movies

```rust
let data = flixhq.trending_movies().await?;
println!("{:#?}", data);
```

returns a future which resolves into an vector of movies. (*[`impl Future<Output = Result<Vec<FlixHQResult>>>`](https://github.com/eatmynerds/consumet.rs/blob/master/src/providers/movies/flixhq.rs#L70-L88)*)\
output:
```rust
[
    FlixHQResult {
        id: "movie/watch-the-nun-ii-100063",
        cover: "https://img.flixhq.to/xxrz/1200x600/379/85/09/8509eafa6b10694de8fe9027e415ecdf/8509eafa6b10694de8fe9027e415ecdf.jpg",
        title: "The Nun II",
        url: "https://flixhq.to/movie/watch-the-nun-ii-100063",
        image: "https://img.flixhq.to/xxrz/250x400/379/82/9f/829fd2ab2816811c59f600deedb524ed/829fd2ab2816811c59f600deedb524ed.jpg",
        release_date: "2023-09-06",
        media_type: Movie,
        genres: [
            "Horror",
            "Mystery",
            "Thriller",
        ],
        description: "Set four years after the ending of the the nun, this follows Sister Irene as she investigates a murder at a boarding school in France. While investiga
ting she is once again forced to face the demonic force Valak, the Nun.",
        rating: "5.9",
        quality: "HD",
        duration: "110 min",
        country: [
            "United States of America",
        ],
        production: [
            "New Line Cinema",
            "Atomic Monster",
            "The Safran Company",
        ],
        casts: [
            "Taissa Farmiga",
            "Jonas Bloquet",
            "Bonnie Aarons",
            "Storm Reid",
            "Katelyn Rose Downey",
        ],
        tags: [
            "Watch The Nun II Online Free",
            "The Nun II Online Free",
            "Where to watch The Nun II",
            "The Nun II movie free online",
            "The Nun II free online",
        ],
    },
    {...}
]
```


### trending_shows

```rust
let data = flixhq.trending_shows().await?;
println!("{:#?}", data);
```

returns a future which resolves into an vector of tv shows. (*[`impl Future<Output = Result<Vec<FlixHQResult>>>`](https://github.com/eatmynerds/consumet.rs/blob/master/src/providers/movies/flixhq.rs#L70-L88)*)\
output:
```rust
[
    FlixHQResult {
        id: "tv/watch-avatar-the-last-airbender-106435",
        cover: "https://img.flixhq.to/xxrz/1200x600/379/a4/e6/a4e6e9b6f934285f0fe97a0bc9544e87/a4e6e9b6f934285f0fe97a0bc9544e87.jpg",
        title: "Avatar: The Last Airbender",
        url: "https://flixhq.to/tv/watch-avatar-the-last-airbender-106435",
        image: "https://img.flixhq.to/xxrz/250x400/379/f1/a8/f1a8e2f3b5eeae90eaa3864a64febff3/f1a8e2f3b5eeae90eaa3864a64febff3.jpg",
        release_date: "2024-02-22",
        media_type: TvSeries,
        genres: [
            "Sci-Fi & Fantasy",
            "Action & Adventure",
            "Drama",
        ],
        description: "A young boy known as the Avatar must master the four elemental powers to save a world at war — and fight a ruthless enemy bent on stopping him.",
        rating: "7.5",
        quality: "HD",
        duration: "50 min",
        country: [
            "United States of America",
        ],
        production: [
            "Nickelodeon Productions",
            "Rideback",
        ],
        casts: [
            "Kay Siu Lim",
            "Dallas Liu",
            "Ken Leung",
            "Elizabeth Yu",
            "Casey Camp-Horinek",
        ],
        tags: [
            "Watch Avatar: The Last Airbender Online Free",
            "Avatar: The Last Airbender Online Free",
            "Where to watch Avatar: The Last Airbender",
            "Avatar: The Last Airbender movie free online",
            "Avatar: The Last Airbender free online",
        ],
    },
    {...}
]
```
