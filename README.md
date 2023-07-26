<p align="center"><img src="https://consumet.org/images/consumetlogo.png" width="175"/></p>

<h1 align="center"> consumet.ts </h1>

consumet.ts is a Node library which provides high-level APIs to get information about several entertainment mediums like books, movies, comics, anime, manga, etc.

<p align="center">
    <a href="https://discord.gg/qTPfvMxzNH">
      <img src="https://img.shields.io/discord/987492554486452315?color=7289da&label=discord&logo=discord&logoColor=7289da" alt="Discord">
    </a>
</p>

<h2> Table of Contents </h2>

- [Quick Start](#quick-start)
  - [Installation](#installation)
  - [Usage](#usage)
- [Documentation](#documentation)
- [Ecosystem](#ecosystem)
- [Provider Request](#provider-request)
- [Contributing](#contributing)
- [Support](#support)
- [Contributors ✨](#contributors-)
  - [Credits](#credits)
- [License](#license)

## Quick Start

### Installation

To use consumet-api-rs in your project, run:
```bash
cargo install consumet-api-rs
```

### Usage

**Example** - searching for a movie using the flixhq provider.
```rs
use consumet_api_rs::models::MovieParser;
use consumet_api_rs::providers::movies;

// Create a new instance of the FlixHQ provider
let flixhq = movies::FlixHQ;
// Search for a movie. In this case, "Vincenzo"
let data = flixhq.search("Vincenzo").await?;
println!("{:#?}", data);
```

Do you want to know more? Head to the [`Getting Started`](https://github.com/carrotshniper21/consumet-api-rs/tree/main/docs/guides/getting-started.md).

## Documentation
- [`Getting Started`](./docs/guides/getting-started.md)
- [`Guides`](https://github.com/consumet/consumet.ts/tree/master/docs)
- [`Anime`](./docs/guides/anime.md)
- [`Manga`](./docs/guides/manga.md)
- [`Books`](./docs/guides/books.md)
- [`Movies`](./docs/guides/movies.md)
- [`Light Novels`](./docs/guides/light-novels.md)
- [`Comics`](./docs/guides/comics.md)
- [`Meta`](./docs/guides/meta.md)
- [`News`](./docs/guides/news.md)

## Ecosystem
- [Provider Status](https://github.com/consumet/providers-status/blob/main/README.md) - A list of providers and their status.
- [Discord Server](https://discord.gg/qTPfvMxzNH) - Join the official discord server and chat with the maintainers.

## Support
You can contact the maintainer of consumet-api-rs via discord `eatmynerds`, or [join the official discord server](https://discord.gg/qTPfvMxzNH) (Recommended).

<a href="https://discord.gg/qTPfvMxzNH">
   <img src="https://discordapp.com/api/guilds/987492554486452315/widget.png?style=banner2">
</a>

## License
Licensed under [APACHE](./LICENSE).
