use crate::blog::posts::{handler_show_post, handler_show_posts, handler_show_user};
use serde::{Deserialize, Serialize};
use warp::{Filter, Reply};

use crate::utils::{
    getFromUrl::get_from_url,
    render::{responseHtml, HandlerResponse},
};

pub fn blogRouting() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    let mainSwitch = (warp::path::end().and_then(show_main))
        .or(warp::path!("todos").and_then(show_todos))
        .or(warp::path!("todos" / u64).and_then(show_todos_by_id))
        .or(warp::path!("posts").and_then(handler_show_posts))
        .or(warp::path!("post" / i32).and_then(handler_show_post))
        .or(warp::path!("user" / i32).and_then(handler_show_user));

    mainSwitch
}

async fn show_main() -> HandlerResponse {
    Ok(responseHtml(200, "Główny widok".into()))
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoItemList {
    id: u64,
    userId: u64,
    title: String,
    //completed: bool,
}

async fn show_todos() -> HandlerResponse {
    let response = get_from_url("https://jsonplaceholder.typicode.com/todos").await;

    let response = match response {
        Ok(response) => response,
        Err(err) => {
            return Ok(responseHtml(
                500,
                format!("Błąd czytania z urla {}", err.toString()),
            ));
        }
    };

    let afterDecode = serde_json::from_str::<Vec<TodoItemList>>(&response);

    let todoList = match afterDecode {
        Ok(afterDecode) => afterDecode,
        Err(err) => {
            return Ok(responseHtml(500, format!("Błąd dekodowania {}", err)));
        }
    };

    let mut todosListHtml: Vec<maud::Markup> = Vec::new();

    for item in todoList {
        todosListHtml.push(maud::html! {
            div style="border: 1px solid black; margin: 10px; padding: 5px;" {
                a href=(format!("/blog/todos/{}", item.id)) {
                    (item.title)
                }
                br {}
                a href=(format!("/blog/users/{}", item.userId)) {
                    "User:" (item.userId)
                }
            }
        });
    }

    let htmlOut = maud::html! {
        html {
            body {
                div {
                    @for item in todosListHtml.iter() {
                        (item)
                    }
                }
            }
        }
    };

    return Ok(responseHtml(200, htmlOut.into_string()));
}

async fn show_todos_by_id(id: u64) -> HandlerResponse {
    Ok(responseHtml(200, format!("Widok todosa {}", id)))
}
