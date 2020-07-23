use super::model::{Album, Comment, Post, Todo, User};
use super::rendering::*;
use super::urls::*;
use futures::join;

use crate::utils::{
    getFromUrl::get_from_url,
    render::{responseHtml, HandlerResponse},
};

type HandlerResult = Result<String, Box<dyn std::error::Error>>;

pub async fn handler_show_posts() -> HandlerResponse {
    match show_posts().await {
        Ok(resp) => Ok(responseHtml(200, resp)),
        Err(e) => Ok(responseHtml(500, format!("Internal error: {:?}", e))),
    }
}

async fn show_posts() -> HandlerResult {
    let posts = get_from_url(&posts_url()).await?;
    let posts: Vec<Post> = serde_json::from_str(&posts)?;
    let result_html = maud::html! {
        html {
            body {
                (create_posts_html(&posts))
            }
        }
    };
    Ok(result_html.into_string())
}

pub async fn handler_show_post(id: i32) -> HandlerResponse {
    match show_post(id).await {
        Ok(resp) => Ok(responseHtml(200, resp)),
        Err(e) => Ok(responseHtml(500, format!("Internal error: {:?}", e))),
    }
}

async fn show_post(id: i32) -> HandlerResult {
    let post_url = post_url(id);
    let comments_url = comments_url(id);

    let post_fut = get_from_url(&post_url);
    let comments_fut = get_from_url(&comments_url);

    let (post, comments) = join!(post_fut, comments_fut);

    let post = post?;
    let comments = comments?;

    let post: Post = serde_json::from_str(&post)?;
    let comments: Vec<Comment> = serde_json::from_str(&comments)?;

    let user = get_from_url(&user_url(post.user_id)).await?;
    let user = serde_json::from_str(&user)?;

    let result_html = maud::html! {
        html {
            body {
                (create_post_html(&post)) br {}
                (create_user_html(&user)) br {}
                (create_comments_html(&comments))
            }
        }
    };

    Ok(result_html.into_string())
}

pub async fn handler_show_user(user_id: i32) -> HandlerResponse {
    match show_user(user_id).await {
        Ok(resp) => Ok(responseHtml(200, resp)),
        Err(e) => Ok(responseHtml(500, format!("Internal error: {:?}", e))),
    }
}

async fn show_user(user_id: i32) -> HandlerResult {
    let user_url = user_url(user_id);
    let albums_url = albums_url(user_id);
    let todos_url = todos_url(user_id);

    let user_fut = get_from_url(&user_url);
    let albums_fut = get_from_url(&albums_url);
    let todos_fut = get_from_url(&todos_url);

    let (user, albums, todos) = join!(user_fut, albums_fut, todos_fut);

    let user = user?;
    let albums = albums?;
    let todos = todos?;

    let user: User = serde_json::from_str(&user)?;
    let albums: Vec<Album> = serde_json::from_str(&albums)?;
    let todos: Vec<Todo> = serde_json::from_str(&todos)?;

    let result_html = maud::html! {
        html {
            body {
                (create_user_html(&user)) br {}
                (create_albums_html(&albums)) br {}
                (create_todos_html(&todos))
            }
        }
    };

    Ok(result_html.into_string())
}
