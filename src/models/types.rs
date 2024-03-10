use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct ExtractConfig {
    pub vis_cloud_helper: Option<String>,
    pub api_key: Option<String>,
    pub is_alternative: Option<bool>,
    pub user_agent: Option<String>,
}

/// Used to get other fields in structs
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Other {
    Poster(String),
}

/// Book Info struct
#[derive(Debug, Deserialize, Serialize)]
pub struct Book {
    pub title: String,
    pub authors: Vec<String>,
    pub publisher: String,
    pub year: String,
    pub edition: String,
    pub volume: String,
    pub series: String,
    pub isbn: Vec<String>,
    pub image: String,
    pub description: String,
    pub link: String,
}

/// Hash struct
#[derive(Debug, Deserialize, Serialize)]
pub struct Hashes {
    pub aich: String,
    pub crc32: String,
    pub edonkey: String,
    pub md5: String,
    pub sha1: String,
    pub sha256: Vec<String>,
    pub tth: String,
}

/// Contains Title Info
#[derive(Debug, Deserialize, Serialize)]
pub struct ITitle {
    pub romaji: Option<String>,
    pub english: Option<String>,
    pub native: Option<String>,
    pub user_preferred: Option<String>,
}

/// Contains Anime Search Results
#[derive(Debug, Deserialize, Serialize)]
pub struct IAnimeResult {
    pub id: String,
    pub title: ITitle,
    pub url: Option<String>,
    pub image: Option<String>,
    pub cover: Option<String>,
    pub status: Option<MediaStatus>,
    pub rating: Option<u32>,
    pub show_type: Option<MediaFormat>,
    pub release_date: Option<String>,
}

/// Contains Search Results
#[derive(Debug, Deserialize, Serialize)]
pub struct ISearch<T> {
    pub current_page: Option<usize>,
    pub has_next_page: bool,
    pub total_pages: Option<usize>,
    pub total_results: usize,
    pub results: Vec<T>,
}

/// Contains Trailer
#[derive(Debug, Deserialize, Serialize)]
pub struct Trailer {
    pub id: String,
    pub site: Option<String>,
    pub thumbnail: Option<String>,
}

/// Contains Date Time
#[derive(Debug, Deserialize, Serialize)]
pub struct FuzzyDate {
    pub year: Option<u32>,
    pub month: Option<u32>,
    pub day: Option<u32>,
}

/// Used to get the Format of the chosen media
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
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

/// Contains Anime Info
#[derive(Debug, Deserialize, Serialize)]
pub struct IAnimeInfo {
    pub mal_id: Option<u32>,
    pub genres: Option<Vec<String>>,
    pub description: Option<String>,
    pub status: Option<MediaStatus>,
    pub total_episodes: Option<u32>,
    pub sub_or_dub: Option<SubOrSub>,
    pub synonyms: Option<Vec<String>>,
    pub country_of_origin: Option<String>,
    pub is_adult: Option<bool>,
    pub is_licensed: Option<bool>,
    pub season: Option<String>,
    pub studios: Option<Vec<String>>,
    pub color: Option<String>,
    pub cover: Option<String>,
    pub trailer: Option<Trailer>,
    pub episodes: Option<Vec<IAnimeEpisode>>,
    pub start_date: Option<FuzzyDate>,
    pub end_date: Option<FuzzyDate>,
    pub recommendations: Option<IAnimeResult>,
    pub relations: Option<Vec<IAnimeResult>>,
}

/// Contains Anime Episode Info
#[derive(Debug, Deserialize, Serialize)]
pub struct IAnimeEpisode {
    pub id: String,
    pub u32: u32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_filler: Option<bool>,
    pub url: Option<String>,
    pub image: Option<String>,
    pub release_date: Option<String>,
}

/// Contains Episode Server Info
#[derive(Debug, Deserialize, Serialize)]
pub struct IEpisodeServer {
    pub name: Option<String>,
    pub url: Option<String>,
}

/// Contains Video Sources
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IVideo {
    pub url: Option<String>,
    pub quality: Option<String>,
    pub is_m3u8: Option<bool>,
    pub is_dash: Option<bool>,
    pub size: Option<u32>,
    pub other: Option<HashMap<String, Other>>,
}

