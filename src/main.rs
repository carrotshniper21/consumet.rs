use pipebomb_extensions_rs::models::movie_parser::MovieParser;
use pipebomb_extensions_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        "{:#?}",
        movies::FlixHQ.search("hi".to_owned(), Some(1)).await?
    );

    Ok(())
}
