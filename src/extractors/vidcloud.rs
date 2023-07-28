use crate::models::{ISubtitle, IVideo};
use crate::utils::{decrypt, util_funcs::USER_AGENT};
use serde::{Deserialize, Serialize};

/// Contains both the Decrypted Sources and Subtitles
#[derive(Debug, Deserialize)]
pub struct VidCloud {
    pub sources: Vec<IVideo>,
    pub subtitles: Vec<ISubtitle>,
}

/// Contains the Subtitles for the Sources
#[derive(Debug, Serialize, Deserialize)]
pub struct Tracks {
    pub file: Option<String>,
    pub label: Option<String>,
    pub kind: Option<String>,
    pub default: Option<bool>,
}

/// Contains the Decrypted Sources File
#[derive(Debug, Deserialize, Clone)]
pub struct Video {
    pub file: Option<String>,
    pub r#type: Option<String>,
}

/// Sources Enum for when its being decrypted
#[derive(Debug, Clone, Deserialize)]
pub enum File {
    EncryptedURL(String),
    DecryptedURL(Vec<Video>),
}

/// Contains the Encrypted Sources
#[derive(Debug, Deserialize)]
pub struct Sources {
    pub sources: Option<serde_json::Value>,
    pub tracks: Option<Vec<Tracks>>,
    pub encrypted: bool,
    pub server: u8,
}

const HOST: &str = "https://dokicloud.one";
const HOST2: &str = "https://rabbitstream.net";

impl VidCloud {
    const SERVER_NAME: &'static str = "VidCloud";

    pub async fn extract(
        &mut self,
        video_url: String,
        is_alternative: Option<bool>,
    ) -> anyhow::Result<VidCloud> {
        let is_alternative = is_alternative.unwrap_or(false);

        let host = if !is_alternative { HOST } else { HOST2 };

        let parts: Vec<&str> = video_url.split('/').collect();
        let id = parts.last().unwrap().split('?').next().unwrap();

        let sources_text = reqwest::Client::new()
            .get(format!("{}/ajax/embed-4/getSources?id={}", host, id))
            .header("X-Requested-With", "XMLHttpRequest")
            .header("Referer", video_url.to_string())
            .header("User-Agent", USER_AGENT)
            .send()
            .await?
            .text()
            .await?;

        let encrypted_sources: Sources =
            serde_json::from_str(&sources_text).expect("Failed to deserialize json");

        let url = match encrypted_sources.sources {
            Some(serde_json::Value::String(sources)) => File::EncryptedURL(sources),
            Some(serde_json::Value::Array(sources)) => {
                let sources = sources
                    .into_iter()
                    .map(|x| serde_json::from_value::<Video>(x).unwrap())
                    .collect::<Vec<_>>();
                File::DecryptedURL(sources)
            }
            _ => {
                panic!("Unisquirrel told me not to do this!")
            }
        };

        let sources = match url {
            File::DecryptedURL(decrypted) => decrypted,
            File::EncryptedURL(encrypted) => {
                let decrypt_key: Vec<(usize, usize)> = reqwest::Client::new()
                    .get("https://raw.githubusercontent.com/enimax-anime/key/e4/key.txt")
                    .send()
                    .await?
                    .json()
                    .await?;

                let mut encrypted_url_temp = encrypted.chars().collect::<Vec<char>>();

                let mut key = String::new();

                for (start, end) in decrypt_key {
                    for var in &mut encrypted_url_temp[start..end] {
                        key.push(*var);
                        *var = '\0';
                    }
                }

                encrypted_url_temp.retain(|x| *x != '\0');

                let encrypted_url = encrypted_url_temp.into_iter().collect::<String>();
                let decrypted_str = decrypt::decrypt_url(encrypted_url, &key.into_bytes())
                    .expect("Unable to decrypt URL");

                let decrypted: Vec<Video> =
                    serde_json::from_str(&decrypted_str).expect("Failed to deserialize json");

                decrypted
            }
        };

        let mut temp_sources: Vec<IVideo> = Vec::new();

        self.sources.push(IVideo {
            url: sources[0].file.clone(),
            quality: Some("auto".to_string()),
            is_m3u8: Some(sources[0].file.clone().unwrap().contains(".m3u8")),
            is_dash: None,
            size: None,
            other: None,
        });

        for source in sources {
            let data = reqwest::Client::new()
                .get(&source.file.unwrap())
                .send()
                .await?
                .text()
                .await?;

            let urls: Vec<String> = data
                .lines()
                .filter(|line| line.contains(".m3u8"))
                .map(|line| line.to_string())
                .collect();

            let qualities: Vec<String> = data
                .lines()
                .filter(|line| line.contains("RESOLUTION="))
                .map(|line| line.to_string())
                .collect();

            let td_array: Vec<(&str, &str)> = qualities
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let f1 = s.split('x').nth(1).unwrap_or("");
                    let f2 = urls[i].as_str();
                    (f1, f2)
                })
                .collect();

            temp_sources.extend(td_array.iter().map(|&(f1, f2)| IVideo {
                url: Some(f2.to_string()),
                quality: Some(f1.to_string()),
                is_m3u8: Some(f2.contains(".m3u8")),
                is_dash: None,
                size: None,
                other: None,
            }));

            self.sources.extend(temp_sources.iter().cloned());
        }

        let subtitles: Vec<ISubtitle> = encrypted_sources
            .tracks
            .unwrap()
            .iter()
            .map(|s| ISubtitle {
                id: None,
                url: s.file.clone(),
                lang: if s.label.is_some() {
                    s.label.clone()
                } else {
                    Some("Default (maybe)".to_string())
                },
            })
            .collect();

        self.subtitles.extend(subtitles);

        Ok(VidCloud {
            sources: self.sources.clone(),
            subtitles: self.subtitles.clone(),
        })
    }
}
