pub mod ffmpeg;

use std::{
    borrow::BorrowMut,
    convert::Infallible,
    env,
    sync::{Arc, Mutex},
};

use anyhow::Error;
use axum::{
    extract::{Multipart, Path},
    response::{sse::Event, Html, Redirect, Sse},
};
use futures_util::{stream, Stream};
use once_cell::sync::Lazy;
use redis::Commands;
use tokio::{
    fs,
    sync::mpsc::{self, Receiver},
};
use tokio_stream::wrappers::ReceiverStream;
use tracing::{event, instrument, Level};

use crate::db::REDIS;

use super::layout::Layout;

enum TwiddleProcessMessage {
    Init,
    Progress(u32),
    Finished,
}

pub async fn twiddle() -> Html<String> {
    Html(
        Layout {
            head: markup::new! {
                title { "Twiddle" }
            },
            main: markup::new! {
                h1 { "Twiddle" }
                form {
                    input[type="file", name="file"] {}
                }
            },
        }
        .to_string(),
    )
}

#[instrument]
pub async fn upload_twiddle(mut upload: Multipart) -> String {
    let uploads_dir = env::var("UPLOADS_DIR").unwrap();
    let file = upload.next_field().await.unwrap().unwrap();

    let filename = file.name().unwrap().to_string();

    event!(Level::INFO, "Uploading: {}", filename);

    fs::write(
        std::path::Path::new(&uploads_dir).join(&filename),
        file.bytes().await.unwrap(),
    )
    .await;

    event!(Level::INFO, "Upload finished: {}", &filename);

    filename
}

pub async fn process_twiddle(
    Path(filename): Path<String>,
) -> Sse<ReceiverStream<Result<Event, Infallible>>> {
    let (tx, rx) = mpsc::channel::<Result<Event, Infallible>>(10);

    tokio::spawn(async move {
        tx.send(Ok(
            Event::default().data(r#"{"stage": "audio", "status": "starting"}"#)
        ));

        if !matches!(
            ffmpeg::generate_audio(
                &format!("{}/{}", env::var("UPLOADS_DIR").unwrap(), filename),
                &format!("{}/{}.m4a", env::var("UPLOADS_DIR").unwrap(), filename),
            )
            .await,
            Ok(0)
        ) {
            tx.send(Ok(
                Event::default().data(r#"{"stage": "audio", "status": "error"}"#)
            ));
            return;
        }

        tx.send(Ok(
            Event::default().data(r#"{"stage": "audio", "status": "finished"}"#)
        ));
        tx.send(Ok(
            Event::default().data(r#"{"stage": "clip", "status": "starting"}"#)
        ));

        if !matches!(
            ffmpeg::generate_short(
                &format!("{}/{}", env::var("UPLOADS_DIR").unwrap(), filename),
                &format!("{}/{}.clip.mp4", env::var("UPLOADS_DIR").unwrap(), filename),
                0,
            )
            .await,
            Ok(0)
        ) {
            tx.send(Ok(
                Event::default().data(r#"{"stage": "clip", "status": "error"}"#)
            ));
            return;
        }

        tx.send(Ok(
            Event::default().data(r#"{"stage": "clip", "status": "finished"}"#)
        ));

        unsafe {
            let _: () = REDIS.lpush("twiddle:clips", "").unwrap();
        }

        tx.send(Ok(Event::default().data(r#"{"status": "finished"}"#)));
    });

    Sse::new(ReceiverStream::new(rx))
}
