use consumet_api_rs::models::BaseParser;
use consumet_api_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let flixhq = movies::FlixHQ;

    println!("{:#?}", flixhq.search("hi".to_owned(), None).await?);

    Ok(())
}
