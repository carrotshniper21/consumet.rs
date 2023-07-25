use consumet_api_rs::models::{MovieParser, StreamingServers};
use consumet_api_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        "{:#?}",
        movies::FlixHQ
            .fetch_episode_sources(
                "1274950".to_owned(),
                "tv/watch-yo-mtv-raps-82018".to_owned(),
                Some(StreamingServers::UpCloud),
            )
            .await?
    );

    Ok(())
}
