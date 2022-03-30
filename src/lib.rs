use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalUrls {
    pub spotify: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    pub name: String,
    pub external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    pub name: String,
    pub artists: Vec<Artist>,
    pub external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    pub name: String,
    pub href: String,
    pub popularity: u32,
    pub album: Album,
    pub external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Items<T> {
    pub items: Vec<T>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponse {
    pub tracks: Items<Track>,
}
