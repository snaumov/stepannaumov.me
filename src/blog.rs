use anyhow::{anyhow, bail, Error, Result};
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};
use pulldown_cmark::{
    html::{self, push_html},
    Parser,
};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use tera::Context;
use walkdir::{DirEntry, WalkDir};

use crate::{AppError, AppState};

const POSTS_DIR: &str = "./posts";

#[derive(Serialize)]
struct BlogPost {
    title: String,
    content: String,
    slug: String,
    date: String,
}

#[derive(Deserialize)]
struct FrontMatter {
    title: String,
    date: String,
    slug: String,
}

impl TryFrom<String> for BlogPost {
    type Error = Error;
    fn try_from(value: String) -> Result<BlogPost> {
        let parts: Vec<&str> = value.splitn(3, "---").collect();
        if parts.len() < 3 {
            bail!("Invalid input: unable to find front matter and content");
        }
        let yaml_str = parts[1];
        let content = parts[2];
        // Parse YAML
        let front_matter: FrontMatter = serde_yaml::from_str(yaml_str)?;
        let parser = Parser::new(&content);
        let mut html_output = String::new();
        push_html(&mut html_output, parser);

        Ok(BlogPost {
            title: front_matter.title,
            content: html_output,
            slug: front_matter.slug,
            date: front_matter.date,
        })
    }
}

async fn get_posts(state: State<AppState>) -> Result<impl IntoResponse, AppError> {
    let posts: Vec<BlogPost> = WalkDir::new(POSTS_DIR)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
        .map(|e| {
            let input = read_to_string(e.path()).unwrap_or("".to_string());
            let blogpost: Result<BlogPost> = input.try_into();
            blogpost
        })
        .filter_map(Result::ok)
        .collect();

    let engine = state.get_engine();
    let mut context = Context::new();
    context.insert("title", "Posts");
    context.insert("posts", &posts);
    let rendered = engine.render("posts.html", &context)?;
    Ok(Html(rendered))
}

async fn get_post_by_slug(
    Path(slug): Path<String>,
    state: State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let post_path = format!("{}/{}.md", POSTS_DIR, slug);
    let input = read_to_string(post_path);
    if !input.is_ok() {
        return Ok((StatusCode::NOT_FOUND, Html("Post not found".to_string())));
    }
    let post: BlogPost = input?.try_into()?;
    let engine = state.get_engine();
    let mut context = Context::new();
    context.insert("post", &post);
    let rendered = engine.render("post.html", &context)?;
    Ok((StatusCode::OK, Html(rendered)))
}

pub fn routes<S>(state: AppState) -> Router<S> {
    Router::new()
        // Views
        .route("/post/:slug", get(get_post_by_slug))
        // Fragment Views
        .route("/get-posts", get(get_posts))
        .with_state(state)
}
