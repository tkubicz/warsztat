pub fn posts_url() -> String {
    "https://jsonplaceholder.typicode.com/posts".into()
}

pub fn post_url(post_id: i32) -> String {
    format!("https://jsonplaceholder.typicode.com/posts/{}", post_id)
}

pub fn comments_url(post_id: i32) -> String {
    format!(
        "https://jsonplaceholder.typicode.com/posts/{}/comments",
        post_id
    )
}

pub fn user_url(user_id: i32) -> String {
    format!("https://jsonplaceholder.typicode.com/users/{}", user_id)
}

pub fn todos_url(user_id: i32) -> String {
    format!(
        "https://jsonplaceholder.typicode.com/users/{}/todos",
        user_id
    )
}

pub fn albums_url(user_id: i32) -> String {
    format!(
        "https://jsonplaceholder.typicode.com/users/{}/albums",
        user_id
    )
}
