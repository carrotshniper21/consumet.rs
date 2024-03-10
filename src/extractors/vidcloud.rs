use crate::{
    models::{ExtractConfig, VideoExtractor},
    utils::{decrypt, util_funcs::USER_AGENT},
    CLIENT,
};
use openssl::base64;
use serde::{Deserialize, Serialize};

/// Contains both the Decrypted Sources and Subtitles
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VidCloud {
    pub sources: Vec<VidCloudSource>,
    pub subtitles: Vec<VidCloudSubtitle>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VidCloudSubtitle {
    pub url: String,
    pub lang: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VidCloudSource {
    pub url: String,
    pub quality: String,
    pub is_m3u8: bool,
}

/// Contains the Subtitles for the Sources
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tracks {
    pub file: String,
    pub label: String,
    pub kind: String,
    pub default: Option<bool>,
}

/// Contains the Decrypted Sources File
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Video {
    pub file: Option<String>,
    pub r#type: Option<String>,
}

/// Sources Enum for when its being decrypted
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum File {
    EncryptedURL(String),
    DecryptedURL(Vec<Video>),
}

/// Contains the Encrypted Sources
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sources {
    pub sources: Option<serde_json::Value>,
    pub tracks: Option<Vec<Tracks>>,
    pub encrypted: bool,
    pub server: u8,
}

const HOST: &str = "https://dokicloud.one";
const HOST2: &str = "https://rabbitstream.net";

impl VideoExtractor for VidCloud {
    type VideoSource = VidCloud;

    // NOTE: Only needs video_url & is_alternativeparam
    async fn extract(
        &mut self,
        video_url: String,
        args: ExtractConfig,
    ) -> anyhow::Result<Self::VideoSource> {
        let ExtractConfig {
            vis_cloud_helper: _,
            api_key: _,
            is_alternative,
            user_agent: _,
        } = args;

        let is_alternative: bool = is_alternative.unwrap_or(false);

        let host = if !is_alternative { HOST } else { HOST2 };

        let parts: Vec<&str> = video_url.split('/').collect();
        let id = parts.last().unwrap().split('?').next().unwrap();

        let sources_text = CLIENT
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
                panic!("Please fix this") // Note: I will never fix this - eatmynerds
            }
        };

        let sources = match url {
            File::DecryptedURL(decrypted) => decrypted,
            File::EncryptedURL(encrypted) => {
                let decrypt_key: String = CLIENT
                    .get("https://raw.githubusercontent.com/eatmynerds/key/e4/key.txt")
                    .send()
                    .await?
                    .text()
                    .await?;

                let key_json: Vec<u8> = serde_json::from_str(&decrypt_key)?;

                let key_string = base64::encode_block(&key_json);

                let decrypted_str = decrypt::decrypt_url(&encrypted, &key_string.into_bytes())
                    .expect("Unable to decrypt URL");

                let decrypted: Vec<Video> =
                    serde_json::from_str(&decrypted_str).expect("Failed to deserialize json");

                decrypted
            }
        };

        let mut temp_sources: Vec<VidCloudSource> = vec![];

        self.sources.push(VidCloudSource {
            url: sources[0].file.clone().unwrap(),
            quality: "auto".to_string(),
            is_m3u8: sources[0].file.clone().unwrap().contains(".m3u8"),
        });

        for source in sources {
            let data = CLIENT
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

            temp_sources.extend(td_array.iter().map(|&(f1, f2)| VidCloudSource {
                url: f2.to_string(),
                quality: f1.to_string(),
                is_m3u8: f2.contains(".m3u8"),
            }));

            self.sources.extend(temp_sources.iter().cloned());
        }

        let subtitles: Vec<VidCloudSubtitle> = encrypted_sources
            .tracks
            .unwrap()
            .iter()
            .map(|s| VidCloudSubtitle {
                url: s.file.clone(),
                lang: s.label.clone(),
            })
            .collect();

        self.subtitles.extend(subtitles);

        Ok(Self {
            sources: self.sources.clone(),
            subtitles: self.subtitles.clone(),
        })
    }
}
