use consumet_api_rs::models::MovieParser;
use consumet_api_rs::providers::movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!(
        "{:#?}",
        movies::FlixHQ
            .fetch_episode_servers(
                "98488".to_owned(),
                "movie/watch-the-venture-bros-radiant-is-the-blood-of-the-baboon-heart-98488"
                    .to_owned()
            )
            .await?
    );

    Ok(())
}
