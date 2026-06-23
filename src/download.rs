use std::{collections::HashSet, num::NonZeroU32, sync::Arc};

use futures::{StreamExt, stream};
use governor::{Quota, RateLimiter};
use indicatif::{ProgressBar, ProgressStyle};
use tokio::{fs, fs::File, io::AsyncWriteExt};

const MAX_CONCURRENT: usize = 10;
const REQUESTS_PER_MINUTE: u32 = 50;

pub async fn download_beatmaps_by_ids(ids: &HashSet<u32>) {
    fs::create_dir_all("downloads").await.unwrap();

    let limiter = Arc::new(RateLimiter::direct(Quota::per_minute(
        NonZeroU32::new(REQUESTS_PER_MINUTE).unwrap(),
    )));

    let bar = ProgressBar::new(ids.len() as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "{msg}\n[{elapsed_precise}] [{bar:40.green/white}] {pos}/{len} ({eta}) [{bytes_per_sec}]",
        )
        .unwrap()
        .progress_chars("=>-"),
    );
    bar.set_message("Starting downloads...");

    let client = reqwest::Client::new();

    stream::iter(ids.iter())
        .map(|id| {
            let client = client.clone();
            let limiter = Arc::clone(&limiter);
            let bar = bar.clone();

            async move {
                limiter.until_ready().await;

                let response = client
                    .get(format!("https://osu.direct/api/d/{id}?noVideo=true"))
                    .send()
                    .await
                    .unwrap();

                let mut file = File::create(format!("downloads/{id}.osz")).await.unwrap();
                let mut stream = response.bytes_stream();

                while let Some(chunk) = stream.next().await {
                    file.write_all(&chunk.unwrap()).await.unwrap();
                }

                bar.println(format!("Downloaded {id}.osz"));
                bar.inc(1);
            }
        })
        .buffer_unordered(MAX_CONCURRENT)
        .collect::<Vec<_>>()
        .await;

    bar.finish_with_message("All downloads complete.");
}
