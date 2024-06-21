use axum::response::Html;

use super::layout::Layout;

pub async fn index() -> Html<String> {
    Html(
        Layout {
            head: markup::new! {
                title { "Home" }
            },
            main: markup::new! {
                h1 { "Home" }
                p { "Welcome to Aleks's utilities board." }
                p { "This is just a small website to host things I want, so I give no guarantees that it will be at all useful to anyone else." }
                p { "With that said, feel free to poke around!" }
            },
        }
        .to_string(),
    )
}
