use std::{collections::HashSet, num::NonZeroU32, sync::Arc};

use anyhow::Result;
use futures::{StreamExt, stream};
use governor::{Quota, RateLimiter};
use rosu_v2::Osu;

const MAX_CONCURRENT: usize = 15;
const REQUESTS_PER_MINUTE: u32 = 1080;

pub async fn fetch_beatmaps_by_md5s(
    osu: &Osu,
    md5s_list: &HashSet<String>,
) -> Result<HashSet<u32>> {
    let limiter = Arc::new(RateLimiter::direct(Quota::per_minute(
        NonZeroU32::new(REQUESTS_PER_MINUTE).unwrap(),
    )));

    let results: Vec<_> = stream::iter(md5s_list.iter())
        .map(|md5| {
            // let osu = osu.clone();
            let limiter = Arc::clone(&limiter);
            async move {
                limiter.until_ready().await;
                osu.beatmap().checksum(md5).await
            }
        })
        .buffer_unordered(MAX_CONCURRENT)
        .collect()
        .await;

    let mut errors = Vec::new();

    let beatmaps: Vec<_> = results
        .into_iter()
        .filter_map(|r| match r {
            Ok(beatmap) => {
                println!("Got beamapset: {}", beatmap.mapset_id);
                Some(beatmap)
            }
            Err(e) => {
                errors.push(e);
                None
            }
        })
        .collect();

    let mapset_ids: HashSet<u32> = beatmaps.iter().map(|beatmap| beatmap.mapset_id).collect();

    Ok(mapset_ids)
}
