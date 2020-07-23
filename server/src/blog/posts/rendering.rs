use super::model::{Album, Comment, Post, Todo, User};

pub fn create_post_html(p: &Post) -> maud::Markup {
    maud::html! {
        div style="border: 1px solid black;" {
            b { "Post" } br {}
            a href=(format!("/blog/user/{}", p.user_id)) {
                "user:" (p.user_id)
            } br {}
            "id:" (p.id) br {}
            "title:" (p.title) br {}
            "body:" (p.body)
        }
    }
}

pub fn create_posts_html(posts: &[Post]) -> maud::Markup {
    maud::html! {
        div {
            @for post in posts.iter() {
                (create_post_html(post))
            }
        }
    }
}

pub fn create_comment_html(comment: &Comment) -> maud::Markup {
    maud::html! {
        div style="border: 1px solid black;" {
            b { "Comment" } br {}
            "id: " (comment.id) br {}
            "name: " (comment.name) br {}
            "email: " (comment.email) br {}
            "body: " (comment.body)
        }
    }
}

pub fn create_comments_html(comments: &[Comment]) -> maud::Markup {
    maud::html! {
        div {
            @for comment in comments.iter() {
                (create_comment_html(comment))
            }
        }
    }
}

pub fn create_user_html(user: &User) -> maud::Markup {
    let address_str = format!(
        "{}, {}, {}, {} ({}, {})",
        user.address.street,
        user.address.suite,
        user.address.city,
        user.address.zipcode,
        user.address.geo.lat,
        user.address.geo.lng
    );
    maud::html! {
        div style="border: 1px solid black;" {
            b { "User" } br {}
            "id: " (user.id) br {}
            "name: " (user.name) br {}
            "email: " (user.email) br {}
            "address: " (address_str) br {}
            "phone: " (user.phone) br {}
            "website: " (user.website) br {}
            "company: " (user.company.name)
        }
    }
}

pub fn create_album_html(album: &Album) -> maud::Markup {
    maud::html! {
        div style="border: 1px solid black;" {
            b { "Album" } br {}
            "user id:" (album.user_id) br {}
            "id: " (album.id) br {}
            "title: " (album.title)
        }
    }
}

pub fn create_albums_html(albums: &[Album]) -> maud::Markup {
    maud::html! {
        div {
            @for album in albums.iter() {
                (create_album_html(album))
            }
        }
    }
}

pub fn create_todo_html(todo: &Todo) -> maud::Markup {
    maud::html! {
        div style="border: 1px solid black;" {
            b { "Todo" } br {}
            "user id: " (todo.user_id) br {}
            "id: " (todo.id) br {}
            "title: " (todo.title) br {}
            "completed: " (todo.completed)
        }
    }
}

pub fn create_todos_html(todos: &[Todo]) -> maud::Markup {
    maud::html! {
        div {
            @for todo in todos.iter() {
                (create_todo_html(todo))
            }
        }
    }
}
