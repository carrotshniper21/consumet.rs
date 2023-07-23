use crate::models::Hashes;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Bofa {
    Poster(String),
}

#[derive(Clone, Copy, Debug)]
pub struct IProviderStats<'a> {
    pub name: &'a str,
    pub base_url: &'a str,
    pub lang: &'a [&'a str],
    pub is_nsfw: bool,
    pub logo: &'a str,
    pub class_path: &'a str,
    pub is_working: bool,
}

pub struct ITitle {
    romaji: Option<String>,
    english: Option<String>,
    native: Option<String>,
    user_preferred: Option<String>,
}

pub struct IAnimeResult {
    id: String,
    title: ITitle,
    url: Option<String>,
    image: Option<String>,
    cover: Option<String>,
    status: Option<MediaStatus>,
    rating: Option<u32>,
    show_type: Option<MediaFormat>,
    release_date: Option<String>,
}

#[derive(Debug)]
pub struct ISearch<T> {
    pub current_page: Option<usize>,
    pub has_next_page: Option<bool>,
    pub total_pages: usize,
    /**
     * total results must include results from all pages
     */
    pub total_results: usize,
    pub results: Vec<T>,
}

pub struct Trailer {
    id: String,
    site: Option<String>,
    thumbnail: Option<String>,
}

pub struct FuzzyDate {
    year: Option<u32>,
    month: Option<u32>,
    day: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaFormat {
    Tv,
    TvShort,
    Movie,
    Special,
    Ova,
    Ona,
    Music,
    Manga,
    Novel,
    OneShot,
}

impl std::fmt::Display for MediaFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bofa = match self {
            Self::Tv => "TV",
            Self::TvShort => "TV_SHORT",
            Self::Movie => "MOVIE",
            Self::Special => "SPECIAL",
            Self::Ova => "OVA",
            Self::Ona => "ONA",
            Self::Music => "MUSIC",
            Self::Manga => "MANGA",
            Self::Novel => "NOVEL",
            Self::OneShot => "ONE_SHOT",
        };

        write!(f, "{}", bofa)
    }
}

pub struct IAnimeInfo {
    mal_id: Option<u32>,
    genres: Option<Vec<String>>,
    description: Option<String>,
    status: Option<MediaStatus>,
    total_episodes: Option<u32>,
    sub_or_dub: Option<SubOrSub>,
    synonyms: Option<Vec<String>>,
    /**
     * two letter representation of country: e.g JP for japan
     */
    country_of_origin: Option<String>,
    is_adult: Option<bool>,
    is_licensed: Option<bool>,
    /**
     * `FALL`, `WINTER`, `SPRING`, `SUMMER`
     */
    season: Option<String>,
    studios: Option<Vec<String>>,
    color: Option<String>,
    cover: Option<String>,
    trailer: Option<Trailer>,
    episodes: Option<Vec<IAnimeEpisode>>,
    start_date: Option<FuzzyDate>,
    end_date: Option<FuzzyDate>,
    recommendations: Option<IAnimeResult>,
    relations: Option<Vec<IAnimeResult>>,
}

pub struct IAnimeEpisode {
    id: String,
    u32: u32,
    title: Option<String>,
    description: Option<String>,
    is_filler: Option<bool>,
    url: Option<String>,
    image: Option<String>,
    release_date: Option<String>,
}

pub struct IEpisodeServer {
    name: String,
    url: String,
}

#[derive(Clone)]
pub struct IVideo {
    /**
     * The **MAIN URL** of the video provider that should take you to the video
     */
    pub url: String,
    /**
     * The Quality of the video should include the `p` suffix
     */
    pub quality: Option<String>,
    /**
     * make sure to set this to `true` if the video is hls
     */
    pub is_m3u8: Option<bool>,
    pub is_dash: Option<bool>,
    /**
     * size of the video in **bytes**
     */
    pub size: Option<u32>,
    pub other: HashMap<String, Bofa>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamingServers {
    GogoCDN,
    StreamSB,
    MixDrop,
    UpCloud,
    VidCloud,
    StreamTape,
    VizCloud,
    MyCloud,
    Filemoon,
    VidStreaming,
}

impl std::fmt::Display for StreamingServers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bofa = match self {
            Self::GogoCDN => "gogocdn",
            Self::StreamSB => "streamsb",
            Self::MixDrop => "mixdrop",
            Self::UpCloud => "upcloud",
            Self::VidCloud => "vidcloud",
            Self::StreamTape => "streamtape",
            Self::VizCloud => "vizcloud",
            Self::MyCloud => "mycloud",
            Self::Filemoon => "filemoon",
            Self::VidStreaming => "vidstreaming",
        };

