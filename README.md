<p align="center"><img src="https://consumet.org/images/consumetlogo.png" width="175"/></p>

<h1 align="center"> consumet.rs </h1>

consumet.rs is a crates.io library which provides high-level APIs to get information about several entertainment mediums like books, movies, comics, anime, manga, etc.

<p align="center">
    <a href="https://github.com/rust-lang/rust-clippy/actions?query=workflow%3A%22Clippy+Test+(bors)%22+event%3Apush+branch%3Aauto">
        <img src="https://github.com/rust-lang/rust-clippy/workflows/Clippy%20Test%20(bors)/badge.svg?branch=auto&event=push" alt="Clippy Test">
    </a>
    <a href="https://github.com/eatmynerds/consumet.rs/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/consumet/api" alt="GitHub">
  </a>
    <a href="https://crates.io/crates/consumet">
        <img src="https://img.shields.io/crates/v/consumet.svg">
    </a>
    <a href="https://discord.gg/qTPfvMxzNH">
      <img src="https://img.shields.io/discord/987492554486452315?color=7289da&label=discord&logo=discord&logoColor=7289da" alt="Official Discord">
    </a>
   
</p>

<h2> Table of Contents </h2>

- [Quick Start](#quick-start)
  - [Installation](#installation)
  - [Usage](#usage)
- [Documentation](#documentation)
- [Ecosystem](#ecosystem)
- [Support](#support)
- [License](#license)

## Quick Start

### Installation

To use consumet.rs in your project, run:
```bash
cargo install consumet
```

### Usage

**Example** - searching for a movie using the flixhq provider.
```rs
use consumet::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a new instance of the FlixHQ provider
    let flixhq = movies::FlixHQ;

    // Search for a movie. In this case, "Vincenzo"
    let data = flixhq.search("Vincenzo", None).await?;
    println!("{:#?}", data);

    Ok(())
}
```

Do you want to know more? Head to the [`Getting Started`](https://github.com/eatmynerds/consumet.rs/tree/master/docs/guides/getting-started.md).

## Documentation
- [`Getting Started`](https://github.com/eatmynerds/consumet.rs/tree/master/docs/guides/getting-started.md)
- [`Guides`](https://github.com/eatmynerds/consumet.rs/tree/master/docs)
- [`Movies`](https://github.com/eatmynerds/consumet.rs/tree/master/docs/guides/movies.md)

## Ecosystem
- [Provider Status](https://github.com/consumet/providers-status/blob/main/README.md) - A list of providers and their status.
- [Discord Server](https://discord.gg/qTPfvMxzNH) - Join the official discord server and chat with the maintainers.

## Support
You can contact the maintainer of consumet.rs via [email](mailto:vipershniper08@gmail.com), or [join the official discord server](https://discord.gg/qTPfvMxzNH) (Recommended).

<a href="https://discord.gg/qTPfvMxzNH">
   <img src="https://discordapp.com/api/guilds/987492554486452315/widget.png?style=banner2">
</a>

# Contributors
Checkout the original [`consumet.ts`](https://github.com/consumet/consumet.ts/)

Credits to the beautiful people who wrote the original consumet.ts

[![](https://contrib.rocks/image?repo=consumet/consumet.ts)](https://github.com/consumet/consumet.ts/graphs/contributors)

## License
Licensed under [GPL-3.0](./LICENSE).
