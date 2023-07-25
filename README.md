# Compile This With Nightly!

# Usage

## Movies

### Flixhq

To use the `FlixHQ` crate, you can follow this example:

#### Searching

```rust
use consumet_api_rs::models::MovieParser;
use consumet_api_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        "{:#?}",
        movies::FlixHQ.search("hi".to_owned(), Some(1)).await?
    );

    Ok(())
}
```

The `movies::FlixHQ.search` function is used to search for movies with the example search query `hi` and an optional
page number `1`.

#### Info

```rust
use consumet_api_rs::models::MovieParser;
use consumet_api_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        "{:#?}",
        movies::FlixHQ.fetch_media_info("tv/watch-yo-mtv-raps-82018".to_owned()).await?
    );

    Ok(())
}
```

The `movies::FlixHQ.fetch_media_info` function is used to fetch info for movies with their corresponding id for
example: `tv/watch-yo-mtv-raps-82018`.

#### Servers

```rust
use consumet_api_rs::models::MovieParser;
use consumet_api_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        "{:#?}",
        movies::FlixHQ
            .fetch_episode_servers(
                "98488".to_owned(),
                "movie/watch-the-venture-bros-radiant-is-the-blood-of-the-baboon-heart-98488"
                    .to_owned()
            )
            .await?
    );

    Ok(())
}
```

The `movies::FlixHQ.fetch_episode_servers` function is used to fetch servers for movies with their id `98488` and their
media id `movie/watch-the-venture-bros-radiant-is-the-blood-of-the-baboon-heart-98488`. But for shows the id field is the episode id instead.

#### Sources
```rust
use consumet_api_rs::models::{MovieParser, StreamingServers};
use consumet_api_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        "{:#?}",
        movies::FlixHQ
            .fetch_episode_sources(
                "1274950".to_owned(),
                "tv/watch-yo-mtv-raps-82018".to_owned(),
                Some(StreamingServers::UpCloud),
            )
            .await?
    );

    Ok(())
}
```
The `movies::FlixHQ.fetch_episode_sources` function is used to fetch sources for shows with their episode_id: `1274950` for films this would be the id. And the media id `movie/watch-the-venture-bros-radiant-is-the-blood-of-the-baboon-heart-98488`. But for shows the id field is the episode id instead. And the third paramter is the preferred server which can be `None` which defaults to UpCloud
