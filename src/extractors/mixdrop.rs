use crate::models::{Bofa, IVideo};
use regex::Regex;
use reqwest::Url;

use std::collections::HashMap;

pub struct MixDrop {
    pub sources: Vec<IVideo>,
}

impl MixDrop {
    const SERVER_NAME: &'static str = "MixDrop";

    async fn extract(&mut self, video_url: Url) -> anyhow::Result<MixDrop> {
        let data = reqwest::get(video_url).await?.text().await?;

        let re = Regex::new(r"(eval)(\(f.*?)(\n<\/script>)").unwrap();

        if let Some(captures) = re.captures(&data) {
            let formated = captures.get(2).unwrap().as_str().replace("eval", "");
            let re = Regex::new(r#"poster="([^"]+)"|wurl="([^"]+)""#).unwrap();

            let matches: Vec<String> = re
                .find_iter(&formated)
                .map(|m| m.as_str().split('=').nth(1).unwrap().replace('\"', ""))
                .collect();

            let jazz = matches
                .iter()
                .map(|x| {
                    if x.starts_with("http") {
                        x.to_string()
                    } else {
                        format!("https:{}", x)
                    }
                })
                .collect::<Vec<String>>();

            let (poster, source) = jazz.split_at(2);

            self.sources.push(IVideo {
                url: source[0].clone(),
                quality: None,
                is_m3u8: Some(source.contains(&".m3u8".to_owned())),
                is_dash: None,
                size: None,
                other: {
                    let mut other = HashMap::new();
                    other.insert("poster".to_owned(), Bofa::Poster(poster[0].clone()));
                    other
                },
            });

            Ok(MixDrop {
                sources: self.sources.clone(),
            })
        } else {
            self.sources.push(IVideo {
                url: String::new(),
                quality: None,
                is_m3u8: None,
                is_dash: None,
                size: None,
                other: { HashMap::new() },
            });

            Ok(MixDrop {
                sources: self.sources.clone(),
            })
        }
    }
}