/// Used to get the video url for the server chosen
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
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
        let server_names = match self {
            Self::GogoCDN => "GogoCDN",
            Self::StreamSB => "StreamSB",
            Self::MixDrop => "MixDrop",
            Self::UpCloud => "UpCloud",
            Self::VidCloud => "VidCloud",
            Self::StreamTape => "StreamTape",
            Self::VizCloud => "VizCloud",
            Self::MyCloud => "MyCloud",
            Self::Filemoon => "FileMoon",
            Self::VidStreaming => "VidStreaming",
        };

        write!(f, "{}", server_names)
    }
}

/// Used to check the status of the provided media
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
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

/// Used to check if something is Sub, Dub or Both
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
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

/// Contains Manga Search Result Info
#[derive(Debug, Deserialize, Serialize)]
pub struct IMangaResult {
    pub id: String,
    pub title: String,
    pub alt_titles: Option<Vec<String>>,
    pub image: Option<String>,
    pub description: Option<String>,
    pub status: Option<MediaStatus>,
    pub release_date: Option<u32>,
}

/// Contains Manga Chapter Info
#[derive(Debug, Deserialize, Serialize)]
pub struct IMangaChapter {
    pub id: String,
    pub title: String,
    pub volume: Option<u32>,
    pub pages: Option<u32>,
    pub release_date: Option<String>,
}

/// Contains Manga Info
#[derive(Debug, Deserialize, Serialize)]
pub struct IMangaInfo {
    pub mal_id: Option<u32>,
    pub authors: Option<Vec<String>>,
    pub genres: Option<Vec<String>>,
    pub links: Option<Vec<String>>,
    pub characters: Option<Vec<String>>,
    pub recommendations: Option<Vec<IMangaResult>>,
    pub chapters: Option<Vec<IMangaChapter>>,
}

/// Contains Manga Chapter Pages
#[derive(Debug, Deserialize, Serialize)]
pub struct IMangaChapterPage {
    pub img: String,
    pub page: u32,
}

/// Contains Light Novel Search Results
#[derive(Debug, Deserialize, Serialize)]
pub struct ILightNovelResult {
    pub id: String,
    pub title: String,
    pub url: String,
    pub image: Option<String>,
}

/// Contains Light Novel Chapter Info
#[derive(Debug, Deserialize, Serialize)]
pub struct ILightNovelChapter {
    pub id: String,
    pub title: String,
    pub volume: Option<u32>,
    pub url: Option<String>,
}

/// Contains Light Novel Chapter Content
#[derive(Debug, Deserialize, Serialize)]
pub struct ILightNovelChapterContent {
    pub text: String,
    pub html: Option<String>,
}

/// Contains Light Novel Info
#[derive(Debug, Deserialize, Serialize)]
pub struct ILightNovelInfo {
    pub authors: Option<Vec<String>>,
    pub genres: Option<Vec<String>>,
    pub description: Option<String>,
    pub chapters: Option<Vec<ILightNovelChapter>>,
    pub status: Option<MediaStatus>,
    pub views: Option<u32>,
    pub rating: Option<u32>,
}

/// Contains Book Search Results Info
#[derive(Debug, Deserialize, Serialize)]
pub struct LibgenBook {
    pub title: String,
    pub authors: Vec<String>,
    pub publisher: String,
    pub year: String,
    pub edition: String,
    pub volume: String,
    pub series: String,
    pub isbn: Vec<String>,
    pub link: String,
    pub id: String,
    pub language: String,
    pub format: String,
    pub size: String,
    pub pages: String,
    pub image: String,
    pub description: String,
    pub table_of_contents: String,
    pub topic: String,
    pub hashes: Hashes,
}

/// Contains Book Search Results
#[derive(Debug, Deserialize, Serialize)]
pub struct LibgenResult {
    pub result: Vec<LibgenBook>,
    pub has_next_page: bool,
}

/// Contains Comics Info
#[derive(Debug, Deserialize, Serialize)]
pub struct GetComicsComics {
    pub image: String,
    pub title: String,
    pub year: String,
    pub size: String,
    pub excerpt: String,
    pub category: String,
    pub description: String,
    pub download: String,
    pub ufile: String,
    pub mega: String,
    pub mediafire: String,
    pub zippyshare: String,
    pub read_online: String,
}

