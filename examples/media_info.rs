use consumet_api_rs::models::MovieParser;
use consumet_api_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        "{:#?}",
        movies::FlixHQ.fetch_media_info("movie/watch-the-flash-97519".to_owned()).await?
    );

    Ok(())
}

