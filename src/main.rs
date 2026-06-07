use indicatif::MultiProgress;

mod config;
mod db;
mod ui;

fn main() -> anyhow::Result<()> {
    let mp = MultiProgress::new();

    let config = config::load()?;
    config::validate(&config)?;

    let missing_beatmaps = db::get_missing_hashes(&config.osu_path)?;
    ui::info(&mp, &format!("{} maps missing", missing_beatmaps.len()));

    Ok(())
}