        write!(f, "{}", bofa)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaStatus {
    OnGoing,
    Completed,
    Hiatus,
    Cancelled,
    NotYetAired,
    Unknown,
}

impl std::fmt::Display for MediaStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bofa = match self {
            Self::OnGoing => "Ongoing",
            Self::Completed => "Completed",
            Self::Hiatus => "Hiatus",
            Self::Cancelled => "Cancelled",
            Self::NotYetAired => "Not yet aired",
            Self::Unknown => "Unknown",
        };

        write!(f, "{}", bofa)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubOrSub {
    Sub,
    Dub,
    Both,
}

impl std::fmt::Display for SubOrSub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bofa = match self {
            Self::Sub => "sub",
            Self::Dub => "dub",
            Self::Both => "both",
        };

        write!(f, "{}", bofa)
    }
}

pub struct IMangaResult {
    id: String,
    title: String,
    alt_titles: Option<Vec<String>>,
    image: Option<String>,
    description: Option<String>,
    status: Option<MediaStatus>,
    release_date: Option<u32>,
}

pub struct IMangaChapter {
    id: String,
    title: String,
    volume: Option<u32>,
    pages: Option<u32>,
    release_date: Option<String>,
}

pub struct IMangaInfo {
    mal_id: Option<u32>,
    authors: Option<Vec<String>>,
    genres: Option<Vec<String>>,
    links: Option<Vec<String>>,
    characters: Option<Vec<Box<dyn std::any::Any>>>,
    recommendations: Option<Vec<IMangaResult>>,
    chapters: Option<Vec<IMangaChapter>>,
}

pub struct IMangaChapterPage {
    img: String,
    page: u32,
}

pub struct ILightNovelResult {
    id: String,
    title: String,
    url: String,
    image: Option<String>,
}

pub struct ILightNovelChapter {
    id: String,
    title: String,
    volume: Option<u32>,
    url: Option<String>,
}

pub struct ILightNovelChapterContent {
    text: String,
    html: Option<String>,
}

pub struct ILightNovelInfo {
    authors: Option<Vec<String>>,
    genres: Option<Vec<String>>,
    description: Option<String>,
    chapters: Option<Vec<ILightNovelChapter>>,
    status: Option<MediaStatus>,
    views: Option<u32>,
    rating: Option<u32>,
}

pub struct LibgenBook {
    id: String,
    language: String,
    format: String,
    size: String,
    pages: String,
    table_of_contents: String,
    topic: String,
    hashes: Hashes,
}

pub struct LibgenResult {
    result: Vec<LibgenBook>,
    has_next_page: bool,
}

pub struct GetComicsComics {
    image: String,
    title: String,
    year: String,
    size: String,
    excerpt: String,
    category: String,
    description: String,
    download: String,
    ufile: String,
    mega: String,
    mediafire: String,
    zippyshare: String,
    read_online: String,
}

pub struct ComicRes {
    containers: Vec<GetComicsComics>,
    has_next_page: bool,
}

pub struct ZLibrary {
    book_rating: String,
    book_quality: String,
    language: String,
    size: String,
    pages: String,
}

#[derive(Clone)]
pub struct ISubtitle {
    /**
     * The id of the subtitle. **not** required
     */
    pub id: Option<String>,
    /**
     * The **url** that should take you to the subtitle **directly**.
     */
    pub url: Option<String>,
    /**
     * The language of the subtitle
     */
    pub lang: Option<String>,
}

/**
 * The start, and the end of the intro or opening in seconds.
 */
pub struct Intro {
    start: u32,
    end: u32,
}

pub struct ISource {
    headers: Option<String>,
    intro: Option<Intro>,
    subtitles: Option<Vec<ISubtitle>>,
    sources: Vec<IVideo>,
}

/**
 * Used **only** for movie/tvshow providers
 */
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TvType {
    TvSeries,
    Movie,
    Anime,
}

