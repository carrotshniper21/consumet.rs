use consumet_api_rs::models::movie_parser::MovieParser;
use consumet_api_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        "{:#?}",
        movies::FlixHQ.search("boob".to_owned(), Some(1)).await?
    );

    Ok(())
}
