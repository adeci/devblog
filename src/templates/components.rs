use figrs::{Figlet, FigletOptions};
use maud::{Markup, html};

pub fn ascii_header() -> Markup {
    let options = FigletOptions {
        font: "Georgia11".to_string(),
        ..FigletOptions::default()
    };

    let figure = match Figlet::text("adeci".to_string(), options) {
        Ok(figlet) => figlet.text,
        Err(_) => "adeci".to_string(),
    };

    html! {
        div class="header-container" {
            div class="header-left" {
                a href="/" class="ascii-link" {
                    pre class="ascii-art" {
                        (maud::PreEscaped(figure))
                    }
                }
                div class="subtitle" { "technical blog" }
            }
            div class="header-right" {
                a href="https://github.com/adeci" class="social-link" { "github" }
                a href="https://linkedin.com/in/alexdecious" class="social-link" { "linkedin" }
            }
        }
    }
}

pub fn footer() -> Markup {
    html! {
        div class="footer" {
            div class="footer-links" {
                a href="https://github.com/adeci/devblog" class="source-link" { "view source" }
            }
            div class="footer-text" {
                span { "powered by " }
                a href="https://nixos.org" class="source-link" { "Nix" }
                span { " and a computer in my closet as the lord intended" }
            }
            div class="footer-text" {
                span { "The content for this site is " }
                a href="https://creativecommons.org/licenses/by-sa/4.0/" class="source-link" { "CC-BY-SA" }
            }
        }
    }
}
