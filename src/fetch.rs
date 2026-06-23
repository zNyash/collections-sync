use std::{collections::HashSet, num::NonZeroU32, sync::Arc};

use anyhow::Result;
use futures::{StreamExt, stream};
use governor::{Quota, RateLimiter};
use indicatif::{ProgressBar, ProgressStyle};
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

    let bar = ProgressBar::new(md5s_list.len() as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "{msg}\n[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .progress_chars("=>-"),
    );
    bar.set_message("Fetching beatmap IDs...");

    let mapset_ids: HashSet<u32> = stream::iter(md5s_list.iter())
        .map(|md5| {
            let limiter = Arc::clone(&limiter);
            let bar = bar.clone();

            async move {
                limiter.until_ready().await;
                let result = osu.beatmap().checksum(md5).await;
                bar.inc(1);
                result.ok().map(|b| b.mapset_id)
            }
        })
        .buffer_unordered(MAX_CONCURRENT)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .flatten()
        .collect();

    bar.finish_with_message(format!("Fetched {} beatmpa sets.", mapset_ids.len()));

    Ok(mapset_ids)
}
