use consumet_api_rs::models::MovieParser;
use consumet_api_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let flixhq = movies::FlixHQ;

    let data = flixhq
        .fetch_episode_sources(
            "1167571".to_owned(),
            "tv/watch-vincenzo-67955".to_owned(),
            None,
        )
        .await?;
    println!("{:#?}", data);

    Ok(())
}
