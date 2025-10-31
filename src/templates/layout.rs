use super::components::{ascii_header, footer};
use crate::styles::terminal_styles;
use maud::{DOCTYPE, Markup, html};

pub fn page(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) " - adeci's dev blog" }
                (terminal_styles())
            }
            body class="terminal" {
                div class="container" {
                    div class="main-content" {
                        (ascii_header())
                        (content)
                    }
                    (footer())
                }
            }
        }
    }
}
