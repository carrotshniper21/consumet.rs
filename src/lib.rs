pub mod extractors;
pub mod html;
pub mod models;
pub mod providers;
pub mod utils;

use lazy_static::lazy_static;
use reqwest::Client;

lazy_static! {
    static ref CLIENT: Client = Client::new();
}
