mod config;

fn main() -> anyhow::Result<()> {
    let config = config::load()?;
    config::validate(&config)?;

    println!("{:?}", config);

    let db_path = config.osu_path.join("osu!.db");
    let osu_db_parsed = rosu_db::parse_osu_db(&db_path)?;
    println!("Database path: {:?}", osu_db_parsed);
    Ok(())
}
