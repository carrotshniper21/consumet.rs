use serde::Serialize;

/// Contains the type of what Anilist could be
#[derive(Serialize)]
pub enum AnilistType {
    #[serde(rename = "ANIME")]
    Anime,
    #[serde(rename = "MANGA")]
    Manga,
}

impl std::fmt::Display for AnilistType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = match self {
            Self::Anime => "ANIME",
            Self::Manga => "MANGA",
        };
        write!(f, "{}", list)
    }
}

/// Advanced Query
/// # Parameters
/// * `None`
pub fn anilist_advanced_query() -> &'static str {
    "query ($page: Int, $id: Int, $type: MediaType, $isAdult: Boolean = false, $search: String, $format: [MediaFormat], $status: MediaStatus, $size: Int, $countryOfOrigin: CountryCode, $source: MediaSource, $season: MediaSeason, $seasonYear: Int, $year: String, $onList: Boolean, $yearLesser: FuzzyDateInt, $yearGreater: FuzzyDateInt, $episodeLesser: Int, $episodeGreater: Int, $durationLesser: Int, $durationGreater: Int, $chapterLesser: Int, $chapterGreater: Int, $volumeLesser: Int, $volumeGreater: Int, $licensedBy: [String], $isLicensed: Boolean, $genres: [String], $excludedGenres: [String], $tags: [String], $excludedTags: [String], $minimumTagRank: Int, $sort: [MediaSort] = [POPULARITY_DESC, SCORE_DESC]) {{ Page(page: $page, perPage: $size) {{ pageInfo {{ total perPage currentPage lastPage hasNextPage }} media(id: $id, type: $type, season: $season, format_in: $format, status: $status, countryOfOrigin: $countryOfOrigin, source: $source, search: $search, onList: $onList, seasonYear: $seasonYear, startDate_like: $year, startDate_lesser: $yearLesser, startDate_greater: $yearGreater, episodes_lesser: $episodeLesser, episodes_greater: $episodeGreater, duration_lesser: $durationLesser, duration_greater: $durationGreater, chapters_lesser: $chapterLesser, chapters_greater: $chapterGreater, volumes_lesser: $volumeLesser, volumes_greater: $volumeGreater, licensedBy_in: $licensedBy, isLicensed: $isLicensed, genre_in: $genres, genre_not_in: $excludedGenres, tag_in: $tags, tag_not_in: $excludedTags, minimumTagRank: $minimumTagRank, sort: $sort, isAdult: $isAdult) {{  id idMal status(version: 2) title {{ userPreferred romaji english native } bannerImage coverImage{{ extraLarge large medium color }} episodes season popularity description format seasonYear genres averageScore countryOfOrigin nextAiringEpisode {{ airingAt timeUntilAiring episode }}  }} }} }}"
}

/// Search Query
/// # Parameters
/// * `query` - query to search for.
/// * `page` - page number (default: 1)
/// * `per_page` - number of results per page (default: 25)
/// * `query_type` - Either AnilistType::Anime or AnilistType::Manga
pub fn anilist_search_query(
    query: String,
    page: Option<u32>,
    per_page: Option<u32>,
    query_type: AnilistType,
) -> String {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(20);

    format!("query ($page: Int = {page}, $id: Int, $type: MediaType = {query_type}, $search: String = '{query}', $isAdult: Boolean = false, $size: Int = {per_page}) {{ Page(page: $page, perPage: $size) {{ pageInfo {{ total perPage currentPage lastPage hasNextPage }} media(id: $id, type: $type, search: $search, isAdult: $isAdult) {{ id idMal status(version: 2) title {{ userPreferred romaji english native }} bannerImage popularity coverImage{{ extraLarge large medium color }} episodes format season description seasonYear chapters volumes averageScore genres nextAiringEpisode {{ airingAt timeUntilAiring episode }}  }} }} }}")
}

