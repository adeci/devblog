use include_dir::{Dir, include_dir};
use serde::{Deserialize, Serialize};
use std::path::Path;

static POSTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/posts");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub content: String,
}

#[derive(Debug, Deserialize)]
struct PostFrontmatter {
    title: String,
    date: String,
    tags: Option<Vec<String>>,
}

pub fn load_posts() -> Vec<Post> {
    let mut posts = vec![];

    for file in POSTS_DIR.files() {
        if let Some(extension) = file.path().extension()
            && extension == "md"
            && let Ok(post) =
                load_single_post_from_content(file.contents_utf8().unwrap(), file.path())
        {
            posts.push(post);
        }
    }

    posts.sort_by(|a, b| b.date.cmp(&a.date));
    posts
}

fn load_single_post_from_content(
    content: &str,
    path: &Path,
) -> Result<Post, Box<dyn std::error::Error>> {
    let slug = path.file_stem().unwrap().to_string_lossy().to_string();

    if content.starts_with("---\n") {
        let parts: Vec<&str> = content.splitn(3, "---\n").collect();
        if parts.len() >= 3 {
            let frontmatter: PostFrontmatter = serde_yaml::from_str(parts[1])?;
            let markdown_content = parts[2];

            let mut options = pulldown_cmark::Options::empty();
            options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
            let parser = pulldown_cmark::Parser::new_ext(markdown_content, options);

            let mut html_content = String::new();
            pulldown_cmark::html::push_html(&mut html_content, parser);

            return Ok(Post {
                slug,
                title: frontmatter.title,
                date: frontmatter.date,
                tags: frontmatter.tags.unwrap_or_default(),
                content: html_content,
            });
        }
    }

    Err("Invalid post format".into())
}

pub fn get_post_by_slug(slug: &str) -> Option<Post> {
    load_posts().into_iter().find(|post| post.slug == slug)
}
