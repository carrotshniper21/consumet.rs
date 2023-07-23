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
The `movies::FlixHQ.search` function is used to search for movies with the example search query `"hi"` and an optional page number `1`. Then the results are printed using the println! macro.

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

The `movies::FlixHQ.fetch_media_info` function is used to fetch info for movies with their corresponding id for example: `"tv/watch-yo-mtv-raps-82018"`. Then the results are printed using the println! macro.

# TODO

Please note that currently, only the searching functionality for the flixhq movie provider is available. Additional functionality, such as retrieving detailed movie information or accessing other providers, will be added in future updates. 
