const PAGES: &[[&str; 2]] = &[["Home", "/"], ["Twiddle", "/twiddle"]];

markup::define! {
    Layout<Head: markup::Render, Main: markup::Render>(
        head: Head,
        main: Main,
    ) {
        @markup::doctype()
        html {
            head {
                @head

                style { @markup::raw(include_str!("main.css")) }
            }

            body {
                @Sidenav {}

                main {
                    @main
                }
            }
        }
    }

    Sidenav() {
        nav {
            h1 { "Utilities" }
            @for [title, path] in PAGES {
                a.nav_link[href = *path] { @title }
            }
        }
    }
}
