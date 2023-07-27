#[derive(Debug)]
/// Book Info struct
pub struct Book {
    title: String,
    authors: Vec<String>,
    publisher: String,
    year: String,
    edition: String,
    volume: String,
    series: String,
    isbn: Vec<String>,
    image: String,
    description: String,
    link: String,
}

#[derive(Debug)]
/// Hash struct
pub struct Hashes {
    aich: String,
    crc32: String,
    edonkey: String,
    md5: String,
    sha1: String,
    sha256: Vec<String>,
    tth: String,
}