impl std::fmt::Display for TvType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bofa = match self {
            Self::TvSeries => "TV Series",
            Self::Movie => "Movie",
            Self::Anime => "Anime",
        };

        write!(f, "{}", bofa)
    }
}

#[derive(Debug, Clone)]
pub struct IMovieEpisode {
    pub id: String,
    pub title: String,
    pub url: Option<String>,
    pub number: Option<u32>,
    pub season: Option<usize>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub release_date: Option<String>,
}

#[derive(Debug)]
pub struct IMovieResult {
    pub id: Option<String>,
    pub cover: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub image: Option<String>,
    pub release_date: Option<String>,
    pub media_type: Option<TvType>,
}

pub struct INewsFeed {
    /** topics of the feed */
    topics: Vec<Topics>,
    /** preview of the news feed */
    preview: INewsFeedPreview,
}

pub struct INewsInfo {
    /** intro of the news */
    intro: String,
    /** description of the news */
    description: String,
}

pub struct INews {
    /** id of the news */
    id: String,
    /** title of the news */
    title: String,
    /** time at which the news was uploaded */
    uploaded_at: String,
    /** thumbnail image URL of the news */
    thumbnail: String,
    /** URL of the news */
    url: String,
}

pub struct INewsFeedPreview {
    /** intro of the feed */
    intro: String,
    /** some contents of the feed */
    full: String,
}

#[derive(Debug)]
pub struct IMovieSeason {
    pub season: usize,
    pub image: Option<String>,
    pub episodes: Option<Vec<Vec<IMovieEpisode>>>,
}

#[derive(Debug)]
pub struct IMovieInfo {
    pub genres: Option<Vec<String>>,
    pub description: Option<String>,
    pub rating: Option<String>,
    pub status: Option<MediaStatus>,
    pub duration: Option<String>,
    pub country: Option<Vec<String>>,
    pub production: Option<Vec<String>>,
    pub casts: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub total_episodes: Option<usize>,
    pub seasons: Option<IMovieSeason>,
    pub episodes: Option<Vec<Vec<IMovieEpisode>>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Genres {
    Action,
    Adventure,
    Cars,
    Comedy,
    Drama,
    Ecchi,
    Fantasy,
    Horror,
    MahouShoujo,
    Mecha,
    Music,
    Mystery,
    Psychological,
    Romance,
    Scifi,
    SliceOfLife,
    Sports,
    SuperNatural,
    Thriller,
}

impl std::fmt::Display for Genres {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bofa = match self {
            Self::Action => "Action",
            Self::Adventure => "Adventure",
            Self::Cars => "Cars",
            Self::Comedy => "Comedy",
            Self::Drama => "Drama",
            Self::Ecchi => "Ecchi",
            Self::Fantasy => "Fantasy",
            Self::Horror => "Horror",
            Self::MahouShoujo => "Mahou Shoujo",
            Self::Mecha => "Mecha",
            Self::Music => "Music",
            Self::Mystery => "Mystery",
            Self::Psychological => "Psychological",
            Self::Romance => "Romance",
            Self::Scifi => "Sci-Fi",
            Self::SliceOfLife => "Slice of Life",
            Self::Sports => "Sports",
            Self::SuperNatural => "Supernatural",
            Self::Thriller => "Thriller",
        };

        write!(f, "{}", bofa)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Topics {
    Anime,
    Animation,
    Manga,
    Games,
    Novels,
    LiveAction,
    Covid19,
    Industry,
    Music,
    People,
    Merch,
    Events,
}

impl std::fmt::Display for Topics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bofa = match self {
            Self::Anime => "anime",
            Self::Animation => "animation",
            Self::Manga => "manga",
            Self::Games => "games",
            Self::Novels => "novels",
            Self::LiveAction => "live-action",
            Self::Covid19 => "covid-19",
            Self::Industry => "industry",
            Self::Music => "music",
            Self::People => "people",
            Self::Merch => "merch",
            Self::Events => "events",
        };

        write!(f, "{}", bofa)
    }
}

pub struct ProxyConfig {
    /**
     * The proxy URL
     * @example https://proxy.com
     **/
    url: String,
    /**
     * X-API-Key header value (if any)
     **/
    key: Option<String>,
}
