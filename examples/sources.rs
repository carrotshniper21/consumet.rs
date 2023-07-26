use consumet_api_rs::models::{MovieParser, StreamingServers};
use consumet_api_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        "{:#?}",
        movies::FlixHQ
            .fetch_episode_sources(
                "121".to_owned(),
                "movie/watch-the-nameless-121".to_owned(),
                Some(StreamingServers::MixDrop),
            )
            .await?
    );

    Ok(())
}
