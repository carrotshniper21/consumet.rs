use consumet_api_rs::models::{BaseParser, MovieParser};
use consumet_api_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dramacool = movies::DramaCool;

    println!("{:#?}", dramacool.fetch_media_info("drama-detail/richie".to_owned()).await?);

    Ok(())
}
