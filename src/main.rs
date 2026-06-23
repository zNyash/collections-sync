mod config;
mod db;
mod download;
mod fetch;
mod ui;

use config::Config;
use rosu_v2::Osu;

use crate::{download::download_beatmaps_by_ids, fetch::fetch_beatmaps_by_md5s};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let config = Config::load()?;
    let osu = Osu::new(config.client_id, config.client_secret).await?;

    let missing_beatmaps = db::get_missing_hashes(&config.osu_path)?;
    println!(
        "Theres {:?} missing beatmaps from your colletions.",
        missing_beatmaps.len()
    );

    let mapset_ids = fetch_beatmaps_by_md5s(&osu, &missing_beatmaps).await?;

    download_beatmaps_by_ids(&mapset_ids).await;

    Ok(())
}
