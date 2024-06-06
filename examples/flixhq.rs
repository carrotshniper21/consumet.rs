use consumet::{
    models::StreamingServers,
    providers::movies,
    providers::movies::flixhq::{FlixHQInfo, FlixHQSourceType},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let flixhq = movies::FlixHQ;

    let search_results = flixhq.search("Vincenzo", None).await?;

    let media_id = &search_results.results[0].id;

    let media_info = flixhq.info(media_id).await?;

    match media_info {
        FlixHQInfo::TV(show) => {
            let media_id = &show.id;

            let episode_id = &show.seasons.episodes[0][0].id;

            let servers = flixhq.servers(episode_id, media_id).await?;

            let server_name = &servers.servers[0].name;

            let server = match server_name.as_str() {
                "UpCloud" => StreamingServers::UpCloud,
                "VidCloud" => StreamingServers::VidCloud,
                _ => panic!("Server not found!"),
            };

            let sources = flixhq.sources(episode_id, media_id, Some(server)).await?;

            match sources.sources {
                FlixHQSourceType::VidCloud(sources) => {
                    println!("{:#?}", sources);
                }
                FlixHQSourceType::MixDrop(sources) => {
                    println!("{:#?}", sources);
                }
            }
        }
        FlixHQInfo::Movie(movie) => {
            let media_id = &movie.id;

            let episode_id = &media_id.rsplit("-").collect::<Vec<&str>>()[0];

            let servers = flixhq.servers(episode_id, media_id).await?;

            let server_name = &servers.servers[0].name;

            let server = match server_name.as_str() {
                "UpCloud" => StreamingServers::UpCloud,
                "VidCloud" => StreamingServers::VidCloud,
                _ => panic!("Server not found!"),
            };

            let sources = flixhq.sources(episode_id, media_id, Some(server)).await?;

            match sources.sources {
                FlixHQSourceType::VidCloud(sources) => {
                    println!("{:#?}", sources);
                }
                FlixHQSourceType::MixDrop(sources) => {
                    println!("{:#?}", sources);
                }
            }
        }
    }

    Ok(())
}
