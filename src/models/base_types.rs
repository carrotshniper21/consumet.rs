use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Book Info struct
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

#[derive(Debug, Deserialize)]
/// Hash struct
pub struct Hashes {
    pub aich: String,
    pub crc32: String,
    pub edonkey: String,
    pub md5: String,
    pub sha1: String,
    pub sha256: Vec<String>,
    pub tth: String,
}
