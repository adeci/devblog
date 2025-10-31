use super::layout::page;
use crate::posts::Post;
use maud::{Markup, html};

pub fn home_page(posts: &[Post]) -> Markup {
    page(
        "Home",
        html! {
            div class="prompt" {
                span class="prompt-symbol" { "$ " }
                span { "ls posts/" }
            }

            div class="posts-list" {
                @if posts.is_empty() {
                    p { "No posts found." }
                } @else {
                    @for post in posts {
                        a href=(format!("/post/{}", post.slug)) class="post-entry-link" {
                            div class="post-entry" {
                                span class="date" { "[" (post.date) "]" }
                                " "
                                span class="post-title" {
                                    (post.title)
                                }
                                @if !post.tags.is_empty() {
                                    " "
                                    span class="tags" {
                                        @for tag in &post.tags {
                                            span class="tag" { "#" (tag) }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div class="bottom-prompt" {
                span class="prompt-symbol" { "$ " }
                span class="cursor" { }
            }
        },
    )
}

pub fn post_page(post: &Post) -> Markup {
    page(
        &post.title,
        html! {
            div class="prompt" {
                span class="prompt-symbol" { "$ " }
                span { "cat posts/" (post.slug) ".md" }
            }

            article class="post-content" {
                header {
                    h1 { (post.title) }
                    div class="post-meta" {
                        span class="date" { (post.date) }
                        @if !post.tags.is_empty() {
                            " "
                            span class="tags" {
                                @for tag in &post.tags {
                                    span class="tag" { "#" (tag) }
                                }
                            }
                        }
                    }
                }

                div class="content" {
                    (maud::PreEscaped(post.content.clone()))
                }
            }

            div class="bottom-prompt" {
                span class="prompt-symbol" { "$ " }
                span class="cursor" { }
            }
            div class="navigation" {
                a href="/" { "← Back to posts" }
            }
        },
    )
}