/// Media Detail Query
/// # Parameters
/// * `id` - id to fetch info for.
pub fn anilist_media_detail_query(id: String) -> String {
    format!("query ($id: Int = {id}) {{ Media(id: $id) {{ id idMal title {{ english native romaji }} synonyms countryOfOrigin isLicensed isAdult externalLinks {{ url site type language }} coverImage {{ extraLarge large color }} startDate {{ year month day }} endDate {{ year month day }} bannerImage season seasonYear description type format status(version: 2) episodes duration chapters volumes trailer {{ id site thumbnail }} genres source averageScore popularity meanScore nextAiringEpisode {{ airingAt timeUntilAiring episode }} characters(sort: ROLE) {{ edges {{ role node {{ id name {{ first middle last full native userPreferred }} image {{ large medium }} }} voiceActors(sort: LANGUAGE) {{ id languageV2 name {{ first middle last full native userPreferred }} image {{ large medium }} }} }} }} recommendations {{ edges {{ node {{ id mediaRecommendation {{ id idMal title {{ romaji english native userPreferred }} status episodes coverImage {{ extraLarge large medium color }} bannerImage format chapters meanScore nextAiringEpisode {{ episode timeUntilAiring airingAt }} }} }} }} }} relations {{ edges {{ id relationType node {{ id idMal status coverImage {{ extraLarge large medium color }} bannerImage title {{ romaji english native userPreferred }} episodes chapters format nextAiringEpisode {{ airingAt timeUntilAiring episode }} meanScore }} }} }} studios(isMain: true) {{ edges {{ isMain node {{ id name }} }} }} }} }}")
}

/// Anilist Trending Query
/// * `page` - page number (default: 1)
/// * `per_page` - number of results per page (default: 25)
/// * `query_type` - Either AnilistType::Anime or AnilistType::Manga
pub fn anilist_trending_query(
    page: Option<u32>,
    per_page: Option<u32>,
    query_type: AnilistType,
) -> String {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(20);

    format!("query ($page: Int = {page}, $id: Int, $type: MediaType = {query_type}, $isAdult: Boolean = false, $size: Int = {per_page}, $sort: [MediaSort] = [TRENDING_DESC, POPULARITY_DESC]) {{ Page(page: $page, perPage: $size) {{ pageInfo {{ total perPage currentPage lastPage hasNextPage }} media(id: $id, type: $type, isAdult: $isAdult, sort: $sort) {{ id idMal status(version: 2) title {{ userPreferred romaji english native }} genres trailer {{ id site thumbnail }} description format bannerImage coverImage{{ extraLarge large medium color }} episodes meanScore duration season seasonYear averageScore nextAiringEpisode {{ airingAt timeUntilAiring episode }}  }} }} }}")
}

/// Anilist Popular Query
/// * `page` - page number (default: 1)
/// * `per_page` - number of results per page (default: 25)
/// * `query_type` - Either AnilistType::Anime or AnilistType::Manga
pub fn anilist_popular_query(
    page: Option<u32>,
    per_page: Option<u32>,
    query_type: AnilistType,
) -> String {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(20);

    format!("query ($page: Int = {page}, $id: Int, $type: MediaType = {query_type}, $isAdult: Boolean = false, $size: Int = ${per_page}, $sort: [MediaSort] = [POPULARITY_DESC]) {{ Page(page: $page, perPage: $size) {{ pageInfo {{ total perPage currentPage lastPage hasNextPage }} media(id: $id, type: $type, isAdult: $isAdult, sort: $sort) {{ id idMal status(version: 2) title {{ userPreferred romaji english native }} trailer {{ id site thumbnail }} format genres bannerImage description coverImage {{ extraLarge large medium color }} episodes meanScore duration season seasonYear averageScore nextAiringEpisode {{ airingAt timeUntilAiring episode }}  }} }} }}")
}

/// Anlist Genre Query
/// * `page` - page number (default: 1)
/// * `per_page` - number of results per page (default: 25)
/// * `query_type` - Either AnilistType::Anime or AnilistType::Manga

