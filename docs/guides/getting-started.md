<h1 align="center">consumet-api-rs</h1>

## Getting Started

Hello! Thank you for checking out consumet-api-rs!

This document aims to be a gentle introduction to the library and its usage.

Let's start!

### Installation
Install with cargo:
```sh
cargo add consumet_api_rs
```
### Usage

**Example** - searching for a film using the flixhq provider.
```rust
use consumet_api_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a new instance of the flixhq provider
    let flixhq = movies::FlixHQ;
    // Search for a movie. In this case, "Vincenzo"
    let results = flixhq.search("Vincenzo", None).await?;
    // Print the results
    println!("{:#?}", results);

    // Get the first movie info
    let first_movie = results.results[0].id.clone().expect("No id found!");
    let movie_info = flixhq.info(first_movie.as_str()).await?;
    // Print the info
    println!("{:#?}", movie_info);

    Ok(())
}
```
*see also [MOVIE documentation](./movies.md#movies) for more information.*\
Awesome, that was easy.

if you want to use different providers, you can check the providers list [here](https://consumet.org/extensions/list/) or in [json format](https://github.com/consumet/providers-status/blob/main/providers-list.json).

if you have any questions, please join the [official consumet api discord server](https://discord.gg/qTPfvMxzNH) or open an [issue](https://github.com/carrotshniper21/consumet-api-rs/issues).

<p align="end">(<a href="https://github.com/consumet/extensions/blob/master/docs">back to table of contents</a>)</p>

