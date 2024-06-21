use std::{
    collections::HashMap,
    env::{self, current_dir},
    path::PathBuf,
    str::FromStr,
};

use anyhow::Result;
use deno_task_shell::{execute, parser::parse};
use futures_util::Future;

async fn run_command(command: &str) -> Result<i32> {
    Ok(0)
}

pub async fn generate_short(infile: &str, outfile: &str, start: u32) -> Result<i32> {
    run_command(&format!(
        "ffmpeg -ss {} -i {} -c copy -t 30 {}",
        start, infile, outfile
    ))
    .await
}

pub async fn generate_audio(infile: &str, outfile: &str) -> Result<i32> {
    let cmd = format!("ffmpeg -i {} -vn -acodec copy {}", infile, outfile);
    Ok(run_command(&cmd).await?)
}