pub fn anilist_genres_query(
    genres: Vec<String>,
    page: Option<u32>,
    per_page: Option<u32>,
) -> String {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(20);
    let genres = serde_json::to_string(&genres).unwrap();

    format!("query ($genres: [String] = {genres}, $page: Int = ${page}, $type: MediaType = ANIME, $isAdult: Boolean = false, $size: Int = ${per_page}) {{Page(page: $page, perPage: $size) {{ pageInfo {{ total perPage currentPage lastPage hasNextPage }} media(type: $type, isAdult: $isAdult, genre_in: $genres) {{ id idMal status(version: 2) title {{ userPreferred romaji english native }} trailer {{ id site thumbnail }} format bannerImage description coverImage {{ extraLarge large medium color }} episodes meanScore duration season seasonYear averageScore nextAiringEpisode {{ airingAt timeUntilAiring episode }}  }} }} }}")
}

/// Anilist Airing Schedule Query
/// * `page` - page number (default: 1)
/// * `per_page` - number of results per page (default: 25)
/// * `week_start` - where the week started 1-6
/// * `week_end` - where the week ends 1-6
/// * `not_yet_aired` - check if its available yet
pub fn anilist_airing_schedule_query(
    page: Option<u32>,
    per_page: Option<u32>,
    week_start: u32,
    week_end: u32,
    not_yet_aired: bool,
) -> String {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(20);

    format!("query {{ Page(page: ${page}, perPage: ${per_page}) {{ pageInfo {{ total perPage currentPage lastPage hasNextPage }} airingSchedules( notYetAired: ${not_yet_aired}, airingAt_greater: ${week_start}, airingAt_lesser: ${week_end}) {{ airingAt episode media {{ id description idMal title {{ romaji english userPreferred native }} countryOfOrigin description popularity bannerImage coverImage {{ extraLarge large medium color }} genres averageScore seasonYear format }} }} }} }}")
}

/// Anilist Site Statistics Query
/// # Parameters
/// * `None`
pub fn anilist_site_statistics_query() -> &'static str {
    "query { SiteStatistics { anime { nodes { count } } } }"
}

/// Anilist Chracter Query
/// # Parameters
/// * `None`
pub fn anilist_character_query() -> &'static str {
    "query character($id: Int) { Character(id: $id) { id name { first middle last full native userPreferred alternative alternativeSpoiler } image { large medium } description gender dateOfBirth { year month day } bloodType age favourites media { edges { characterRole node { id idMal title { romaji english native userPreferred } coverImage { extraLarge large medium color } averageScore startDate { year month day } episodes format status } } } } }"
}

/// Anilist Staff Query
/// # Parameters
/// * `None`
pub fn anilist_staff_query() -> &'static str {
    "query staff($id: Int, $sort: [MediaSort], $characterPage: Int, $staffPage: Int, $onList: Boolean, $type: MediaType, $withCharacterRoles: Boolean = false, $withStaffRoles: Boolean = false) { Staff(id: $id) { id name { first middle last full native userPreferred alternative } image { large } description favourites isFavourite isFavouriteBlocked age gender yearsActive homeTown bloodType primaryOccupations dateOfBirth { year month day } dateOfDeath { year month day } language: languageV2 characterMedia(page: $characterPage, sort: $sort, onList: $onList) @include(if: $withCharacterRoles) { pageInfo { total perPage currentPage lastPage hasNextPage } edges { characterRole characterName node { id type bannerImage isAdult title { userPreferred } coverImage { large } startDate { year } mediaListEntry { id status } } characters { id name { userPreferred } image { large } } } } staffMedia(page: $staffPage, type: $type, sort: $sort, onList: $onList) @include(if: $withStaffRoles) { pageInfo { total perPage currentPage lastPage hasNextPage } edges { staffRole node { id type isAdult title { userPreferred } coverImage { large } mediaListEntry { id status } } } } } }"
}

/// Kitsu Search query
/// # Parameters
/// * `query` - query to search for.
pub fn kitsu_search_query(query: String) -> String {
    format!("query{{ searchAnimeByTitle(first:5, title:'{query}'){{ nodes {{id season startDate titles {{ localized }} episodes(first: 2000){{ nodes {{ number createdAt titles {{ canonical }} description thumbnail {{ original {{ url }} }} }} }} }} }} }}")
}
