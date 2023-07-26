/// Libgen Book Object
#[derive(Debug)]
pub struct LibgenBookObject {
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
    pub hashes: HashesObject,
}

/// Hash Object
#[derive(Debug)]
pub struct HashesObject {
    pub aich: String,
    pub crc32: String,
    pub edonkey: String,
    pub md5: String,
    pub sha1: String,
    pub sha256: Vec<String>,
    pub tth: String,
}

/// Comic Object
#[derive(Debug)]
pub struct GetComicsComicsObject {
    pub image: String,
    pub title: String,
    pub year: String,
    pub size: String,
    pub excerpt: String,
    pub description: String,
    pub download: String,
    pub category: String,
    pub ufile: String,
    pub mega: String,
    pub media_fire: String,
    pub zippy_share: String,
    pub read_online: String,
}
