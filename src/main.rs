mod config;
mod db;
mod ui;

use config::Config;

fn main() -> anyhow::Result<()> {
    let config = Config::load()?;

    println!("{config:?}");
    // let missing_beatmaps = db::get_missing_hashes(&config.osu_path)?;

    Ok(())
}
