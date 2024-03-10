<h1 align="center">consumet.rs</h1>

## Getting Started

Hello! Thank you for checking out consumet.rs!

This document aims to be a gentle introduction to the library and its usage.

Let's start!

### Installation
Install with cargo:
```sh
cargo add consumet
```
### Usage

**Example** - searching for a film using the flixhq provider.
```rust
use consumet::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a new instance of the flixhq provider
    let flixhq = movies::FlixHQ;

    // Search for a movie. In this case, "Vincenzo"
    let results = flixhq.search("Vincenzo", None).await?;

    // Print the results
    println!("{:#?}", results);

    let movie_id = &results.results[0].id;

    // Get the first movie info
    let movie_info = flixhq.info(&movie_id).await?;

    // Print the info
    println!("{:#?}", movie_info);

    Ok(())
}
```
*see also [MOVIE documentation](./movies.md#movies) for more information.*\
Awesome, that was easy.

if you want to use different providers, you can check the providers list [here](https://consumet.org/extensions/list/) or in [json format](https://github.com/consumet/providers-status/blob/main/providers-list.json).

if you have any questions, please join the [official consumet api discord server](https://discord.gg/qTPfvMxzNH) or open an [issue](https://github.com/eatmynerds/consumet.rs/issues).

<p align="end">(<a href="https://github.com/eatmynerds/consumet.rs/blob/master/docs">back to table of contents</a>)</p>

