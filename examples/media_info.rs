use consumet_api_rs::models::MovieParser;
use consumet_api_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        "{:#?}",
        movies::FlixHQ
            .fetch_media_info("movie/watch-puss-in-boots-17289".to_owned())
            .await?
    );

    Ok(())
}
