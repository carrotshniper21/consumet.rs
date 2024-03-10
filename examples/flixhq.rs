use consumet::{
    models::StreamingServers,
    providers::movies,
    providers::movies::flixhq::{FlixHQInfo, FlixHQSourceType},
};
use std::process::Command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let flixhq = movies::FlixHQ;

    let data = flixhq.search("Vincenzo", None).await?;

    let movie_id = &data.results[0].id;

    let movie_info = flixhq.info(&movie_id).await?;

    match movie_info {
        FlixHQInfo::Tv(show) => {
            let media_id = show.id;

            let episode_id = &show.seasons.episodes[0][0].id;

            let servers = flixhq.servers(&episode_id, &media_id).await?;

            let chosen_server = match servers.servers[0].name.as_str() {
                "UpCloud" => StreamingServers::UpCloud,
                "VidCloud" => StreamingServers::VidCloud,
                _ => todo!(),
            };

            let sources = flixhq
                .sources(&episode_id, &media_id, Some(chosen_server))
                .await?;

            match sources.sources {
                FlixHQSourceType::VidCloud(embed_links) => {
                    let _ = Command::new("mpv")
                        .arg(&embed_links[0].url)
                        .spawn()
                        .unwrap();
                }
                FlixHQSourceType::MixDrop(_) => {}
            }
        }
        FlixHQInfo::Movie(movie) => {
            let media_id = movie.id;

            let episode_id = media_id.rsplit("-").collect::<Vec<&str>>()[0];

            let servers = flixhq.servers(episode_id, &media_id).await?;

            let chosen_server = match servers.servers[0].name.as_str() {
                "UpCloud" => StreamingServers::UpCloud,
                "VidCloud" => StreamingServers::VidCloud,
                _ => todo!(),
            };

            let sources = flixhq
                .sources(episode_id, &media_id, Some(chosen_server))
                .await?;

            match sources.sources {
                FlixHQSourceType::VidCloud(embed_links) => {
                    let _ = Command::new("mpv")
                        .arg(&embed_links[0].url)
                        .spawn()
                        .unwrap();
                }
                FlixHQSourceType::MixDrop(_) => {}
            }
        }
    }

    Ok(())
}
