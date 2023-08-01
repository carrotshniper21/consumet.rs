<h1 align="center">consumet-api-rs</h1>

<h2>MOVIES</h2>

By using `MOVIES` category you can interact with the movie providers. And have access to the movie providers methods. Which allows you to search for movies and shows, get the movie/tv series information, get the movie/tv series episodes with streaming links.

```rust
use consumet_api_rs::providers::movies;

// <provider_name> is the name of the provider you want to use. list of the providers is below.
let movie_provider = movies::<provider_name>;
```

## Common Methods

``languages`` - array, the language of the current provider, return language code, example: ``["en"]``

``is_nsfw`` - bool, ``true`` if the provider providers NSFW content.

``is_working`` - bool, a bool to identify the state of the current provider, ``true`` if the provider is working, ``false`` otherwise. 

``name`` - string, the name of the current provider, example: ``"FlixHQ"``

``base_url`` - string, url to the base URL of the current provider

``logo`` - string, url to the logo image of the current provider

``class_path`` - string,

``supported_types`` - A ``array`` of supported types by the provider, to check if a type is supported use  ``println!("{:#?}", supported_types)``.
  

## Movies Providers List
This list is in alphabetical order. (except the sub bullet points)

- [FlixHQ](../providers/flixhq.md)

<p align="end">(<a href="https://github.com/carrotshniper21/consumet-api-rs/blob/main/docs">back to table of contents</a>)</p>
