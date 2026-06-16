// src/db.rs
use anyhow::Result;
use rosu_db::{parse_collection_db, parse_osu_db};
use std::collections::HashSet;
use std::path::Path;

pub fn get_missing_hashes(osu_path: &Path) -> Result<HashSet<String>> {
    let osu_db = parse_osu_db(&osu_path.join("osu!.db"))?;
    let collection_db = parse_collection_db(&osu_path.join("collection.db"))?;

    // all md5s of beatmaps you have installed
    let installed: HashSet<String> = osu_db
        .beatmaps
        .iter()
        .filter_map(|b| b.md5.clone())
        .collect();

    // all md5s referenced in your collections, minus the ones already installed
    let missing: HashSet<String> = collection_db
        .collections
        .iter()
        .flat_map(|c| c.beatmap_md5s.iter())
        .filter_map(|md5| md5.clone())
        .filter(|md5| !installed.contains(md5))
        .collect();

    Ok(missing)
}