/// Contains Comic Results
#[derive(Debug, Deserialize, Serialize)]
pub struct ComicRes {
    pub containers: Vec<GetComicsComics>,
    pub has_next_page: bool,
}

/// Contains Book Info
#[derive(Debug, Deserialize, Serialize)]
pub struct ZLibrary {
    pub book_rating: String,
    pub book_quality: String,
    pub language: String,
    pub size: String,
    pub pages: String,
}

/// Contains Subtitle Info
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ISubtitle {
    pub id: Option<String>,
    pub url: Option<String>,
    pub lang: Option<String>,
}

/// The start, and the end of the intro or opening in seconds.
#[derive(Debug, Deserialize, Serialize)]
pub struct Intro {
    pub start: u32,
    pub end: u32,
}

/// Contains Source Info
#[derive(Debug, Deserialize, Serialize)]
pub struct ISource {
    pub headers: Option<String>,
    pub intro: Option<Intro>,
    pub subtitles: Option<Vec<ISubtitle>>,
    pub sources: Option<Vec<IVideo>>,
}

/// Used **only** for movie/tvshow providers
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
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

/// Contains Movie Episode Info
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IMovieEpisode {
    pub id: String,
    pub title: Option<String>,
    pub url: String,
    pub number: Option<u32>,
    pub season: Option<usize>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub release_date: Option<String>,
}

/// Contains Movie Result Info
#[derive(Debug, Deserialize, Serialize)]
pub struct IMovieResult {
    pub id: Option<String>,
    pub cover: Option<String>,
    pub title: String,
    pub other_names: Option<Vec<String>>,
    pub url: Option<String>,
    pub image: Option<String>,
    pub release_date: String,
    pub media_type: Option<TvType>,
}

/// Contains News Feed Info
#[derive(Debug, Deserialize, Serialize)]
pub struct INewsFeed {
    /** topics of the feed */
    pub topics: Vec<Topics>,
    /** preview of the news feed */
    pub preview: INewsFeedPreview,
}

/// Contains News Info
#[derive(Debug, Deserialize, Serialize)]
pub struct INewsInfo {
    /** intro of the news */
    pub intro: String,
    /** description of the news */
    pub description: String,
}

/// Contains More News Info
#[derive(Debug, Deserialize, Serialize)]
pub struct INews {
    /** id of the news */
    pub id: String,
    /** title of the news */
    pub title: String,
    /** time at which the news was uploaded */
    pub uploaded_at: String,
    /** thumbnail image URL of the news */
    pub thumbnail: String,
    /** URL of the news */
    pub url: String,
}

/// Contains News Feed Preview info
#[derive(Debug, Deserialize, Serialize)]
pub struct INewsFeedPreview {
    /** intro of the feed */
    pub intro: String,
    /** some contents of the feed */
    pub full: String,
}

/// Contains Movie Seasons
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IMovieSeason {
    pub season: Option<usize>,
    pub image: Option<String>,
    pub episodes: Option<Vec<Vec<IMovieEpisode>>>,
}

/// Contains Movie Info
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IMovieInfo {
    pub genres: Option<Vec<String>>,
    pub description: Option<String>,
    pub rating: Option<String>,
    pub status: Option<MediaStatus>,
    pub quality: Option<String>,
    pub duration: Option<String>,
    pub country: Option<Vec<String>>,
    pub production: Option<Vec<String>>,
    pub casts: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub total_episodes: Option<usize>,
    pub seasons: Option<IMovieSeason>,
    pub episodes: Option<Vec<Vec<IMovieEpisode>>>,
}

///  Contains all the possible Genres
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

///  Contains all the possible Topics
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
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
            Self::Anime => "Anime",
            Self::Animation => "Animation",
            Self::Manga => "Manga",
            Self::Games => "Games",
            Self::Novels => "Novels",
            Self::LiveAction => "LiveAction",
            Self::Covid19 => "Covid19",
            Self::Industry => "Industry",
            Self::Music => "Music",
            Self::People => "People",
            Self::Merch => "Merch",
            Self::Events => "Events",
        };

        write!(f, "{}", bofa)
    }
}

/// Optional Proxy Configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct ProxyConfig {
    /// The proxy URL
    /// <https://proxy.com>
    pub url: String,

    /// X-API-Key header value (if any)
    pub key: Option<String>,

    pub rotate_interval: usize,
}
