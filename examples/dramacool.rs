use consumet::{
    models::StreamingServers, providers::movies, providers::movies::dramacool::DramaCoolSourceType,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dramacool = movies::DramaCool;

    let search_results = dramacool.search("hi", None).await?;

    let media_id = &search_results.results[0].id;

    let media_info = dramacool.info(media_id).await?;

    let episode_id = &media_info.episodes[0].id;

    let servers = dramacool.servers(episode_id).await?;

    let server_name = &servers.servers[0].name;

    let server = match server_name.as_str() {
        "asianload" => StreamingServers::AsianLoad,
        _ => panic!("Server not found!"),
    };

    let sources = dramacool.sources(episode_id, Some(server)).await?;

    match sources.sources {
        DramaCoolSourceType::StreamSB(sources) => {
            println!("{:#?}", sources);
        }
        DramaCoolSourceType::AsianLoad(sources) => {
            println!("{:#?}", sources);
        }
        DramaCoolSourceType::StreamTape(sources) => {
            println!("{:#?}", sources);
        }
        DramaCoolSourceType::MixDrop(sources) => {
            println!("{:#?}", sources);
        }
    }

    Ok(())
}
